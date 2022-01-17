use super::route::Router;
use actix_web::web;
mod handlers;
mod utils;

pub fn accounts_factory(app: &mut web::ServiceConfig) {
    let router: Router = Router {
        prefix: String::from("/accounts"),
    };
    // C Account All
    app.route(
        &router.new_route(String::from("/")),
        web::post().to(handlers::post),
    );
    // R Account All
    app.route(
        &router.new_route(String::from("/")),
        web::get().to(handlers::get_all),
    );
    // R Account 1
    app.route(
        &router.new_route(String::from("/{id}")),
        web::get().to(handlers::get),
    );
    // R Balance All
    app.route(
        &router.new_route(String::from("/balance/")),
        web::get().to(handlers::get_balance_all),
    );
    // R Balance 1
    app.route(
        &router.new_route(String::from("/balance/{id}")),
        web::get().to(handlers::get_balance),
    );
    // R Summary All
    app.route(
        &router.new_route(String::from("/summary/")),
        web::get().to(handlers::get_summary_all),
    );
    // U Balance 1
    app.route(
        &router.new_route(String::from("/balance/")),
        web::put().to(handlers::update_balance),
    );
    // D Account 1
    app.route(
        &router.new_route(String::from("/{id}")),
        web::delete().to(handlers::delete),
    );
}
