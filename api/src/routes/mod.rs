use actix_web::web;
mod accounts;
mod route;

pub fn api_factory(app: &mut web::ServiceConfig) {
    accounts::accounts_factory(app);
}
