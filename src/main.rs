use clap::Parser;
use log::info;
use std::{collections::HashMap, thread, time::Duration};
use tda_sdk::responses::Candle;

mod account;
use account::kraken::{get_kraken_creds, KrakenAccount};
use account::tdameritrade::{get_tdameritrade_creds, TDAmeritradeAccount};
use account::types::AccountType;

mod logger;
use logger::init_logging;

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
    accounts: Vec<AccountType>,
    candles: HashMap<String, Vec<Candle>>,
    symbols: Vec<String>,
}

impl Mate {
    pub fn default() -> Self {
        let (client_id, refresh_token) = get_tdameritrade_creds();
        let td_account = TDAmeritradeAccount::new(
            "TDAmeritrade",
            "My td ameritrade account",
            client_id.as_str(),
            refresh_token.as_str(),
        );

        let (client_key, client_secret) = get_kraken_creds();
        let kraken_account = KrakenAccount::new(
            "Kraken",
            "My kraken account",
            client_key.as_str(),
            client_secret.as_str(),
        );

        let accounts: Vec<AccountType> = vec![
            AccountType::TDAmeritradeAccount(td_account),
            AccountType::KrakenAccount(kraken_account),
        ];

        Mate {
            accounts: accounts,
            candles: HashMap::new(),
            symbols: vec![],
        }
    }

    pub fn status(&self) {
        for account in &self.accounts {
            match account {
                AccountType::TDAmeritradeAccount(account) => {
                    info!("Found TDAmeritrade Accounts: {}", account.get_account_ids())
                }
                AccountType::KrakenAccount(account) => {
                    info!("Found Kraken Accounts: {}", account.get_account_balances());
                }
            }
        }
    }

    pub fn update_td(&mut self, mut account: TDAmeritradeAccount) {
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

    let mut mate = Mate::default();
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
                _ => info!("Unknown account found"),
            }
        }

        // sleep for an hour, as not to miss any trading window
        let hour = Duration::from_secs(60 * 60);
        // let day = Duration::from_secs(60 * 60 * 24);
        thread::sleep(hour);
    }
}
