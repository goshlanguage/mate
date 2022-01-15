use actix_web::{HttpRequest, Responder};
use crate::diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use crate::establish_connection;
use crate::models::*;
use super::types::Accounts;


pub async fn get_all() -> impl Responder {
  return_state()
}

pub async fn get(req: HttpRequest) -> impl Responder {
  use crate::schema::accounts::dsl::*;

  let connection = establish_connection();
  let account_name = req.match_info().get("name").unwrap().to_string();

  let result = accounts
      .filter(name.eq(account_name))
      .first(&connection);

  match result {
    Ok(a) => Accounts{ accounts: vec![a]},
    Err(_err) => Accounts{ accounts: Vec::new() },
  }
}

pub fn return_state() -> Accounts {
  use crate::schema::accounts::dsl::*;

  let connection = establish_connection();

  let results = accounts
      .limit(1)
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

//     return return_state()
// }