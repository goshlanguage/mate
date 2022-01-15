use actix_web::web;

mod logout;
use super::route::Route;

pub fn auth_factory(app: &mut web::ServiceConfig) {
    let auth_routes: Route = Route {
        prefix: String::from("/auth"),
    };

    app.route(
        &auth_routes.new(String::from("/logout")),
        web::get().to(logout::logout),
    );
}
