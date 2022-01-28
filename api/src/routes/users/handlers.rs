use super::utils::*;
use crate::models::NewUserPayload;
use actix_web::{web, HttpRequest, Responder};

pub async fn get(req: HttpRequest) -> impl Responder {
    let id: i32 = req
        .match_info()
        .get("id")
        .unwrap()
        .to_string()
        .parse()
        .unwrap();
    get_user_by_id(id)
}

pub async fn add_user(payload: web::Json<NewUserPayload>) -> impl Responder {
    create_user(&payload)
}

pub async fn delete_user(req: HttpRequest) -> impl Responder {
    let id: i32 = req
        .match_info()
        .get("id")
        .unwrap()
        .to_string()
        .parse()
        .unwrap();

    delete(id);
    "ok".to_string()
}
