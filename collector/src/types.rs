use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json::{from_value, json, Map, Value};
use std::{fs::create_dir, path::Path};
use tda_sdk::responses::Candle;

use accounts::kraken::KrakenAccount;
use accounts::tdameritrade::TDAmeritradeAccount;
use accounts::types::AccountType;

use chrono;

/// TODO
///   Fix this broken module situation
#[path = "./state/file.rs"]
mod state;
use state::*;

pub struct Collector {
    pub conf: CollectorConfig,
    pub accounts: Vec<AccountType>,
}

pub struct CollectorConfig {
    pub accounts: Vec<String>,
    pub crypto_watchlist: Vec<String>,
    pub filepath: String,
    pub poll_seconds: u64,
    pub stock_watchlist: Vec<String>,
}

impl Collector {
    /// new returns a collector configured with enabled accounts and settings from a given CollectorConfig
    /// it also ensures that the filesystem tree we need exists, and that each account is valid
    pub fn new(conf: CollectorConfig) -> Collector {
        let mut collector = Collector {
            accounts: Vec::new(),
            conf,
        };

        ensure_filesystem_tree_exists(collector.conf.filepath.as_str());

        for account in &collector.conf.accounts {
            let new_account = accounts::new_account(account, account, "", "", "").unwrap();
            collector.accounts.push(new_account);
        }

        collector
    }

    /// update iterates through the collector accounts, performing the update necessary for the given account types, collecting equity information when given
    /// an equity account, and getting tick data for crypto accounts
    pub fn update(&self) {
        for account in &self.accounts {
            match account {
                AccountType::TDAmeritradeAccount(account) => {
                    info!("Collecting stock data");
                    self.poll_tdameritrade(account);
                }
                AccountType::KrakenAccount(account) => {
                    info!("Collecting crypto pairs");
                    self.poll_kraken(account);
                }
            }
        }
    }

    fn poll_tdameritrade(&self, account: &TDAmeritradeAccount) {
        for symbol in &self.conf.stock_watchlist {
            let mut data: Value = Value::String("".to_string());

            let filepath = format!("{}/equity/daily/{}.json", self.conf.filepath, symbol);

            let filepath_exists = Path::new(filepath.as_str()).exists();
            if filepath_exists {
                data = read_file(filepath.as_str());
                info!("Read state file from {}", filepath);
            };

            let mut candles: Vec<MateCandle> = from_value(data).unwrap();
            match candles.len() {
                0 => {
                    let ticker = symbol.to_string();
                    data = self.get_td_candles(account, ticker);
                }
                n => {
                    info!("Found existing candles, checking for period within current window");
                    let last_8_hours_s: usize = 60 * 60 * 8;
                    let last_date = candles[n - 1].datetime;
                    if last_date < get_epoch() - last_8_hours_s {
                        info!("Data older than 8 hours, collecting new daily candle");
                        let last_day = account.get_daily_candle(symbol.to_string());
                        match last_day {
                            Ok(c) => {
                                let mc = MateCandle::from_candle(c);
                                candles.push(mc);
                                data = json!(candles);
                            }
                            Err(_e) => continue,
                        }
                    } else {
                        info!("Already have most recent data for {}, continuing", symbol);
                        continue;
                    }
                }
            }

            write_file(filepath.as_str(), data);
        }
    }

    fn get_td_candles(&self, account: &TDAmeritradeAccount, symbol: String) -> Value {
        let candles = account.get_candles(symbol.clone());
        let mut new_mate_candles: Vec<MateCandle> = Vec::new();

        // TODO
        // remove this hacky workaround when upstream supports Serialize
        for candle in candles {
            let new_mc = MateCandle::from_candle(candle);
            new_mate_candles.push(new_mc);
        }
        info!("Fetched data for {}", symbol);
        json!(new_mate_candles)
    }

    fn poll_kraken(&self, account: &KrakenAccount) {
        let pairs = self.conf.crypto_watchlist.to_vec();

        let tick_results = account.get_ticks(pairs);
        let ticks = match tick_results {
            Ok(t) => t,
            Err(e) => {
                error!("{}", e);
                return;
            }
        };

        info!("Fetched data for {}", self.conf.crypto_watchlist.join(", "));

        let keys = ticks.keys().cloned();
        for pair in keys {
            let mut data: Map<String, Value> = Map::new();

            ensure_dir_exists(format!("{}/crypto/tick/{}", self.conf.filepath, pair).as_str());

            let filepath = format!(
                "{}/crypto/tick/{}/{}.json",
                self.conf.filepath,
                pair,
                get_year_month_day()
            );

            let filepath_exists = Path::new(filepath.as_str()).exists();
            if filepath_exists {
                data = read_map_from_file(filepath.as_str());
                info!("Read state file from {}", filepath);
            };

            data.insert(get_epoch().to_string(), ticks.get(&pair).unwrap().clone());

            write_map_to_file(&filepath, &data);
        }
    }
}

/// TODO:
///   Add serialize upstream
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct MateCandle {
    pub close: f64,
    pub datetime: usize,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub volume: i64,
}

impl MateCandle {
    fn from_candle(candle: Candle) -> MateCandle {
        MateCandle {
            close: candle.close,
            datetime: candle.datetime,
            high: candle.high,
            low: candle.low,
            open: candle.open,
            volume: candle.volume,
        }
    }
}

/// PeriodInterval defines the overall period to request for a stock or crypto pair
#[allow(dead_code)]
enum PeriodInterval {
    Day,
    Month,
    Year,
    Ytd,
}

/// FrequencyInterval defines the frequency that each candle represents
#[allow(dead_code)]
enum FrequencyInterval {
    Minute,
    Daily,
    Weekly,
    Monthly,
}

/// Scaffolds out our collector filesystem structure if needed
fn ensure_filesystem_tree_exists(filepath: &str) {
    ensure_dir_exists(filepath);
    ensure_dir_exists(format!("{}/equity/", filepath).as_str());
    ensure_dir_exists(format!("{}/equity/daily", filepath).as_str());
    ensure_dir_exists(format!("{}/crypto/", filepath).as_str());
    ensure_dir_exists(format!("{}/crypto/tick", filepath).as_str());
}

fn ensure_dir_exists(filepath: &str) {
    let path_exists = Path::new(filepath).exists();
    if !path_exists {
        create_dir(filepath).unwrap();
    }
}

fn get_year_month_day() -> String {
    let current_date = chrono::Utc::now();
    current_date.format("%Y%m%d").to_string()
}

fn get_epoch() -> usize {
    chrono::Utc::now().timestamp() as usize
}
