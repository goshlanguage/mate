use crate::diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use crate::establish_connection;
use crate::models::*;

// CREATE
/// Creates an account and returns the stored model
pub fn create_account(new: &NewAccountPayload) -> Account {
  use crate::schema::accounts;

  let connection = establish_connection();

  // TODO
  // only store these values encrypted
  let new_account = NewAccount {
    name: &new.name,
    vendor: &new.vendor,
    client_key: &new.client_key,
    client_secret: &new.client_secret,
    created: &chrono::Utc::now().naive_utc(),
  };

  // TODO
  // catch and return error here perhaps
  let mut result = diesel::insert_into(accounts::table)
  .values(new_account)
  .get_result::<Account>(&connection)
  .expect("Error saving new account");

  result.client_secret = "REDACTED".to_string();
  return result
}

// CREATE
/// Creates an account and returns the stored model
pub fn create_account_balance(new: &NewAccountBalancePayload) -> Balance {
  use crate::schema::account_histories;

  let connection = establish_connection();

  let new_balance = NewAccountBalance {
    account_id: &new.account_id,
    balance: &new.balance,
    updated: &chrono::Utc::now().naive_utc(),
  };

  // TODO
  // catch and return error here perhaps
  let result = diesel::insert_into(account_histories::table)
    .values(new_balance)
    .get_result::<Balance>(&connection)
    .expect("Error saving new account balance");

  return result
}

// READ Account Summaries
/// Returns an AccountSummary associated with the given id or a blank array for no account found
/// Return serializes to:
/// {"accounts":[]}
pub fn get_summaries() -> AccountSummaries {
  let mut result = Vec::new();
  for account in get_accounts().accounts {
    let balance = get_balance_by_id(account.id).balances[0].balance;

    result.push(AccountSummary{
      name: account.name,
      balance,
    });
  }
  AccountSummaries{ accounts: result}
}

// READ ID
/// Returns accounts associated with the given id or a blank array for no account found
/// Return serializes to:
/// {"accounts":[]}
pub fn get_account(account_id: i32) -> Accounts {
  use crate::schema::accounts::dsl::*;

  let connection = establish_connection();

  let result = accounts
      .filter(id.eq(account_id))
      .first(&connection);

  match result {
    Ok(a) => Accounts{ accounts: vec![a]},
    Err(_err) => Accounts{ accounts: Vec::new() },
  }
}

// READ Balance ID
/// Returns account balances associated with the given id or a blank array for no account history found
/// Return serializes to:
/// {"balances":[]}
pub fn get_balance_by_id(target_account_id: i32) -> Balances {
  use crate::schema::account_histories::dsl::*;

  let connection = establish_connection();

  let result = account_histories
      .filter(account_id.eq(target_account_id))
      .order(updated.desc())
      .first(&connection);

  match result {
    Ok(b) => Balances{ balances: vec![b]},
    Err(_err) => Balances{ balances: Vec::new() },
  }
}

// READ Account ALL
/// Returns all accounts stored in the database
pub fn get_accounts() -> Accounts {
  use crate::schema::accounts::dsl::*;

  let connection = establish_connection();

  let results = accounts
  .load::<Account>(&connection)
  .unwrap();

  let mut array_buffer = Vec::new();

  // TODO
  // Refactor this into some sort of map/collect statement that does the same thing
  for account in results {
    let account = Account{
      id: account.id,
      name: account.name,
      vendor: account.vendor,
      client_key: "REDACTED".to_string(),
      client_secret: "REDACTED".to_string(),
      created: account.created,
      updated: account.updated,
    };
    array_buffer.push(account);
  }
  Accounts{ accounts: array_buffer }
}

// READ Balance ALL
/// Returns all account balances stored in the database
pub fn get_balances() -> Balances {
  use crate::schema::account_histories::dsl::*;

  let connection = establish_connection();

  let results = account_histories
    .load::<Balance>(&connection)
    .unwrap();

  Balances{ balances: results }
}

// DELETE
/// deletes a given account by id
pub fn delete_account(rm_id: i32) -> Accounts {
  use crate::schema::accounts::dsl::*;

  let connection = establish_connection();

  // TODO
  // catch and return error here perhaps
  // diesel::delete(
  //   account_histories.filter(id.eq(rm_id)))
  //   .execute(&connection)
  //   .expect(format!("Error delete account histories for {}", rm_id).as_str());

  diesel::delete(
    accounts.filter(id.eq(rm_id)))
    .execute(&connection)
    .expect(format!("Error delete account {}", rm_id).as_str());

  return get_accounts()
}
