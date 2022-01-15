use super::route::Route;
use actix_web::web;
mod get;

pub fn equities_factory(app: &mut web::ServiceConfig) {
  let route: Route = Route {prefix: String::from("/data")};
  app.route(
    &route.new(String::from("/get/{symbol}")),
    web::post().to(get::get)
  );
}
