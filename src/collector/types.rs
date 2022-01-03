use chrono::format::strftime;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use serde_json::{from_value, json, Map, Value};
use std::{fs::create_dir, path::Path};

extern crate chrono;
use chrono::prelude::*;

/// TODO
///   Fix this broken module situation
#[path = "./state/file.rs"]
mod state;
use state::{read_file, write_file};

#[path = "../account/mod.rs"]
mod account;
use account::kraken::{get_kraken_creds, KrakenAccount};
use account::tdameritrade::{get_tdameritrade_creds, TDAmeritradeAccount};
use account::types::AccountType;

pub struct Collector {
    pub conf: CollectorConfig,
    pub accounts: Vec<AccountType>,
}

pub struct CollectorConfig {
    pub accounts: Vec<String>,
    pub crypto_watchlist: Vec<String>,
    pub stock_watchlist: Vec<String>,
    pub filepath: String,
}

impl Collector {
    pub fn new(conf: CollectorConfig) -> Collector {
        let mut collector = Collector {
            accounts: Vec::new(),
            conf,
        };

        ensure_dir_exists(format!("{}", collector.conf.filepath).as_str());
        ensure_dir_exists(format!("{}/equity/", collector.conf.filepath).as_str());
        ensure_dir_exists(format!("{}/crypto/", collector.conf.filepath).as_str());
        ensure_dir_exists(format!("{}/crypto/tick", collector.conf.filepath).as_str());

        for account in &collector.conf.accounts {
            match account.to_lowercase().as_str() {
                "tdameritrade" => {
                    info!("Setting up TDAmeritrade");
                    let (client_id, refresh_token) = get_tdameritrade_creds();
                    let td_account = TDAmeritradeAccount::new(
                        "TDAmeritrade",
                        "My account",
                        client_id.as_str(),
                        refresh_token.as_str(),
                    );

                    collector
                        .accounts
                        .push(AccountType::TDAmeritradeAccount(td_account));
                }
                "kraken" => {
                    info!("Setting up Kraken");
                    let (client_key, client_secret) = get_kraken_creds();
                    let kraken_account = KrakenAccount::new(
                        "Kraken",
                        "My kraken account",
                        client_key.as_str(),
                        client_secret.as_str(),
                    );

                    collector
                        .accounts
                        .push(AccountType::KrakenAccount(kraken_account));
                }
                _ => warn!("Unrecognized account: {}", account),
            };
        }

        collector
    }

    pub fn update(&self) {
        // let datapath = Path::new(filepath.as_str());
        // if datapath.exists() {
        //     data = read_file(format!("{}/data.txt", self.config.filepath).as_str());
        //     info!("Read state file from last iteration")
        // }

        for account in &self.accounts {
            match account {
                AccountType::TDAmeritradeAccount(account) => {
                    info!("Collecting stock data");

                    for symbol in &self.conf.stock_watchlist {
                        let mut data: Map<String, Value> = Map::new();
                        let candles = account.get_candles(symbol.to_string());

                        let mut new_mate_candles: Vec<MateCandle> = Vec::new();
                        for candle in candles {
                            let new_mc = MateCandle {
                                close: candle.close,
                                datetime: candle.datetime,
                                high: candle.high,
                                low: candle.low,
                                open: candle.open,
                                volume: candle.volume,
                            };
                            new_mate_candles.push(new_mc);
                        }
                        info!("Fetched data for {}", symbol);
                        data.insert(symbol.clone(), json!(new_mate_candles));

                        let filepath = format!("{}/equity/{}.json", self.conf.filepath, symbol);
                        write_file(filepath.as_str(), &data);
                    }
                }
                AccountType::KrakenAccount(account) => {
                    info!("Collecting crypto pairs");

                    let pairs = self.conf.crypto_watchlist.to_vec();
                    let ticks = account.get_ticks(pairs);

                    info!("Fetched data for {}", self.conf.crypto_watchlist.join(", "));

                    let keys = ticks.keys().cloned();
                    for pair in keys {
                        let mut data: Map<String, Value> = Map::new();

                        ensure_dir_exists(
                            format!("{}/crypto/tick/{}", self.conf.filepath, pair).as_str(),
                        );

                        let filepath = format!(
                            "{}/crypto/tick/{}/{}.json",
                            self.conf.filepath,
                            pair,
                            get_year_month_day()
                        );

                        let filepath_exists = Path::new(filepath.as_str()).exists();
                        if filepath_exists {
                            data = read_file(filepath.as_str());
                            info!("Read state file from {}", filepath);
                        };

                        data.insert(
                            get_seconds_from_midnight(),
                            ticks.get(&pair).unwrap().clone(),
                        );

                        write_file(&filepath, &data);
                    }
                }
            }
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

/// PeriodInterval defines the overall period to request for a stock or crypto pair
#[allow(dead_code)]
enum PeriodInterval {
    Day,
    Month,
    Year,
    YTD,
}

/// FrequencyInterval defines the frequency that each candle represents
#[allow(dead_code)]
enum FrequencyInterval {
    Minute,
    Daily,
    Weekly,
    Monthly,
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

fn get_seconds_from_midnight() -> String {
    let dt = chrono::Utc::now();
    format!("{}", dt.num_seconds_from_midnight())
}
