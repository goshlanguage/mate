use super::utils::*;
use crate::models::{NewAccountBalancesPayload, NewAccountPayload, UpdateAccountPayload};
use actix_web::{web, HttpRequest, Responder};

// CREATE
// curl -i -X POST -d '{"name":"tdameritrade", "vendor": "tdameritrade", "client_key": "", "client_secret": ""}' -H 'Content-Type: application/json' http://localhost:8000/accounts/
pub async fn post(payload: web::Json<NewAccountPayload>) -> impl Responder {
    create_account(&payload)
}

// READ ACCOUNT ID
// curl http://localhost:8000/accounts/1
pub async fn get(req: HttpRequest) -> impl Responder {
    let id: i32 = req
        .match_info()
        .get("id")
        .unwrap()
        .to_string()
        .parse()
        .unwrap();
    get_account(id)
}

// READ ACCOUNT ALL
// curl http://localhost:8000/accounts/
pub async fn get_all() -> impl Responder {
    get_accounts()
}

// READ BALANCE ID
// curl http://localhost:8000/accounts/
pub async fn get_balance(req: HttpRequest) -> impl Responder {
    let id: i32 = req
        .match_info()
        .get("id")
        .unwrap()
        .to_string()
        .parse()
        .unwrap();
    get_balance_by_id(id)
}

// READ BALANCE ALL
// curl http://localhost:8000/accounts/balance/
pub async fn get_balance_all() -> impl Responder {
    get_balances()
}

// READ ACCOUNT ALL
// curl http://localhost:8000/accounts/
pub async fn get_summary_all() -> impl Responder {
    get_summaries()
}

// UPDATE
// curl -i -X PUT -d '{"account_id":1, "name":"day trading account", "vendor": "tdameritrade", "client_key": "", "client_secret": ""}' -H 'Content-Type: application/json' http://localhost:8000/accounts/balance/
pub async fn put_account(payload: web::Json<UpdateAccountPayload>) -> impl Responder {
    update_account(&payload)
}

// UPDATE
// curl -i -X PUT -d '{"balances": [{"account_id":1, "balance": 475.78},{"account_id":2, "balance": 4757.80}]' -H 'Content-Type: application/json' http://localhost:8000/accounts/balance/
pub async fn update_balances(payload: web::Json<NewAccountBalancesPayload>) -> impl Responder {
    update_account_balances(&payload)
}

// DELETE
// curl -X DELETE http://localhost:8000/accounts/1
pub async fn delete(req: HttpRequest) -> impl Responder {
    let id: i32 = req
        .match_info()
        .get("id")
        .unwrap()
        .to_string()
        .parse()
        .unwrap();
    delete_account(id)
}
