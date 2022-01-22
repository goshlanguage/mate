use log::{error, info};
use crate::magic_crypt::MagicCryptTrait;
extern crate num_traits;
use num_traits::cast::ToPrimitive;
use serde::{Deserialize, Serialize};
use serde_json::{from_value, json, Map, Value};
use std::{env, fs::create_dir, path::Path};
use tda_sdk::responses::{Candle, SecuritiesAccount};

use accounts::kraken::KrakenAccount;
use accounts::tdameritrade::TDAmeritradeAccount;
use accounts::types::AccountType;

/// TODO
///   Fix this broken module situation
#[path = "./state/mod.rs"]
mod state;
use state::{api::*, file::*, s3::S3};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub vendor: String,
    pub client_key: String,
    pub client_secret: String,
    pub created: chrono::NaiveDateTime,
    pub updated: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Accounts {
    pub accounts: Vec<Account>,
}

impl Accounts {
    pub fn new() -> Accounts {
        Accounts {
            accounts: Vec::new(),
        }
    }
}

pub struct Collector {
    pub accounts: Vec<AccountType>,
    pub bucket: S3,
    pub conf: CollectorConfig,
}

pub struct CollectorConfig {
    pub accounts: Vec<String>,
    pub api_host: Option<String>,
    pub crypto_watchlist: Vec<String>,
    pub filepath: Option<String>,
    pub poll_seconds: u64,
    pub s3_bucket: String,
    pub s3_proto: String,
    pub s3_region: String,
    pub stock_watchlist: Vec<String>,
}

impl Collector {
    /// new returns a collector configured with enabled accounts and settings from a given CollectorConfig
    /// it also ensures that the filesystem tree we need exists, and that each account is valid
    pub fn new(conf: CollectorConfig) -> Collector {
        let bucket = S3::default();

        let mut collector = Collector {
            accounts: Vec::new(),
            bucket,
            conf,
        };

        if !&collector.conf.s3_bucket.is_empty() {
            collector.bucket = S3::new(
                collector.conf.s3_bucket.to_owned(),
                collector.conf.s3_proto.to_owned(),
                collector.conf.s3_region.to_owned(),
            );
        }

        if !&collector.conf.filepath.is_none() {
            let filepath = &collector.conf.filepath.as_ref().unwrap();
            ensure_filesystem_tree_exists(filepath);
        }

        match &collector.conf.api_host {
            Some(host) => {
                info!("Configuring accounts from the API");

                let mut api_host = host.to_string();

                // ensure the host string doesn't end in / or normalize the string
                let last_char = &host.to_string().pop().unwrap();
                let slash = "/".chars().next().unwrap();
                if last_char == &slash {
                    api_host = host.to_string();
                    api_host.truncate(api_host.len() - 1);
                }

                collector.conf.api_host = Some(api_host.to_string());

                info!("API Host: {}", &api_host);

                let api_client = Client { api_host };
                let accounts: Accounts = match api_client.get_accounts() {
                    Ok(a) => a,
                    Err(e) => {
                        error!("Api endpoint unavailable");
                        error!("{}", e);
                        Accounts::new()
                    }
                };

                for account in &accounts.accounts {
                    info!(
                        "Setting up {} account {}",
                        &account.vendor.as_str(),
                        &account.name.as_str()
                    );

                    let decrypted_secret = decrypt(account.client_secret.to_string());

                    let new_account = accounts::new_account(
                        account.name.as_str(),
                        account.vendor.as_str(),
                        &account.id.to_string(),
                        Some(account.id),
                        account.client_key.as_str(),
                        decrypted_secret.as_str(),
                    )
                    .unwrap();
                    collector.accounts.push(new_account);
                }
            }
            None => {
                info!("Configuring accounts from the command line");
                for account in &collector.conf.accounts {
                    let new_account =
                        accounts::new_account(account, account, "", None, "", "").unwrap();
                    collector.accounts.push(new_account);
                }
            }
        }

        collector
    }

    /// update iterates through the collector accounts, performing the update necessary for the given account types, collecting equity information when given
    /// an equity account, and getting tick data for crypto accounts
    pub fn update(&self) {
        for account in &self.accounts {
            match account {
                AccountType::TDAmeritradeAccount(account) => {
                    info!("Collecting Kraken balance data");

                    let mut db_id = 0;
                    if !&account.database_id.is_none() {
                        db_id = account.database_id.unwrap();
                    }

                    let payload = self.poll_td_balance(account, db_id);

                    // if the API is enabled, post data
                    if self.conf.api_host.is_some() {
                        let api_client = Client {
                            api_host: get_api_host(self.conf.api_host.as_ref().unwrap().to_string()),
                        };
                        match api_client.submit_account_balances(payload){
                            Ok(_) => (),
                            Err(e) => {
                                error!("account balance to API failed: {}", e.to_string());
                            },
                        };
                    }

                    info!("Collecting stock data");
                    self.poll_tdameritrade(account);
                }
                AccountType::KrakenAccount(account) => {
                    info!("Collecting Kraken balance data");

                    // if API is enabled, post data
                    if self.conf.api_host.is_some() {
                        let mut db_id = 0;
                        if !&account.database_id.is_none() {
                            db_id = account.database_id.unwrap();
                        }

                        let payload = self.poll_kraken_balance(account, db_id);
                        let api_client = Client {
                            api_host: get_api_host(self.conf.api_host.as_ref().unwrap().to_string()),
                        };

                        match api_client.submit_account_balances(payload){
                            Ok(_) => (),
                            Err(e) => {
                                error!("account balance to API failed: {}", e.to_string());
                            },
                        };
                    }

                    info!("Collecting crypto pairs");
                    self.poll_kraken(account);
                }
            }
        }
    }

    fn poll_tdameritrade(&self, account: &TDAmeritradeAccount) {
        for symbol in &self.conf.stock_watchlist {
            let mut data: Value = Value::String("".to_string());

            // if file usage is set, read in state from file
            if !&self.conf.filepath.is_none() {
                let prefix = &self.conf.filepath.as_ref().unwrap();
                let filepath = format!("{}/equity/daily/{}.json", prefix, symbol);

                let filepath_exists = Path::new(filepath.as_str()).exists();
                if filepath_exists {
                    data = read_file(filepath.as_str());
                    info!("Read state file from {}", filepath);
                };
            }

            let mut candles: Vec<MateCandle> = from_value(data).unwrap_or_default();
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

            // if file usage is set, save our data state to disk
            if !&self.conf.filepath.is_none() {
                let prefix = self.conf.filepath.as_ref().unwrap();
                let filepath = format!("{}/equity/daily/{}.json", prefix, symbol);

                write_file(filepath.as_str(), data.to_owned());
            }

            if !self.conf.s3_bucket.is_empty() {
                info!("saving to bucket");

                let path = format!("/equity-daily-{}-{}.json", symbol, get_year_month_day(),);

                self.bucket.save(path, json!(data.to_owned()).to_string());
            }
        }
    }

    /// poll_td_balance takes in a mate_account_id because the id of the account in the database
    ///   will differ from the actual account id and this isn't better handled yet
    fn poll_td_balance(
        &self,
        account: &TDAmeritradeAccount,
        mate_account_id: i32,
    ) -> NewAccountBalancesPayload {
        let accounts = &account.get_accounts();

        let mut resp = NewAccountBalancesPayload {
            balances: Vec::new(),
        };

        for a in accounts {
            info!("Current Balances:");
            match &a.securities_account {
                #[allow(unused_variables)]
                SecuritiesAccount::MarginAccount {
                    r#type,
                    account_id,
                    round_trips,
                    is_day_trader,
                    is_closing_only_restricted,
                    initial_balances,
                    current_balances,
                    projected_balances,
                } => {
                    info!("account id: {}", account_id);
                    info!("round trips: {}", round_trips);
                    info!("is_day_trader: {}", is_day_trader);
                    info!("is_closing_only_restricted: {}", is_closing_only_restricted);

                    info!(
                        "initially available_funds: {}",
                        initial_balances.account_value
                    );
                    info!("initially cash balance: {}", initial_balances.cash_balance);

                    if current_balances.available_funds.is_some() {
                        info!(
                            "available_funds: {}",
                            current_balances.available_funds.unwrap()
                        );
                    }

                    if current_balances.buying_power.is_some() {
                        info!("buying_power: {}", current_balances.buying_power.unwrap());
                    }

                    if current_balances.cash_available_for_trading.is_some() {
                        info!(
                            "cash_available_for_trading: {}",
                            current_balances.cash_available_for_trading.unwrap()
                        );
                    }
                    info!("cash_balance: {}", current_balances.cash_balance);

                    if current_balances.equity.is_some() {
                        info!("equity: {}", current_balances.equity.unwrap());
                    }

                    info!("liquidation_value: {}", current_balances.liquidation_value);

                    if current_balances.margin_balance.is_some() {
                        info!(
                            "margin_balance: {}",
                            current_balances.margin_balance.unwrap()
                        );
                    }

                    if current_balances.total_cash.is_some() {
                        info!("total_cash: {}", current_balances.total_cash.unwrap());
                    }

                    if current_balances.unsettled_cash.is_some() {
                        info!(
                            "unsettled_cash: {}",
                            current_balances.unsettled_cash.unwrap()
                        );
                    }

                    if projected_balances.available_funds.is_some() {
                        info!(
                            "projected_funds: {}",
                            projected_balances.available_funds.unwrap()
                        );
                    }

                    resp.balances.push(NewAccountBalancePayload {
                        account_id: mate_account_id,
                        balance: current_balances.cash_balance,
                    });
                }
            }
        }

        resp
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

            data.insert(get_epoch().to_string(), ticks.get(&pair).unwrap().clone());

            if self.conf.filepath.is_some() {
                let prefix = self.conf.filepath.as_ref().unwrap();
                ensure_dir_exists(format!("{}/crypto/tick/{}", prefix, pair).as_str());

                let filepath = format!(
                    "{}/crypto/tick/{}/{}.json",
                    prefix,
                    pair,
                    get_year_month_day()
                );

                let filepath_exists = Path::new(filepath.as_str()).exists();
                if filepath_exists {
                    data = read_map_from_file(filepath.as_str());
                    info!("Read state file from {}", filepath);
                };
                write_map_to_file(&filepath, &data);
            }

            if !self.conf.s3_bucket.is_empty() {
                info!("saving to bucket");

                let path = format!("/crypto-tick-{}-{}.json", pair, get_year_month_day(),);

                self.bucket.save(path, json!(data.to_owned()).to_string());
            }
        }
    }

    fn poll_kraken_balance(
        &self,
        account: &KrakenAccount,
        mate_account_id: i32,
    ) -> NewAccountBalancesPayload {
        let balance = &account.get_account_balance();

        NewAccountBalancesPayload {
            balances: vec![NewAccountBalancePayload {
                account_id: mate_account_id,
                balance: balance.to_f64().unwrap(),
            }],
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

#[derive(Debug, Deserialize, Serialize)]
pub struct NewAccountBalancePayload {
    pub account_id: i32,
    pub balance: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewAccountBalancesPayload {
    pub balances: Vec<NewAccountBalancePayload>,
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

fn get_api_host(api_host: String) -> String {
    let mut host = api_host.to_string();

    // ensure the host string doesn't end in / or normalize the string
    let last_char = &api_host.to_string().pop().unwrap();
    let slash = "/".chars().next().unwrap();
    if last_char == &slash {
        host = api_host;
        host.truncate(host.len() - 1);
    }

    host
}

fn decrypt(input: String) -> String {
    let salt = match env::var("MATE_SALT") {
        Ok(val) => val,
        Err(e) => panic!(
            "Didn't find the MATE_SALT env var, please set this and try again. {}",
            e
        ),
    };

    let mc = new_magic_crypt!(salt, 256);
    mc.decrypt_base64_to_string(&input.as_str()).unwrap()
}
