use bytes::Bytes;
use clap::Parser;
use env_logger::Builder;
use hyper::{
    body::to_bytes,
    service::{make_service_fn, service_fn},
    Body, Request, Server,
};
use log::{info, LevelFilter};
use route_recognizer::Params;
use router::Router;
use std::sync::Arc;
use tokio_postgres::NoTls;

mod handlers;
mod router;

type Response = hyper::Response<hyper::Body>;
type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Clone, Debug)]
pub struct AppState {
    pub brokers: Vec<String>,
}

// You can see the spec for clap's arg attributes here:
//      https://github.com/clap-rs/clap/blob/v3.0.0-rc.11/examples/derive_ref/README.md#arg-attributes
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short, long, default_value_t = 8080)]
    port: i32,

    #[clap(long, default_value = "postgres")]
    postgres_database: String,

    #[clap(long, default_value = "127.0.0.1")]
    postgres_hostname: String,

    #[clap(long, env = "POSTGRES_PASSWORD")]
    postgres_password: String,

    #[clap(long, default_value_t = 5432)]
    postgres_port: i32,

    #[clap(long, default_value = "postgres")]
    postgres_user: String,

    #[clap(short, long, parse(from_occurrences))]
    verbose: usize,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();
    init_logging(args.verbose);

    let psql_conn_string = format!(
        "host={} port={} user={} password={}",
        args.postgres_hostname, args.postgres_port, args.postgres_user, args.postgres_password
    );
    info!("psql connstring: {}", psql_conn_string);

    // Connect to the database.
    let (client, connection) = tokio_postgres::connect(psql_conn_string.as_str(), NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let rows = client.query("SELECT $1::TEXT", &[&"hello world"]).await?;

    let value: &str = rows[0].get(0);
    info!("{}", value);

    let mut router: router::Router = Router::new();
    router.get("/brokers/", Box::new(handlers::brokers_get));
    router.get("/brokers/:broker", Box::new(handlers::brokers_get_one));
    router.post("/brokers/", Box::new(handlers::brokers_update));
    // router.delete("/brokers/", Box::new(handlers::brokers_delete));

    let shared_router = Arc::new(router);

    let state = Vec::new();
    let new_service = make_service_fn(move |_| {
        let app_state = AppState {
            brokers: state.clone(),
        };

        let router_capture = shared_router.clone();
        async {
            Ok::<_, Error>(service_fn(move |req| {
                route(router_capture.clone(), req, app_state.clone())
            }))
        }
    });

    let addr = format!("0.0.0.0:{}", args.port)
        .parse()
        .expect("address creation works");
    let server = Server::bind(&addr).serve(new_service);
    info!("Listening on http://{}", addr);
    let _ = server.await;

    Ok(())
}

async fn route(
    router: Arc<Router>,
    req: Request<hyper::Body>,
    app_state: AppState,
) -> Result<Response, Error> {
    let found_handler = router.route(req.uri().path(), req.method());
    let resp = found_handler
        .handler
        .invoke(Context::new(app_state, req, found_handler.params))
        .await;
    Ok(resp)
}

#[derive(Debug)]
pub struct Context {
    pub state: AppState,
    pub req: Request<Body>,
    pub params: Params,
    body_bytes: Option<Bytes>,
}

impl Context {
    pub fn new(state: AppState, req: Request<Body>, params: Params) -> Context {
        Context {
            state,
            req,
            params,
            body_bytes: None,
        }
    }

    pub async fn body_json<T: serde::de::DeserializeOwned>(&mut self) -> Result<T, Error> {
        let body_bytes = match self.body_bytes {
            Some(ref v) => v,
            _ => {
                let body = to_bytes(self.req.body_mut()).await?;
                self.body_bytes = Some(body);
                self.body_bytes.as_ref().expect("body_bytes was set above")
            }
        };
        Ok(serde_json::from_slice(&body_bytes)?)
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
