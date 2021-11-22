use chrono::Utc;
use chrono::prelude::*;

use std::{
    env,
    thread,
    time::Duration,
};

use std::collections::HashMap;

use tda_sdk::{
    AccessToken,
    Client,
    params::GetAccountsParams,
    params::GetPriceHistoryParams,
    responses::Candle,
    responses::SecuritiesAccount,
};

// mate makes use of the tda-sdk crate for access to a brokerage API
// https://github.com/rideron89/tda-sdk-rs

pub struct Mate {
    client: Client,
    candles: HashMap<String, Vec<Candle>>,
    symbols: Vec<String>
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
            candles: HashMap::new(),
            symbols: vec![],
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
    // You can cross check https://www.tradingview.com/symbols/$exchange-$symbol/technicals/ for validity/manual checking
    // eg: https://www.tradingview.com/symbols/NASDAQ-MSFT/technicals/
    //
    // TODO Fix no boundary checking
    // TODO check for NaN
    // returns computed ema as f64
    fn ema(&mut self, symbol: &str, period: i32) -> f64 {

        let sma_i =  (period) as usize;
        let sma_e = (2 * period) as usize;
        let base_case = self.sma(symbol, sma_i as usize, sma_e as usize);

        let len_candles = self.candles[symbol.clone()].len();
        let close = self.candles[symbol.clone()][len_candles-(period as usize)].close;
        let smoothing_factor = 2.0;
        let multiplier = smoothing_factor / (1.0 + f64::from(period));
        let ema0 = round((close - base_case) * multiplier + base_case);

        let mut emas = vec![ema0];
        // EMA = Closing price x multiplier + EMA (previous day) x (1-multiplier)
        for i in 0..period {
            let len_candles = self.candles[symbol.clone()].len();
            let close = self.candles[symbol.clone()][len_candles-((period - i) as usize)].close;
            let previous_ema = emas[i as usize];

            let ema_i = round((close - previous_ema) * multiplier + previous_ema);
            emas.push(ema_i);
        }

        return emas[emas.len()-1]
    }

    fn sma(&mut self, symbol: &str, start: usize, end: usize) -> f64 {
        let mut sum = 0.0;

        for i in start..end {
            let i_usize = i as usize;
            sum += self.candles[symbol.clone()][self.candles[symbol.clone()].len()-1-i_usize].close;
        }

        let difference = (end - start) as f64;
        let average = sum / difference;
        return round(average)
    }

    fn refresh_candles(&mut self) {

        for symbol in self.symbols.clone() {
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

            self.candles.insert(symbol, resp.candles);
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

    return (client_id, refresh_token)
}

// round is a helper for f64 that rounds the number to a decimal point notation used for representing money
fn round(i: f64) -> f64 {
    return (i * 100.0).round() / 100.0
}

fn main() {
    let mut mate = Mate::new();
    println!("{}", mate.status());

    mate.symbols = vec!["MSFT".to_string()];

    loop {
        // ensure that data we're watching is fetched
        mate.refresh_candles();

        let sma20 = mate.sma("MSFT", 0, 20);
        let sma50 = mate.sma("MSFT", 0, 50);
        let sma100 = mate.sma("MSFT", 0, 100);

        println!("SMA20: {}\tSMA50: {}\tSMA100: {}", sma20, sma50, sma100);

        let ema20 = mate.ema("MSFT", 20);
        let ema50 = mate.ema("MSFT", 50);

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
