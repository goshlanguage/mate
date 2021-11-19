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
    responses::Candle,
    responses::SecuritiesAccount,
};

// mate makes use of the tda-sdk crate for access to a brokerage API
// https://github.com/rideron89/tda-sdk-rs

pub struct Mate {
    client: Client,
    ema_cache: HashMap<String, Vec<f64>>,
    candles: Vec<Candle>,
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
            ema_cache: HashMap::new(),
            candles: vec![],
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
    // returns computed ema as f64
    fn ema(&mut self, symbol: &str, period: i32) -> f64 {

        let candle_len = self.candles.len();
        let sma_i =  candle_len - ( 2 * period as usize );
        let sma_e = candle_len - period as usize;

        let base_case = self.sma(symbol, sma_i as usize, sma_e as usize);


        let close = self.candles[self.candles.len()-1].close;
        let smoothing_factor = 2.0;
        let multiplier = smoothing_factor / (1.0 + f64::from(period));
        let ema1 = (close * multiplier) + (base_case * (1.0 - multiplier));

        let emas = vec![ema1];
        // EMA = Closing price x multiplier + EMA (previous day) x (1-multiplier)
        return emas[emas.len()-1]
    }

    fn sma(&mut self, symbol: &str, start: usize, end: usize) -> f64 {
        println!("Calculating SMA for {} from {} to {}", symbol, start, end);

        let mut sum = 0.0;

        for i in start..end {
            let i_usize = i as usize;
            sum += self.candles[self.candles.len()-1-i_usize].close;
        }

        let difference = (end - start) as f64;
        let average = sum / difference;
        return average
    }

    fn refresh_candles(&mut self) {
        // https://developer.tdameritrade.com/price-history/apis/get/marketdata/%7Bsymbol%7D/pricehistory
        // let params = GetPriceHistoryParams::default();
        let params = GetPriceHistoryParams{
            end_date: None,
            frequency_type: None,
            frequency: None,
            need_extended_hours_data: None,
            period_type: Some(String::from("day")),
            period: Some(String::from("1")),
            start_date: None,
        };

        let history = self.client.get_price_history(&self.symbols[0].to_owned(), params);

        let resp = match history {
            Ok(val) => val,
            Err(e) => {
                println!("Failed to get price history: {}", e.to_string());
                return
            },
        };

        let start_ms = resp.candles[0].datetime as i64;
        let end_ms = resp.candles[resp.candles.len()-1].datetime as i64;

        let start_time = start_ms / 1000;
        let end_time = end_ms / 1000;

        let naive_start = NaiveDateTime::from_timestamp(start_time, 0);
        let naive_end = NaiveDateTime::from_timestamp(end_time, 0);

        let datetime_start: DateTime<Utc> = DateTime::from_utc(naive_start, Utc);
        let datetime_end: DateTime<Utc> = DateTime::from_utc(naive_end, Utc);

        let readable_start = datetime_start.to_rfc2822();
        let readable_end = datetime_end.to_rfc2822();

        println!("Gathered candle data for {} between {} and {}", resp.symbol, readable_start, readable_end);
        println!("Candle data contains {} candles", resp.candles.len());

        self.candles = resp.candles;
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

    mate.symbols = vec!["MSFT".to_string()];

    loop {
        // ensure that data we're watching is fetched
        mate.refresh_candles();

        let sma20 = mate.sma("MSFT", 0, 20);
        let sma50 = mate.sma("MSFT", 0, 50);
        let sma100 = mate.sma("MSFT", 0, 100);

        println!("Calculated SMA20: {}\tSMA50: {}\tSMA100: {}", sma20, sma50, sma100);

        let ema20 = mate.ema("MSFT", 20);
        let ema50 = mate.ema("MSFT", 50);

        println!("Calculated EMA20: {}\tEMA50: {}", ema20, ema50);
        if ema20 > ema50 {
            println!("buy");
        } else {
            println!("sell");
        }

        thread::sleep(Duration::from_secs(60));
    }
}
