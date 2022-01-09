use actix_web::web;
mod route;
mod auth;

pub fn api_factory(app: &mut web::ServiceConfig) {
  auth::auth_factory(app);
}
