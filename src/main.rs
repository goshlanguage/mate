use std::{
    env, thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use std::collections::HashMap;

use tda_sdk::{
    params::GetAccountsParams,
    params::GetPriceHistoryParams,
    responses::Candle,
    responses::{GetPriceHistoryResponse, SecuritiesAccount},
    Client,
};

mod ta;

// mate makes use of the tda-sdk crate for access to a brokerage API
// https://github.com/rideron89/tda-sdk-rs
pub struct Mate {
    client: Client,
    candles: HashMap<String, Vec<Candle>>,
    last_candles: HashMap<String, Candle>,
    symbols: Vec<String>,
}

impl Mate {
    pub fn default() -> Self {
        let (client_id, refresh_token) = get_creds();
        let mut client = Client::new(&client_id, &refresh_token, None);

        let response = client.get_access_token().unwrap();
        client.set_access_token(&Some(response.into()));

        Mate {
            client,
            candles: HashMap::new(),
            last_candles: HashMap::new(),
            symbols: vec![],
        }
    }

    // status reflects a TD Ameritrade account balance and other information.
    pub fn status(&self) -> String {
        let mut status = String::new();

        let accounts = self
            .client
            .get_accounts(GetAccountsParams::default())
            .unwrap();

        if accounts.len() > 1 {
            panic!("Encountered more than 1 TD Ameritrade account. Sorry, this isn't currently supported properly.")
        }

        for account in accounts {
            match account.securities_account {
                SecuritiesAccount::MarginAccount {
                    current_balances,
                    projected_balances,
                    ..
                } => {
                    status = format!(
                        "{}Account Balance: ${}\n",
                        status, current_balances.cash_balance
                    );
                    status = format!(
                        "{}Available Balance: ${}\n",
                        status, projected_balances.cash_available_for_trading
                    );
                }
            }
        }

        println!("Watching: {:?}", self.symbols);

        status
    }

    fn refresh_candles(&mut self) {
        for symbol in self.symbols.to_vec() {
            // https://developer.tdameritrade.com/price-history/apis/get/marketdata/%7Bsymbol%7D/pricehistory
            let params = GetPriceHistoryParams {
                end_date: None,
                frequency_type: Some(String::from("daily")),
                frequency: Some(String::from("1")),
                need_extended_hours_data: None,
                period_type: Some(String::from("year")),
                period: Some(String::from("1")),
                start_date: None,
            };

            let history = self.client.get_price_history(&symbol, params);

            let resp = match history {
                Ok(val) => val,
                Err(e) => {
                    println!("Failed to get price history: {}", e.to_string());
                    return;
                }
            };
            self.candles.insert(symbol.clone(), resp.candles);

            // last_candles provides a last period's candle to use the close price in calculations
            // this is used in calculations like SMA and EMA
            let now = SystemTime::now();
            let mut epoch = now.duration_since(UNIX_EPOCH).unwrap().as_millis();

            let last_session_params = GetPriceHistoryParams {
                end_date: Some(epoch.to_string()),
                frequency_type: None,
                frequency: None,
                need_extended_hours_data: Some(false),
                period_type: None,
                period: None,
                start_date: Some(epoch.to_string()),
            };

            let last_session_history = self.client.get_price_history(&symbol, last_session_params);

            let last_resp = match last_session_history {
                Ok(val) => val,
                Err(e) => {
                    println!("Failed to get price history: {}", e.to_string());

                    let day_in_ms = 24 * 60 * Duration::new(60, 0).as_millis();
                    epoch -= day_in_ms;
                    self.get_price_history_by_epoch(symbol.clone(), epoch)
                }
            };

            let mut last_candles = last_resp.candles;
            self.last_candles
                .insert(symbol.clone(), last_candles.pop().unwrap());
        }
    }

    fn get_price_history_by_epoch(
        &self,
        symbol: String,
        mut epoch: u128,
    ) -> GetPriceHistoryResponse {
        let last_session_params = GetPriceHistoryParams {
            end_date: Some(epoch.to_string()),
            frequency_type: None,
            frequency: None,
            need_extended_hours_data: Some(false),
            period_type: None,
            period: None,
            start_date: Some(epoch.to_string()),
        };

        let last_session_history = self.client.get_price_history(&symbol, last_session_params);

        let last_resp = match last_session_history {
            Ok(val) => val,
            Err(e) => {
                println!("Failed to get price history: {}", e.to_string());

                let hour_in_ms = 60 * Duration::new(60, 0).as_millis();
                epoch -= hour_in_ms;
                self.get_price_history_by_epoch(symbol, epoch)
            }
        };

        last_resp
    }

    fn get_latest_candles(&self, symbol: String) -> Vec<Candle> {
        let last_candle_ref = self.last_candles.get(&symbol).unwrap().to_owned();
        let mut candles = self.candles.get(&symbol).unwrap().clone();

        candles.push(last_candle_ref);
        (*candles).to_vec()
    }
}

// get_creds looks for `TDA_CLIENT_ID` and `TDA_REFRESH_TOKEN` environment variables and panics if they aren't present.
fn get_creds() -> (String, String) {
    let client_id = match env::var("TDA_CLIENT_ID") {
        Ok(val) => val,
        Err(e) => panic!(
            "Didn't find the TDA_CLIENT_ID env var, please set this and try again. {}",
            e
        ),
    };

    let refresh_token = match env::var("TDA_REFRESH_TOKEN") {
        Ok(val) => val,
        Err(e) => panic!(
            "Didn't find the TDA_REFRESH_TOKEN env var, please set this and try again. {}",
            e
        ),
    };

    (client_id, refresh_token)
}

fn main() {
    let mut mate = Mate::default();

    mate.symbols = vec!["MSFT".to_string()];

    loop {
        mate.refresh_candles();
        println!("{}", mate.status());

        let candlevec = mate.get_latest_candles("MSFT".to_string());
        let msft_candles = candlevec.as_slice();

        let sma20 = ta::sma(msft_candles, 0, 20);
        let sma50 = ta::sma(msft_candles, 0, 50);
        let sma100 = ta::sma(msft_candles, 0, 100);

        println!("SMA20: {}\tSMA50: {}\tSMA100: {}", sma20, sma50, sma100);

        let ema20 = ta::ema(msft_candles, 20);
        let ema50 = ta::ema(msft_candles, 50);

        // TODO Check for NaN values to ensure we don't submit a faulty order
        println!("EMA20: {}\tEMA50: {}", ema20, ema50);
        if ema20 > 0.0 && ema50 > 0.0 {
            if ema20 > ema50 {
                println!("buy");
                println!("set stop loss at 95%");
            } else {
                println!("sell");
            }
        }

        println!();

        // sleep for an hour, as not to miss any trading window
        let hour = Duration::from_secs(60 * 60);
        // let day = Duration::from_secs(60 * 60 * 24);
        thread::sleep(hour);
    }
}
