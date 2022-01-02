use super::traits::get::Get;
use super::types::*;
use log::info;
use std::env;
use tda_sdk::{
    params::{GetAccountsParams, GetPriceHistoryParams},
    responses::{Candle, SecuritiesAccount},
    Client,
};

/// # TDAmeritradeAccount
///  TDAmeritradeAccount represents a brokerage account
/// ```rust
/// let account = TDAmeritradeAccount::new(name, account_id, client_id, refresh_token);
/// ```
#[derive(Clone)]
pub struct TDAmeritradeAccount {
    pub account_id: String,
    pub account: Account,
    pub client_id: String,
    pub refresh_token: String,
    pub active: bool,
}

impl TDAmeritradeAccount {
    pub fn new(
        name: &str,
        account_id: &str,
        client_id: &str,
        refresh_token: &str,
    ) -> TDAmeritradeAccount {
        let mut client = Client::new(client_id, refresh_token, None);

        let response = client.get_access_token().unwrap();
        client.set_access_token(&Some(response.into()));

        TDAmeritradeAccount {
            account_id: account_id.to_string(),
            active: true,
            account: Account::new(name),
            client_id: client_id.to_string(),
            refresh_token: refresh_token.to_string(),
        }
    }

    pub fn client(&self) -> Client {
        let mut client = Client::new(&self.client_id, &self.refresh_token, None);
        let response = client.get_access_token().unwrap();
        client.set_access_token(&Some(response.into()));
        client
    }

    pub fn get_account_ids(&self) -> String {
        info!("Returning account ids");
        let accounts = self
            .client()
            .get_accounts(GetAccountsParams::default())
            .unwrap();

        let mut ids: Vec<String> = Vec::new();
        for account in accounts {
            match account.securities_account {
                SecuritiesAccount::MarginAccount { account_id, .. } => {
                    ids.push(account_id);
                }
            }
        }
        ids.join(", ")
    }

    /// get_candles is responsible for fetching any new candles as necessary
    /// As is the case for EMA 20 and higher, we default to importing 3 years of daily data
    /// to be able to calculate a more precise EMA.
    pub fn get_candles(&mut self, symbol: String) -> Vec<Candle> {
        // https://developer.tdameritrade.com/price-history/apis/get/marketdata/%7Bsymbol%7D/pricehistory
        let params = GetPriceHistoryParams {
            end_date: None,
            frequency_type: Some(String::from("daily")),
            frequency: Some(String::from("1")),
            need_extended_hours_data: None,
            period_type: Some(String::from("year")),
            period: Some(String::from("3")),
            start_date: None,
        };

        let history = self.client().get_price_history(symbol.as_str(), params);

        let resp = match history {
            Ok(val) => val,
            Err(e) => {
                info!("Failed to get price history: {}", e.to_string());
                return Vec::new();
            }
        };

        resp.candles
    }
}

// get_creds looks for `TDA_CLIENT_ID` and `TDA_REFRESH_TOKEN` environment variables and panics if they aren't present.
pub fn get_tdameritrade_creds() -> (String, String) {
    let client_id = match env::var("TDA_CLIENT_ID") {
        Ok(val) => val,
        Err(e) => panic!(
            "Didn't find the TDA_CLIENT_ID env var, please set this and try again. {}",
            e
        ),
    };

    let refresh_token = match env::var("TDA_REFRESH_TOKEN") {
        Ok(val) => val,
        Err(e) => panic!(
            "Didn't find the TDA_REFRESH_TOKEN env var, please set this and try again. {}",
            e
        ),
    };

    (client_id, refresh_token)
}

impl Get for TDAmeritradeAccount {}
