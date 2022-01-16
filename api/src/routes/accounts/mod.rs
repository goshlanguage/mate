use actix_web::web;
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
    &route.new(String::from("/{id}")),
    web::delete().to(handlers::delete)
  );
  app.route(
    &route.new(String::from("/{name}")),
    web::get().to(handlers::get)
  );
  app.route(
    &route.new(String::from("/")),
    web::post().to(handlers::post)
  );
}
