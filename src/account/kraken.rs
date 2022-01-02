use super::traits::get::Get;
use super::types::*;
use krakenrs::{AssetTickerInfo, KrakenCredentials, KrakenRestAPI, KrakenRestConfig};
use serde_json::to_string;
use std::{convert::TryFrom, env, time::Duration};

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
    pub fn get_pairs(&self, pairs: &str) -> String {
        let data = &self
            .client()
            .ticker(vec![pairs.to_string()])
            .expect("api call failed");

        let close_price = &data.get(pairs).unwrap().c[0];
        close_price.clone()
    }
}

// get_creds looks for `KRAKEN_API_KEY` and `KRAKEN_API_SECRET` environment variables and panics if they aren't present.
pub fn get_kraken_creds() -> (String, String) {
    let client_key = match env::var("KRAKEN_API_KEY") {
        Ok(val) => val,
        Err(e) => panic!(
            "Didn't find the KRAKEN_API_KEY env var, please set this and try again. {}",
            e
        ),
    };

    let client_secret = match env::var("KRAKEN_API_SECRET") {
        Ok(val) => val,
        Err(e) => panic!(
            "Didn't find the KRAKEN_API_SECRET env var, please set this and try again. {}",
            e
        ),
    };

    (client_key, client_secret)
}

impl Get for KrakenAccount {}
