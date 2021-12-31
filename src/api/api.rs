use clap::{Arg, App, ArgMatches};
use env_logger::{Builder};
use log::{info, error, LevelFilter};
use std::{process::exit, borrow::Borrow};

fn main() {
  let matches = App::new("mate-api")
      .version("v0.1.0")
      .about("mini algorithmic trading engine API")
      .arg(Arg::with_name("config")
          .short("c")
          .long("config")
          .value_name("FILE")
          .help("Sets a custom config file")
          .takes_value(true))
      .arg(Arg::with_name("v")
          .short("v")
          .multiple(true)
          .help("Sets the level of verbosity"))
      .get_matches();

  init_logging(matches.borrow());

  exit(0)

}

// init_logging is a helper that parses output from clap's get_matches()
//   and appropriately sets up the desired log level
fn init_logging(matches: &ArgMatches) {
  match matches.occurrences_of("v") {
    0 => env_logger::init(),
    1 => {
      Builder::default()
        .filter(None, LevelFilter::Warn)
        .init();
      },
    2 => {
      Builder::default()
      .filter(None, LevelFilter::Info)
      .init();
    },
    3 => {
      Builder::default()
      .filter(None, LevelFilter::Debug)
      .init();
    },
    _ => {
      Builder::default()
        .filter(None, LevelFilter::Trace)
        .init();
    }
  }
}