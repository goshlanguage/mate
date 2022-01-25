use super::route::Router;
use actix_web::web;
mod handlers;
mod utils;

pub mod errors;

pub fn users_factory(app: &mut web::ServiceConfig) {
  let router: Router = Router {
      prefix: String::from("/users"),
  };

  // R User 1
  app.route(
      &router.new_route(String::from("/{id}")),
      web::get().to(handlers::get),
  );
  // C Account All
  // curl -i -XPOST -d '{"first_name":"Ryan","last_name":"Hartje","email":"a@b.c"}' -H 'Content-Type: application/json' http://localhost:8000/users/
  app.route(
      &router.new_route(String::from("/")),
      web::post().to(handlers::add_user),
  );
  // D Account 1
  app.route(
      &router.new_route(String::from("/{id}")),
      web::delete().to(handlers::delete_user),
  );
}