use actix_web::{HttpRequest, Responder, web};
use log::info;

use crate::diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use crate::establish_connection;
use crate::models::*;
use super::types::Accounts;

// curl http://localhost:8000/accounts/
pub async fn get_all() -> impl Responder {
  get_state()
}

// curl http://localhost:8000/accounts/tdameritrade
pub async fn get(req: HttpRequest) -> impl Responder {
  let account_name = req.match_info().get("name").unwrap().to_string();
  get_account(account_name)
}

// curl -i -X POST -d '{"name":"tdameritrade", "balance": 475.78}' -H 'Content-Type: application/json' http://localhost:8000/accounts/
pub async fn post(payload: web::Json<NewAccountPayload>) -> impl Responder {
  info!("Received post for {}", payload.name);
  create_account(&payload)
}

pub fn get_state() -> Accounts {
  use crate::schema::accounts::dsl::*;

  let connection = establish_connection();

  let results = accounts
  .load::<Account>(&connection)
  .unwrap();

  let mut array_buffer = Vec::new();

  for account in results {
    let account = Account{
      id: account.id,
      name: account.name,
      balance: account.balance,
      balance_history: account.balance_history,
    };
    array_buffer.push(account);
  }
  Accounts{ accounts: array_buffer }
}

pub fn create_account(new: &NewAccountPayload) -> Account {
  use crate::schema::accounts;

  let connection = establish_connection();

  let new_account = NewAccount {
    name: &new.name,
    balance: &new.balance,
    balance_history: &vec![ new.balance ],
  };

  // TODO
  // catch and return error here perhaps
  let result = diesel::insert_into(accounts::table)
    .values(new_account)
    .get_result::<Account>(&connection)
    .expect("Error saving new account");

  return result
}

pub fn get_account(account_name: String) -> Accounts {
  use crate::schema::accounts::dsl::*;

  let connection = establish_connection();

  let result = accounts
      .filter(name.eq(account_name))
      .first(&connection);

  match result {
    Ok(a) => Accounts{ accounts: vec![a]},
    Err(_err) => Accounts{ accounts: Vec::new() },
  }
}

// pub async fn post(req: HttpRequest) {
//   let name: String = req.match_info()
//                       .get("name")
//                       .unwrap()
//                       .to_string();

//     let balance: f64 = req.match_info()
//         .get("balance")
//         .unwrap()
//         .to_string()
//         .parse();

//     let balance_history = vec![balance];

//     let connection = establish_connection();
//     let accounts = accounts::table
//         .filter(accounts::columns::name.eq(name.as_str()))
//         .order(accounts::columns::id.asc())
//         .load::<models::Account>(&connection)
//         .unwrap();

//     if accounts.len() == 0 {
//         let new_account = NewAccount::new(name, balance);
//         let _ = diesel::insert_into(accounts::table).values(&new_account)
//             .execute(&connection);
//     }

//     return get_state()
// }