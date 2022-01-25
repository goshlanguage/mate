use actix_web::web;
mod accounts;
mod route;
pub mod users;

pub fn api_factory(app: &mut web::ServiceConfig) {
    accounts::accounts_factory(app);
    users::users_factory(app);
}
