use std::{
    env,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use std::collections::HashMap;

use tda_sdk::{
    Client,
    params::GetAccountsParams,
    params::GetPriceHistoryParams,
    responses::Candle,
    responses::SecuritiesAccount,
};

mod datafeed;
mod ta;

// mate makes use of the tda-sdk crate for access to a brokerage API
// https://github.com/rideron89/tda-sdk-rs
pub struct Mate {
    client: Client,
    candles: HashMap<String, Vec<Candle>>,
    symbols: Vec<String>
}

impl Mate {
    pub fn default() -> Self {
        let (client_id, refresh_token) = get_creds();
        let mut client = Client::new(&client_id, &refresh_token, None);

        let response = client.get_access_token().unwrap();
        client.set_access_token(&Some(response.into()));

        Mate{
            client,
            candles: HashMap::new(),
            symbols: vec![],
        }
    }

    // status reflects a TD Ameritrade account balance and other information.
    pub fn status(&self) -> String {
        let mut status = String::new();

        let accounts = self.client.get_accounts(GetAccountsParams::default()).unwrap();

        if accounts.len() > 1 {
            panic!("Encountered more than 1 TD Ameritrade account. Sorry, this isn't currently supported properly.")
        }

        for account in accounts {
            match account.securities_account {
                SecuritiesAccount::MarginAccount { current_balances, projected_balances, .. } => {
                    status = format!("{}Account Balance: ${}\n", status, current_balances.cash_balance);
                    status = format!("{}Available Balance: ${}\n", status, projected_balances.cash_available_for_trading);
                }
            }
        }

        println!("Watching: {:?}", self.symbols);

        status
    }

    fn refresh_candles(&mut self) {

        for symbol in self.symbols.to_vec() {
            // https://developer.tdameritrade.com/price-history/apis/get/marketdata/%7Bsymbol%7D/pricehistory
            let params = GetPriceHistoryParams{
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
                    return
                },
            };

            // TODO
            // this hacky work around ONLY works for the first iteration, then will fail on all subsequent iterations
            // without first popping the latest appended last candle off of self.candles.
            // Fix this when moved into datafeed
            let now = SystemTime::now();
            let epoch = now
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();

            let last_session_params = GetPriceHistoryParams{
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
                    return
                },
            };

            let mut candles = resp.candles;
            let mut last_candles = last_resp.candles;
            let last_candle = last_candles.pop().unwrap();
            candles.push(last_candle);
            self.candles.insert(symbol, candles);
        }


        // let start_ms = resp.candles[0].datetime as i64;
        // let end_ms = resp.candles[resp.candles.len()-1].datetime as i64;

        // let start_time = start_ms / 1000;
        // let end_time = end_ms / 1000;

        // let naive_start = NaiveDateTime::from_timestamp(start_time, 0);
        // let naive_end = NaiveDateTime::from_timestamp(end_time, 0);

        // let datetime_start: DateTime<Utc> = DateTime::from_utc(naive_start, Utc);
        // let datetime_end: DateTime<Utc> = DateTime::from_utc(naive_end, Utc);

        // let readable_start = datetime_start.to_rfc2822();
        // let readable_end = datetime_end.to_rfc2822();

        // println!("Gathered candle data for {} between {} and {}", resp.symbol, readable_start, readable_end);
        // println!("Candle data contains {} candles", resp.candles.len());
    }
}

// get_creds looks for `TDA_CLIENT_ID` and `TDA_REFRESH_TOKEN` environment variables and panics if they aren't present.
fn get_creds() -> (String, String) {
    let client_id = match env::var("TDA_CLIENT_ID") {
        Ok(val) => val,
        Err(e) => panic!("Didn't find the TDA_CLIENT_ID env var, please set this and try again. {}", e)
    };

    let refresh_token = match env::var("TDA_REFRESH_TOKEN") {
        Ok(val) => val,
        Err(e) => panic!("Didn't find the TDA_REFRESH_TOKEN env var, please set this and try again. {}", e),
    };

    (client_id, refresh_token)
}

fn main() {
    let mut mate = Mate::default();

    mate.symbols = vec!["MSFT".to_string()];

    loop {
        mate.refresh_candles();
        println!("{}", mate.status());

        let mut mr_soft_candles = mate.candles.get("MSFT").unwrap();

        let sma20 = ta::sma(mr_soft_candles, 0, 20);
        let sma50 = ta::sma(mr_soft_candles, 0, 50);
        let sma100 = ta::sma(mr_soft_candles, 0, 100);

        println!("SMA20: {}\tSMA50: {}\tSMA100: {}", sma20, sma50, sma100);

        let ema20 = ta::ema(mr_soft_candles, 20);
        let ema50 = ta::ema(mr_soft_candles, 50);

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
