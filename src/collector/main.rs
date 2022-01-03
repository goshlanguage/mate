use clap::Parser;
use log::info;
use std::{thread, time::Duration};

#[path = "../logger/mod.rs"]
mod logger;
use logger::init_logging;

mod types;
use types::{Collector, CollectorConfig};

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
    accounts: Vec<String>,

    #[clap(short, long)]
    crypto: Vec<String>,

    #[clap(long)]
    filepath: String,

    #[clap(short, long)]
    stock: Vec<String>,

    #[clap(short, long, parse(from_occurrences))]
    verbose: usize,
}

fn init(args: Args) -> Collector {
    init_logging(args.verbose);

    let conf = CollectorConfig {
        accounts: args.accounts,
        crypto_watchlist: args.crypto,
        stock_watchlist: args.stock,
        filepath: args.filepath,
    };

    let collector = Collector::new(conf);
    collector
}

fn main() {
    let args = Args::parse();
    let collector = init(args);

    info!("Starting collector");

    loop {
        collector.update();

        info!("Time to sleep (¬‿¬)");

        // sleep for an hour, as not to miss any trading window
        let hour = Duration::from_secs(60 * 60);
        // let day = Duration::from_secs(60 * 60 * 24);
        thread::sleep(hour);
    }
}
