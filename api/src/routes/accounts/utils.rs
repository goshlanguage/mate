use crate::diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use crate::establish_connection;
use crate::models::*;
use log::{error, info};
use magic_crypt::MagicCryptTrait;
use std::env;

// CREATE
/// Creates an account and returns the stored model
pub fn create_account(new: &NewAccountPayload) -> Account {
    use crate::schema::accounts;

    let connection = establish_connection();

    let now = &chrono::Utc::now().naive_utc();

    let encrypted_secret = encrypt(new.client_secret.to_string());

    let new_account = NewAccount {
        name: &new.name,
        vendor: &new.vendor,
        client_key: &new.client_key,
        client_secret: encrypted_secret.as_str(),
        created: now,
        updated: now,
    };

    // TODO
    // catch and return error here perhaps
    diesel::insert_into(accounts::table)
        .values(new_account)
        .get_result::<Account>(&connection)
        .expect("Error saving new account")
}

// READ Account Summaries
/// Returns an AccountSummary associated with the given id or a blank array for no account found
/// Return serializes to:
/// {"accounts":[]}
pub fn get_summaries() -> AccountSummaries {
    let mut result = Vec::new();
    for account in get_accounts().accounts {
        let balance = get_balance_by_id(account.id).balances[0].balance;

        result.push(AccountSummary {
            name: account.name,
            balance,
        });
    }
    AccountSummaries { accounts: result }
}

// READ ID
/// Returns accounts associated with the given id or a blank array for no account found
/// Return serializes to:
/// {"accounts":[]}
pub fn get_account(account_id: i32) -> Accounts {
    use crate::schema::accounts::dsl::*;

    let connection = establish_connection();

    let result = accounts.filter(id.eq(account_id)).first(&connection);

    match result {
        Ok(a) => Accounts { accounts: vec![a] },
        Err(_err) => Accounts {
            accounts: Vec::new(),
        },
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
        Ok(b) => Balances { balances: vec![b] },
        Err(_err) => Balances {
            balances: Vec::new(),
        },
    }
}

// READ Account ALL
/// Returns all accounts stored in the database
pub fn get_accounts() -> Accounts {
    use crate::schema::accounts::dsl::*;

    let connection = establish_connection();

    let results = accounts.load::<Account>(&connection).unwrap();

    let mut array_buffer = Vec::new();

    // TODO
    // Refactor this into some sort of map/collect statement that does the same thing
    for account in results {
        let account = Account {
            id: account.id,
            name: account.name,
            vendor: account.vendor,
            client_key: account.client_key,
            client_secret: account.client_secret,
            created: account.created,
            updated: account.updated,
        };
        array_buffer.push(account);
    }
    Accounts {
        accounts: array_buffer,
    }
}

// READ Balance ALL
/// Returns all account balances stored in the database
pub fn get_balances() -> Balances {
    use crate::schema::account_histories::dsl::*;

    let connection = establish_connection();

    let results = account_histories.load::<Balance>(&connection).unwrap();

    Balances { balances: results }
}

// Update
/// Update an account and returns the stored model
pub fn update_account(target: &UpdateAccountPayload) -> Account {
    use crate::schema::accounts::dsl::*;

    let connection = establish_connection();

    let encrypted_secret = encrypt(target.client_secret.to_string());

    // catch and return error here perhaps
    diesel::update(accounts.filter(id.eq(target.id)))
        .set((
            name.eq(&target.name),
            vendor.eq(&target.vendor),
            client_key.eq(&target.client_key),
            client_secret.eq(&encrypted_secret.as_str()),
            updated.eq(Some(chrono::Utc::now().naive_utc())),
        ))
        .get_result::<Account>(&connection)
        .expect("Account update failed, id")
}

// PUT
/// Creates account balances if they don't exist and returns the stored models
pub fn update_account_balances(new: &NewAccountBalancesPayload) -> Balances {
    info!("NewAccountBalancesPayload: {:?}", new);
    use crate::schema::account_histories;

    let connection = establish_connection();
    let updated = &chrono::Utc::now().naive_utc();

    let mut balances = Vec::new();

    for newb in &new.balances {
        let new_balance = NewAccountBalance {
            account_id: &newb.account_id,
            balance: &newb.balance,
            updated,
        };

        // TODO
        // catch and return error here perhaps
        balances.push(
            diesel::insert_into(account_histories::table)
                .values(new_balance)
                .get_result::<Balance>(&connection)
                .expect("Error saving new account balance"),
        );
    }

    Balances { balances }
}

// DELETE
/// deletes a given account by id
pub fn delete_account(rm_id: i32) -> Accounts {
    use crate::schema::accounts::dsl::*;

    let connection = establish_connection();

    match diesel::delete(accounts.filter(id.eq(rm_id))).execute(&connection) {
        Ok(_) => (),
        Err(_err) => {
            // TODO return the error to the client
            error!("Error deleting account {}", rm_id);
        }
    }

    get_accounts()
}

fn encrypt(input: String) -> String {
    let salt = match env::var("MATE_SALT") {
        Ok(val) => val,
        Err(e) => panic!(
            "Didn't find the MATE_SALT env var, please set this and try again. {}",
            e
        ),
    };

    let mc = new_magic_crypt!(salt, 256);

    mc.encrypt_str_to_base64(input.as_str())
}
