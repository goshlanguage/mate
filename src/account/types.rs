use super::kraken::KrakenAccount;
use super::tdameritrade::TDAmeritradeAccount;
use super::traits::get::Get;

/// AccountType enum differentiates between what kind of account is being managed.
#[derive(Clone)]
pub enum AccountType {
    KrakenAccount(KrakenAccount),
    TDAmeritradeAccount(TDAmeritradeAccount),
}

#[derive(Clone)]
pub struct Account {
    pub name: String,
}

impl Account {
    pub fn new(name: &str) -> Account {
        Account {
            name: name.to_string(),
        }
    }
}

impl Get for AccountType {}
