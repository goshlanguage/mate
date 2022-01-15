use actix_web::web;
use diesel::prelude::*;
use crate::models::{Account, NewAccount};
use crate::schema::accounts;
use super::route::Route;
mod handlers;
mod types;

pub fn accounts_factory(app: &mut web::ServiceConfig) {
  let route: Route = Route {prefix: String::from("/accounts")};
  app.route(
    &route.new(String::from("/")),
    web::get().to(handlers::get_all)
  );
  app.route(
    &route.new(String::from("/{name}")),
    web::get().to(handlers::get)
  );
}

pub fn create_account(conn: &PgConnection, name: String, balance: f64) -> Account {
  let new_account = NewAccount {
    name: name.as_str(),
    balance: &balance,
    balance_history: &vec![balance]
  };

  // TODO
  // catch and return error here perhaps
  diesel::insert_into(accounts::table)
    .values(&new_account)
    .get_result(conn)
    .expect("Error saving new account")
}
