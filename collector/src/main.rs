use clap::Parser;
use log::info;
use std::{thread, time::Duration};

use matelog::init_logging;

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

    #[clap(long)]
    s3_bucket: Option<String>,

    #[clap(long, default_value = "https")]
    s3_proto: String,

    #[clap(long, default_value = "us-east-1")]
    s3_region: String,

    #[clap(short, long)]
    crypto: Vec<String>,

    #[clap(long)]
    filepath: String,

    #[clap(short, long, default_value_t = 3600)]
    poll_seconds: u64,

    #[clap(short, long)]
    stock: Vec<String>,

    #[clap(short, long, parse(from_occurrences))]
    verbose: usize,
}

fn init(args: Args) -> Collector {
    init_logging(args.verbose);

    let bucket = match args.s3_bucket {
        Some(b) => b,
        None => "".to_string(),
    };

    let conf = CollectorConfig {
        accounts: args.accounts,
        crypto_watchlist: args.crypto,
        poll_seconds: args.poll_seconds,
        s3_bucket: bucket,
        s3_proto: args.s3_proto,
        s3_region: args.s3_region,
        stock_watchlist: args.stock,
        filepath: args.filepath,
    };

    Collector::new(conf)
}

fn main() {
    let args = Args::parse();
    let collector = init(args);

    info!("Starting collector");

    loop {
        collector.update();

        info!("Time to sleep (¬‿¬)");

        let poll_duration = Duration::from_secs(collector.conf.poll_seconds);
        thread::sleep(poll_duration);
    }
}
