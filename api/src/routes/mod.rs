use actix_web::web;
mod auth;
mod accounts;
mod equities;
mod route;

pub fn api_factory(app: &mut web::ServiceConfig) {
    auth::auth_factory(app);
    accounts::accounts_factory(app);
    equities::equities_factory(app);
}
