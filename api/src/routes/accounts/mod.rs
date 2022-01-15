use actix_web::web;
use diesel::prelude::*;
use crate::schema::accounts;
use super::route::Route;
mod handlers;
mod types;

pub fn accounts_factory(app: &mut web::ServiceConfig) {
  let route: Route = Route {prefix: String::from("/accounts")};
  // app.route(
  //   &route.new(String::from("/")),
  //   web::get().to(handlers::get_all)
  // );
  app.route(
    &route.new(String::from("/{account}")),
    web::get().to(handlers::get_all)
  );
}
