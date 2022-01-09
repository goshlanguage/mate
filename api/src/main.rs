use actix_web::{App, HttpServer};
use clap::Parser;
use matelog::init_logging;

mod routes;

/// You can see the spec for clap's arg attributes here:
///      <https://github.com/clap-rs/clap/blob/v3.0.0-rc.11/examples/derive_ref/README.md#arg-attributes>
#[derive(Parser, Debug)]
#[clap(
    name = "mate-api",
    about = "api for mate",
    version = "0.1.0",
    author
)]

struct Args {
    #[clap(short, long, parse(from_occurrences))]
    verbose: usize,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  let args = Args::parse();
  init_logging(args.verbose);

  HttpServer::new(move || {
      App::new().configure(routes::api_factory)
  })
      .bind("127.0.0.1:8000")?
      .workers(3)
      .run()
      .await
}
