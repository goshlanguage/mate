use clap::Parser;
use env_logger::Builder;
use log::{info, LevelFilter};
use std::{collections::HashMap, thread, time::Duration};
use tda_sdk::responses::Candle;

mod account;
use account::tdameritrade::{get_creds, TDAmeritradeAccount};

mod ta;
use ta::average::{ema, sma};

/// You can see the spec for clap's arg attributes here:
///      <https://github.com/clap-rs/clap/blob/v3.0.0-rc.11/examples/derive_ref/README.md#arg-attributes>
#[derive(Parser, Debug)]
#[clap(
    name = "mate",
    about = "mini algorithmic trading engine",
    version = "0.1.0",
    author
)]
struct Args {
    #[clap(short, long, parse(from_occurrences))]
    verbose: usize,
}

// mate makes use of the tda-sdk crate for access to a brokerage API
// https://github.com/rideron89/tda-sdk-rs
pub struct Mate {
    account: TDAmeritradeAccount,
    candles: HashMap<String, Vec<Candle>>,
    symbols: Vec<String>,
}

impl Mate {
    pub fn default() -> Self {
        let (client_id, refresh_token) = get_creds();
        let td_account = TDAmeritradeAccount::new(
            "TDAmeritrade",
            "My account",
            client_id.as_str(),
            refresh_token.as_str(),
        );

        Mate {
            account: td_account,
            candles: HashMap::new(),
            symbols: vec![],
        }
    }

    pub fn status(&self) {
        self.account.get_account_ids();
    }
}

// init_logging is a helper that parses output from clap's get_matches()
//   and appropriately sets up the desired log level
fn init_logging(log_level: usize) {
    match log_level {
        0 => env_logger::init(),
        1 => {
            Builder::default().filter(None, LevelFilter::Warn).init();
        }
        2 => {
            Builder::default().filter(None, LevelFilter::Info).init();
        }
        3 => {
            Builder::default().filter(None, LevelFilter::Debug).init();
        }
        _ => {
            Builder::default().filter(None, LevelFilter::Trace).init();
        }
    }
}

fn main() {
    let args = Args::parse();
    init_logging(args.verbose);

    let mut mate = Mate::default();
    mate.symbols = vec!["MSFT".to_string()];

    loop {
        for symbol in &mate.symbols {
            mate.candles.insert(
                symbol.to_string(),
                mate.account.get_candles(symbol.to_string()),
            );
        }
        mate.status();

        let msft_candles = mate.candles.get("MSFT").unwrap();

        let sma20 = sma(msft_candles, 0, 20);
        let sma50 = sma(msft_candles, 0, 50);
        let sma100 = sma(msft_candles, 0, 100);

        info!("SMA20: {}\tSMA50: {}\tSMA100: {}", sma20, sma50, sma100);

        let ema20 = ema(msft_candles, 20);
        let ema50 = ema(msft_candles, 50);

        // TODO Check for NaN values to ensure we don't submit a faulty order
        info!("EMA20: {}\tEMA50: {}", ema20, ema50);
        if ema20 > 0.0 && ema50 > 0.0 {
            if ema20 > ema50 {
                info!("buy");
                info!("set stop loss at 95%");
            } else {
                info!("sell");
            }
        }

        // sleep for an hour, as not to miss any trading window
        let hour = Duration::from_secs(60 * 60);
        // let day = Duration::from_secs(60 * 60 * 24);
        thread::sleep(hour);
    }
}
