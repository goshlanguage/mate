use clap::Parser;
use env_logger::Builder;
use log::{info, LevelFilter};
use std::{thread, time::Duration};

#[path = "../account/mod.rs"]
mod account;
use account::tdameritrade::{get_creds, TDAmeritradeAccount};

/// You can see the spec for clap's arg attributes here:
///      <https://github.com/clap-rs/clap/blob/v3.0.0-rc.11/examples/derive_ref/README.md#arg-attributes>
#[derive(Parser, Debug)]
#[clap(
    name = "mate-collector",
    about = "collects data for local caching",
    version = "0.1.0",
    author
)]
struct Args {
    #[clap(short, long)]
    watch: Vec<String>,

    #[clap(short, long, parse(from_occurrences))]
    verbose: usize,
}
fn main() {
    let args = Args::parse();
    init_logging(args.verbose);

    info!("Starting collector");

    let (client_id, refresh_token) = get_creds();
    let mut td_account = TDAmeritradeAccount::new(
        "TDAmeritrade",
        "My account",
        client_id.as_str(),
        refresh_token.as_str(),
    );

    loop {
        for symbol in args.watch.iter() {
            let candles = td_account.get_candles(symbol.to_string());
            info!("Fetched {} candles for {}", candles.len(), symbol);
        }

        // sleep for an hour, as not to miss any trading window
        let hour = Duration::from_secs(60 * 60);
        // let day = Duration::from_secs(60 * 60 * 24);
        thread::sleep(hour);
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
