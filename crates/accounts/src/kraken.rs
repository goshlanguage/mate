use super::traits::get::Get;
use super::types::*;
use krakenrs::{KrakenCredentials, KrakenRestAPI, KrakenRestConfig};
use log::info;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
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
    pub database_id: Option<i32>,
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
        database_id: Option<i32>,
    ) -> KrakenAccount {
        let creds = KrakenCredentials {
            key: client_key.to_string(),
            secret: client_secret.to_string(),
        };

        let conf = KrakenRestConfig {
            creds,
            timeout: Duration::new(30, 0),
        };

        KrakenRestAPI::try_from(conf).expect("could not connect to Kraken");

        let mut db_id = None;
        if let Some(..) = database_id {
            let id = database_id.unwrap();
            db_id = Some(id);
        }

        KrakenAccount {
            account_id: account_id.to_string(),
            active: true,
            account: Account::new(name),
            client_key: client_key.to_string(),
            client_secret: client_secret.to_string(),
            database_id: db_id,
        }
    }

    pub fn client(&self) -> KrakenRestAPI {
        let creds = KrakenCredentials {
            key: self.client_key.to_string(),
            secret: self.client_secret.to_string(),
        };

        let conf = KrakenRestConfig {
            creds,
            timeout: Duration::new(30, 0),
        };

        KrakenRestAPI::try_from(conf).expect("could not connect to Kraken")
    }

    pub fn get_account_balance(&self) -> Decimal {
        let mut balance = dec!(0.0);
        let balances = self.client().get_account_balance().unwrap();

        info!("Found account balances for: {:?}", balances.keys());
        for key in balances.keys() {
            // Staked accounts end in .S
            // These accounts are not valid to Kraken, so we need to split
            // the string to get the asset type
            let asset_pair = if key.contains('.') {
                let split = &key.split('.').collect::<Vec<&str>>();
                split[0].to_string().to_owned()
            } else {
                key.to_string()
            };

            let asset_pair = match asset_pair.as_str() {
                "ZUSD" => {
                    balance += balances.get(key).unwrap();
                    continue;
                }
                "ATOM" => "ATOMUSD".to_string(),
                "XXDG" => "XDGUSD".to_string(),
                _ => format!("{}ZUSD", asset_pair),
            };

            let spot_price = &self.get_spot_price(asset_pair.clone());
            info!("Spot price for {}: {:?}", asset_pair, spot_price);

            balance += spot_price * balances.get(key).unwrap();
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

    pub fn get_ticks(&self, pairs: Vec<String>) -> Result<Map<String, Value>, String> {
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
            Err(_e) => Err(_e.to_string()),
        }
    }

    pub fn get_spot_price(&self, asset_pair: String) -> Decimal {
        // Fetch TickerResponse
        // <https://github.com/garbageslam/krakenrs/blob/v5.2.2/src/messages.rs#L142>
        let ticker_data = self.client().ticker(vec![asset_pair.to_string()]).unwrap();

        // Get our asset's value from the returned hashmap
        let asset_price = ticker_data.get(&asset_pair).unwrap();
        let close_price = asset_price.c.first().unwrap();

        // See the AssetTickerInfo model here:
        // <https://github.com/garbageslam/krakenrs/blob/v5.2.2/src/messages.rs#L130-L139>
        Decimal::from_str(close_price).unwrap()
    }
}

impl Get for KrakenAccount {}
