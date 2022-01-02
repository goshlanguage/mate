use crate::{AppState, Context, Response};
use hyper::StatusCode;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
struct BrokerRequest {
    name: String,
}

pub async fn brokers_get(ctx: Context) -> String {
    format!("{{'brokers': '{}'}}", ctx.state.brokers.join(","))
}

pub async fn brokers_get_one(ctx: Context) -> String {
    let param = ctx.params.find("broker").unwrap_or("");
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

    let mut old_ctx = ctx.state.brokers.clone();
    old_ctx.push(body.name);
    ctx.state = Arc::new(AppState { brokers: old_ctx });

    let resp = format!("{{'brokers': '{}'}}", ctx.state.brokers.join(","));
    Response::new(resp.into())
}
