use clap::Parser;
use log::info;
use serde_json::{json, Map, Value};
use std::{path::Path, thread, time::Duration};

#[path = "../account/mod.rs"]
mod account;
use account::tdameritrade::{get_tdameritrade_creds, TDAmeritradeAccount};

#[path = "../logger/mod.rs"]
mod logger;
use logger::init_logging;

mod types;
use types::MateCandle;

mod state;
use state::{read_file, write_file};

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
    #[clap(long)]
    filepath: String,

    #[clap(short, long)]
    watch: Vec<String>,

    #[clap(short, long, parse(from_occurrences))]
    verbose: usize,
}
fn main() {
    let args = Args::parse();
    init_logging(args.verbose);

    info!("Starting collector");

    let (client_id, refresh_token) = get_tdameritrade_creds();
    let mut td_account = TDAmeritradeAccount::new(
        "TDAmeritrade",
        "My account",
        client_id.as_str(),
        refresh_token.as_str(),
    );

    let mut data: Map<String, Value> = Map::new();
    let filepath = format!("{}/data.txt", args.filepath);
    let datapath = Path::new(filepath.as_str());

    loop {
        if datapath.exists() {
            data = read_file(format!("{}/data.txt", args.filepath).as_str());
            info!("Read state file from last iteration")
        }

        for symbol in args.watch.iter() {
            let candles = td_account.get_candles(symbol.to_string());

            let mut new_mate_candles: Vec<MateCandle> = Vec::new();
            for candle in candles {
                let new_mc = MateCandle {
                    close: candle.close,
                    datetime: candle.datetime,
                    high: candle.high,
                    low: candle.low,
                    open: candle.open,
                    volume: candle.volume,
                };
                new_mate_candles.push(new_mc);
            }
            info!("Fetched data for {}", symbol);
            data.insert(symbol.clone(), json!(new_mate_candles));
        }

        write_file(format!("{}/data.txt", args.filepath).as_str(), &mut data);

        // sleep for an hour, as not to miss any trading window
        let hour = Duration::from_secs(60 * 60);
        // let day = Duration::from_secs(60 * 60 * 24);
        thread::sleep(hour);
    }
}
