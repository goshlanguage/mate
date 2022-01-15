use crate::models::Account;

use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use futures::future::{Ready, ready};
use serde::Serialize;

/// Responder is an actix_web Trait that generates responses
/// https://docs.rs/actix-web/0.4.5/actix_web/trait.Responder.html
impl Responder for Account {
  type Error = Error;
  type Future = Ready<Result<HttpResponse, Error>>;

  fn respond_to(self, _req: &HttpRequest) -> Self::Future {
    let body = serde_json::to_string(&self).unwrap();
    ready(Ok(HttpResponse::Ok()
      .content_type("application/json")
      .body(body)))
  }
}

/// The accounts type lets us return an array for the get method, and other methods to
/// represent results of searches, where an empty array represents no matches found.
#[derive(Serialize)]
pub struct Accounts {
  pub accounts: Vec<Account>,
}

impl Responder for Accounts {
  type Error = Error;
  type Future = Ready<Result<HttpResponse, Error>>;

  fn respond_to(self, _req: &HttpRequest) -> Self::Future {
    let body = serde_json::to_string(&self).unwrap();
    ready(Ok(HttpResponse::Ok()
      .content_type("application/json")
      .body(body)))
  }
}
