use crate::models::Account;

use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use futures::future::{Ready, ready};
use serde::Serialize;

// #[derive(Queryable, Serialize)]
// //#[table_name="accounts"]
// pub struct Account {
//     pub name: String,
//     pub balance: f64,
//     pub balance_history: Vec<f64>,
// }

// impl Account{
//   fn new(name: String, balance: f64) -> Account {
//     Account {
//       name,
//       balance,
//       balance_history: vec![balance],
//     }
//   }
// }

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
