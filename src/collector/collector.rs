use clap::{App, Arg, ArgMatches};
use env_logger::Builder;
use log::{error, info, LevelFilter};
use std::{borrow::Borrow, process::exit};

fn main() {
    let matches = App::new("mate-collector")
        .version("v0.1.0")
        .about("A data collector for mate")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .arg(
            Arg::with_name("source")
                .short("s")
                .long("source")
                .multiple(true)
                .takes_value(true)
                .env("MATE_COLLECTOR_SOURCE")
                .help("configures the collector to use the given source"),
        )
        .get_matches();

    init_logging(matches.borrow());

    let sources = get_sources(matches.borrow());
}

// init_logging is a helper that parses output from clap's get_matches()
//   and appropriately sets up the desired log level
fn init_logging(matches: &ArgMatches) {
    match matches.occurrences_of("v") {
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

fn get_sources(matches: &ArgMatches) -> Vec<String> {
    let mut valid_sources: Vec<String> = vec![];

    if matches.occurrences_of("source") > 0 {
        let sources: Vec<_> = matches.values_of("source").unwrap().collect();
        for source in sources {
            match source {
                "tdameritrade" => {
                    info!("enabling tdameritrade");
                    valid_sources.push(source.to_string());
                }
                _ => {
                    error!("unsupported source {}, exiting", source);
                }
            }
        }
    } else {
        error!("No active data source loaded. Please pass a source via the --source flag. Exiting");
        exit(1);
    }

    return valid_sources;
}
