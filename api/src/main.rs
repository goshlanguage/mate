#[macro_use]
extern crate diesel;
#[macro_use]
extern crate magic_crypt;

pub mod models;
mod routes;
pub mod schema;

use actix_cors::Cors;
use actix_web::{http::header, App, HttpServer};
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
    #[clap(short, long, default_value_t = 8000)]
    port: i32,

    #[clap(long, default_value = "postgres")]
    postgres_database: String,

    #[clap(long, default_value = "127.0.0.1")]
    postgres_hostname: String,

    #[clap(long, env = "POSTGRES_PASSWORD", default_value = "")]
    postgres_password: String,

    #[clap(long, default_value_t = 5432)]
    postgres_port: i32,

    #[clap(long, default_value = "postgres")]
    postgres_user: String,

    #[clap(short, long, parse(from_occurrences))]
    verbose: usize,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    init_logging(args.verbose);

    let port = args.port;

    // ensure a DATABASE_URL is set in environment
    match env::var("DATABASE_URL") {
        Ok(_) => (),
        Err(_) => {
            let psql_conn_string = format!(
                "postgresql://{}:{}@{}:{}/{}",
                args.postgres_user,
                args.postgres_password,
                args.postgres_hostname,
                args.postgres_port,
                args.postgres_database
            );
            env::set_var("DATABASE_URL", psql_conn_string);
        }
    }

    HttpServer::new(move || {
        let cors = get_cors_policy();

        App::new().wrap(cors).configure(routes::api_factory)
    })
    .bind(format!("0.0.0.0:{}", port).as_str())?
    .workers(3)
    .run()
    .await
}

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect("Error connecting to database")
}

/// get_cors_policy sets more permissive CORS policy if the environment is staging.
pub fn get_cors_policy() -> Cors {
    let env = match env::var("ENV") {
        Ok(v) => v,
        Err(_) => "staging".to_string(),
    };

    if env != *"prod" {
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
