use actix_web::web;
use super::route::Route;
mod handlers;
mod utils;

pub fn accounts_factory(app: &mut web::ServiceConfig) {
  let route: Route = Route {prefix: String::from("/accounts")};
  // C Account All
  app.route(
    &route.new(String::from("/")),
    web::post().to(handlers::post)
  );
  // R Account All
  app.route(
    &route.new(String::from("/")),
    web::get().to(handlers::get_all)
  );
  // R Account 1
  app.route(
    &route.new(String::from("/{id}")),
    web::get().to(handlers::get)
  );
  // R Balance All
  app.route(
    &route.new(String::from("/balance/")),
    web::get().to(handlers::get_balance_all)
  );
  // R Balance 1
  app.route(
    &route.new(String::from("/balance/{id}")),
    web::get().to(handlers::get_balance)
  );
  // R Summary All
  app.route(
    &route.new(String::from("/summary/")),
    web::get().to(handlers::get_summary_all)
  );
  // U Balance 1
  app.route(
    &route.new(String::from("/balance/")),
    web::put().to(handlers::update_balance)
  );
  // D Account 1
  app.route(
    &route.new(String::from("/{id}")),
    web::delete().to(handlers::delete)
  );
}
