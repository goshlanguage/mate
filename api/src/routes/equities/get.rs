use actix_web::HttpRequest;
use serde_json::{json, Map, value::Value};

pub async fn get(req: HttpRequest) -> String {
  let symbol = req.match_info().get("symbol").unwrap().to_string();
  let symbol_ref = symbol.clone();

  let mut payload = Map::new();
  let err = Value::String(format!("{} not found", symbol_ref));
  payload.insert("error".to_string(), err);

  json!(payload).to_string()
}
