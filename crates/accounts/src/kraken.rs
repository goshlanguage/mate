use super::traits::get::Get;
use super::types::*;
use krakenrs::{KrakenCredentials, KrakenRestAPI, KrakenRestConfig};
use serde_json::{json, Map, Value};
use std::{convert::TryFrom, time::Duration};

/// # KrakenAccount
///  KrakenAccount represents an exchange account
/// ```rust
/// let account = KrakenAccount::new(name, account_id, client_key, client_secret);
/// ```
#[derive(Clone)]
pub struct KrakenAccount {
    pub account_id: String,
    pub account: Account,
    pub client_key: String,
    pub client_secret: String,
    pub active: bool,
}

impl KrakenAccount {
    pub fn new(
        name: &str,
        account_id: &str,
        client_key: &str,
        client_secret: &str,
    ) -> KrakenAccount {
        let creds = KrakenCredentials {
            key: client_key.to_string(),
            secret: client_secret.to_string(),
        };

        let conf = KrakenRestConfig {
            creds: creds,
            timeout: Duration::new(30, 0),
        };

        KrakenRestAPI::try_from(conf).expect("could not connect to Kraken");

        KrakenAccount {
            account_id: account_id.to_string(),
            active: true,
            account: Account::new(name),
            client_key: client_key.to_string(),
            client_secret: client_secret.to_string(),
        }
    }

    pub fn client(&self) -> KrakenRestAPI {
        let creds = KrakenCredentials {
            key: self.client_key.to_string(),
            secret: self.client_secret.to_string(),
        };

        let conf = KrakenRestConfig {
            creds: creds,
            timeout: Duration::new(30, 0),
        };

        KrakenRestAPI::try_from(conf).expect("could not connect to Kraken")
    }

    #[allow(dead_code)]
    pub fn get_account_balances(&self) -> String {
        let mut balance = String::new();
        let accounts = self.client().get_account_balance().unwrap();

        for account in accounts.keys() {
            balance = format!(
                "{}{}:{:?}\n",
                balance,
                account,
                accounts.get(account).unwrap()
            );
        }
        balance
    }

    /// when invoking get_pairs, be sure to use the API format that Kraken expects, or you may experience a failure when unwrapping at current
    /// If you call get_pairs with something that will return a valid response, but the index is different, we'll erroneously try to fetch the wrong index
    /// thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/account/kraken.rs:84:44
    ///
    /// ETHUSD is valid, but the appopriate pair is XETHZUSD
    /// Format: X <Crypto> Z <Currency Pairing>
    ///
    #[allow(dead_code)]
    pub fn get_pairs(&self, pairs: &str) -> String {
        let data = &self
            .client()
            .ticker(vec![pairs.to_string()])
            .expect("api call failed");

        let close_price = &data.get(pairs).unwrap().c[0];
        close_price.clone()
    }

    pub fn get_ticks(&self, pairs: Vec<String>) -> Result<Map<String, Value>, &'static str> {
        let mut data: Map<String, Value> = Map::new();

        let api_result = &self.client().ticker(pairs);
        match api_result {
            Ok(api_data) => {
                for pair in api_data.keys().cloned() {
                    let tick_data = api_data.get(&pair).unwrap();
                    let value = json!(tick_data);

                    data.insert(pair, value);
                }

                Ok(data)
            }
            Err(_e) => Err("Failed to collect pairs, review your pairs and ensure they match https://api.kraken.com/0/public/Assets"),
        }
    }
}

impl Get for KrakenAccount {}
