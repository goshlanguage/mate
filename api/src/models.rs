use super::schema::*;
use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};

// Query Structs
/// Account models what the database contains, and should map closely to the schema,
/// ensuring that types represent their counterparts
/// <https://kotiri.com/2018/01/31/postgresql-diesel-rust-types.html>
#[derive(Clone, Deserialize, Queryable, Serialize)]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub vendor: String,
    pub client_key: String,
    pub client_secret: String,
    pub created: chrono::NaiveDateTime,
    pub updated: Option<chrono::NaiveDateTime>,
}

/// Responder is an actix_web Trait that generates responses
/// https://docs.rs/actix-web/0.4.5/actix_web/trait.Responder.html
impl Responder for Account {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

#[derive(Serialize)]
pub struct AccountSummary {
    pub name: String,
    pub balance: f64,
}

/// Responder is an actix_web Trait that generates responses
/// https://docs.rs/actix-web/0.4.5/actix_web/trait.Responder.html
impl Responder for AccountSummary {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

#[derive(Serialize)]
pub struct AccountSummaries {
    pub accounts: Vec<AccountSummary>,
}

/// Responder is an actix_web Trait that generates responses
/// https://docs.rs/actix-web/0.4.5/actix_web/trait.Responder.html
impl Responder for AccountSummaries {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

#[derive(Clone, Deserialize, Queryable, Serialize)]
pub struct Balance {
    pub id: i32,
    pub account_id: i32,
    pub balance: f64,
    pub updated: chrono::NaiveDateTime,
}

/// Responder is an actix_web Trait that generates responses
/// https://docs.rs/actix-web/0.4.5/actix_web/trait.Responder.html
impl Responder for Balance {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

#[derive(Clone, Deserialize, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created: chrono::NaiveDateTime,
}

impl Responder for User {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

// Payload Structs
/// NewAccountPayload structures what NewAccount request JSON should look like.
/// A valid request for this object would look like:
/// '{"name":"tdameritrade", "vendor": "tdameritrade", "client_key": "", "client_secret": ""}'
#[derive(Deserialize)]
pub struct NewAccountPayload {
    pub name: String,
    pub vendor: String,
    pub client_key: String,
    pub client_secret: String,
}

#[derive(Deserialize)]
pub struct UpdateAccountPayload {
    pub id: i32,
    pub name: String,
    pub vendor: String,
    pub client_key: String,
    pub client_secret: String,
}

#[derive(Debug, Deserialize)]
pub struct NewAccountBalancePayload {
    pub account_id: i32,
    pub balance: f64,
}

#[derive(Debug, Deserialize)]
pub struct NewAccountBalancesPayload {
    pub balances: Vec<NewAccountBalancePayload>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUserPayload {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

// Insertable structs
/// NewAccount represents an insertable model of Account
#[derive(Insertable)]
#[table_name = "accounts"]
pub struct NewAccount<'a> {
    pub name: &'a str,
    pub vendor: &'a str,
    pub client_key: &'a str,
    pub client_secret: &'a str,
    pub created: &'a chrono::NaiveDateTime,
    pub updated: &'a chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "account_histories"]
pub struct NewAccountBalance<'a> {
    pub account_id: &'a i32,
    pub balance: &'a f64,
    pub updated: &'a chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub created: chrono::NaiveDateTime,
}

// Wrapper types
/// Accounts lets us return an array for the get method, and other methods to
/// represent results of searches, where an empty array represents no matches found.
#[derive(Serialize)]
pub struct Accounts {
    pub accounts: Vec<Account>,
}

impl Responder for Accounts {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

/// Balances lets us return an array for the get method, and other methods to
/// represent results of searches, where an empty array represents no matches found.
#[derive(Serialize)]
pub struct Balances {
    pub balances: Vec<Balance>,
}

impl Responder for Balances {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}
