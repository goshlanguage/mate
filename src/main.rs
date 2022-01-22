use clap::Parser;
use log::info;
use std::{collections::HashMap, thread, time::Duration};
use tda_sdk::responses::Candle;

use accounts::kraken::KrakenAccount;
use accounts::tdameritrade::TDAmeritradeAccount;
use accounts::types::AccountType;

use matelog::init_logging;

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
    #[clap(short, long, default_value = "tdameritrade")]
    accounts: Vec<String>,

    #[clap(short, long, parse(from_occurrences))]
    verbose: usize,
}

// mate makes use of the tda-sdk crate for access to a brokerage API
// https://github.com/rideron89/tda-sdk-rs
pub struct Mate {
    accounts: Vec<AccountType>,
    candles: HashMap<String, Vec<Candle>>,
    symbols: Vec<String>,
}

impl Mate {
    pub fn new(accounts: Vec<String>) -> Mate {
        let mut mate = Mate {
            accounts: Vec::new(),
            candles: HashMap::new(),
            symbols: Vec::new(),
        };

        for account in accounts {
            let new_account =
                accounts::new_account(account.as_str(), account.as_str(), "", None, "", "")
                    .unwrap();
            mate.accounts.push(new_account);
        }

        mate
    }

    pub fn default() -> Self {
        Mate {
            accounts: Vec::new(),
            candles: HashMap::new(),
            symbols: Vec::new(),
        }
    }

    pub fn status(&self) {
        for account in &self.accounts {
            match account {
                AccountType::TDAmeritradeAccount(account) => {
                    info!("Found TDAmeritrade Accounts: {}", account.get_account_ids())
                }
                AccountType::KrakenAccount(account) => {
                    info!("Found Kraken Accounts: {}", account.get_account_balance());
                }
            }
        }
    }

    pub fn update_td(&mut self, account: TDAmeritradeAccount) {
        let symbols = self.symbols.clone();
        for symbol in symbols {
            self.candles
                .insert(symbol.to_string(), account.get_candles(symbol.to_string()));
        }

        let msft_candles = self.candles.get("MSFT").unwrap();

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
    }

    pub fn update_kraken(&self, account: KrakenAccount) {
        info!("BTC: {}", account.get_pairs("XXBTZUSD").as_str());
        info!("ETH: {}", account.get_pairs("XETHZUSD").as_str());
    }
}

fn main() {
    let args = Args::parse();
    init_logging(args.verbose);

    let mut mate = Mate::new(args.accounts);
    mate.symbols = vec!["MSFT".to_string()];

    loop {
        mate.status();

        for account in mate.accounts.clone() {
            match account {
                AccountType::TDAmeritradeAccount(account) => {
                    mate.update_td(account);
                }
                AccountType::KrakenAccount(account) => {
                    mate.update_kraken(account);
                }
            }
        }

        // sleep for an hour, as not to miss any trading window
        let hour = Duration::from_secs(60 * 60);
        // let day = Duration::from_secs(60 * 60 * 24);
        thread::sleep(hour);
    }
}
