#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;
mod routes;

use actix_web::{App, HttpServer, http::header};
use actix_cors::Cors;
use clap::Parser;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use matelog::init_logging;
use std::env;

/// You can see the spec for clap's arg attributes here:
///      <https://github.com/clap-rs/clap/blob/v3.0.0-rc.11/examples/derive_ref/README.md#arg-attributes>
#[derive(Parser, Debug)]
#[clap(name = "mate-api", about = "api for mate", version = "0.1.0", author)]

struct Args {
    #[clap(short, long, parse(from_occurrences))]
    verbose: usize,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    init_logging(args.verbose);


    HttpServer::new(move || {
        let cors = get_cors_policy();

        App::new()
            .wrap(cors)
            .configure(routes::api_factory)
    })
        .bind("127.0.0.1:8000")?
        .workers(3)
        .run()
        .await
}

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(format!("Error connecting to database").as_str())
}

/// get_cors_policy sets more permissive CORS policy if the environment is staging.
pub fn get_cors_policy() -> Cors {
    let env = match env::var("ENV") {
        Ok(v) => v,
        Err(_) => "staging".to_string(),
    };

    if env != "prod".to_string() {
        Cors::permissive()
    } else {
        Cors::default()
            .allowed_origin("http://mate.default.svc.cluster.local")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600)
    }
}
