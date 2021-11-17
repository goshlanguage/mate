use chrono::Utc;
use chrono::prelude::*;
use std::{
    collections::HashMap,
    env,
    thread,
    time::Duration,
};
use tda_sdk::{
    AccessToken,
    Client,
    params::GetAccountsParams,
    params::GetPriceHistoryParams,
    responses::SecuritiesAccount,
};

// mate makes use of the tda-sdk crate for access to a brokerage API
// https://github.com/rideron89/tda-sdk-rs

pub struct Mate {
    client: Client,
    ema_cache: HashMap<String, Vec<f64>>,
}

impl Mate {
    pub fn new() -> Self {
        let (client_id, refresh_token) = get_creds();
        let mut client = Client::new(&client_id, &refresh_token, None);

        let access_token: AccessToken = client.get_access_token().unwrap().into();

        // TODO
        // this function seems to not work currently
        // This needs a working expiry check, so when expired, a token can be refreshed
        if access_token.has_expired() {
            let _dt = Utc.timestamp(access_token.expires_at, 0);
            let _now = Utc::now();
            // println!("Current access token is supposedly expired!\nCurrent time: {}", now.to_rfc2822());
            // println!("Access token expiry: {}", dt.to_rfc2822())
        }

        client.set_access_token(&Some(access_token.into()));

        return Mate{
            client: client,
            ema_cache: HashMap::new(),
        }
    }

    // status reflects a TD Ameritrade account balance and other information.
    pub fn status(&self) -> String {
        let mut status = String::new();

        let accounts = self.client.get_accounts(GetAccountsParams::default()).unwrap();

        for account in accounts {
            match account.securities_account {
                SecuritiesAccount::MarginAccount { is_day_trader, projected_balances, .. } => {
                    // status = format!("{}Account ID: {}\n", status, account_id);
                    // status = format!("{}Account Type: {}\n", status, r#type);
                    status = format!("{}Account Balance: ${}\n", status, projected_balances.cash_available_for_withdrawal);
                    status = format!("{}Account DayTrader: {}\n", status, is_day_trader);
                }
            }
        }
        return status.to_string()
    }

    // https://www.investopedia.com/terms/e/ema.asp
    // symbol - ticker symbol of the security you want to query
    // period - number in days to grab info for
    //
    // ema first checks the ema cache, and if unpopulated, computes a table of ema for as many candles as we get back
    // then returns the last element of the ema cache, which should be the most recent.
    //
    // returns computed ema as f64, and a bool if an error was encountered
    // TODO better error handling
    fn ema(&mut self, symbol: &str, period: i8) -> (f64, bool) {
        if self.ema_cache.is_empty() {
            // https://developer.tdameritrade.com/price-history/apis/get/marketdata/%7Bsymbol%7D/pricehistory
            let params = GetPriceHistoryParams::default();
            let history = self.client.get_price_history(&symbol, params);

            let resp = match history {
                Ok(val) => val,
                Err(e) => {
                    println!("Failed to get price history: {}", e.to_string());
                    return (0.0, true)
                },
            };

            // println!("Price history for {}: {:?}", resp.symbol, resp);

            // TODO
            // Calculate the SMA for the first 20 candles, using the 21st candle as the first candle to start
            // producing an EMA using the SMA at index 20 for the first EMA calculation
            //
            // on the first iteration, set index 0 to the first candle's closing price to avoid a later potential divide by zero issue
            self.ema_cache.insert(symbol.to_string(), vec![resp.candles.get(0).unwrap().close]);

            for i in 1..resp.candles.len() {
                let candle = resp.candles.get(i).unwrap();

                if i <= 20 {
                    match self.ema_cache.get_mut(&symbol.to_string()) {
                        Some(list) => { list.push(candle.close) },
                        None => { panic!("Failed to use HashMap for EMA calculation after initial entry. Can not recover.") },
                    }
                } else {
                    match self.ema_cache.get_mut(&symbol.to_string()) {
                        Some(list) => {
                            let smoothing_factor = 2.0;
                            let multiplier = smoothing_factor / (1.0 + f64::from(period));
                            let ema = (candle.close * multiplier) + (list[i-1] * (1.0 - multiplier));
                            list.push(ema)
                        },
                        None => { panic!("Failed to use HashMap for EMA calculation after initial entry. Can not recover.") },
                    }
                }
            }

        }

        match self.ema_cache.get_mut(&symbol.to_string()) {
            Some(list) => {
                return (list[list.len()-1], false) },
            None => { return (0.0, true) },
        }
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

    return (client_id, refresh_token)
}

fn main() {
    let mut mate = Mate::new();
    println!("{}", mate.status());

    loop {
        let (ema20, err) = mate.ema("MSFT", 20);
        if err {
            println!("error computing ema20");
        }

        let (ema50, err) = mate.ema("MSFT", 50);
        if err {
            println!("error computing ema50");
        }

        println!("Calculated EMA20: {}\tEMA50: {}", ema20, ema50);
        if ema20 > ema50 {
            println!("buy");
        } else {
            println!("sell");
        }

        thread::sleep(Duration::from_secs(60));
    }
}
