use crate::{Context, Response};
use hyper::StatusCode;
use serde::Deserialize;

#[derive(Deserialize)]
struct BrokerRequest {
    name: String,
}

pub async fn brokers_get(ctx: Context) -> String {
    format!("{{'brokers': '{}'}}", ctx.state.brokers.join(","))
}

pub async fn brokers_get_one(ctx: Context) -> String {
    let param = match ctx.params.find("broker") {
        Some(v) => v,
        None => "empty",
    };
    format!("{{'brokers': '{}'}}", param)
}

pub async fn brokers_update(mut ctx: Context) -> Response {
    let body: BrokerRequest = match ctx.body_json().await {
        Ok(v) => v,
        Err(e) => {
            return hyper::Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(format!("could not parse JSON: {}", e).into())
            .unwrap()
        }
    };

    ctx.state.brokers.push(body.name.clone());
    let resp = format!("{{'brokers': '{}'}}", ctx.state.brokers.join(","));
    Response::new(resp.into())
}
