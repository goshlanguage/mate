use crate::types::{Accounts, NewAccountBalancesPayload};
use log::info;
use reqwest::StatusCode;
pub mod types;
use types::*;
mod tests;

#[derive(Clone)]
pub struct Client {
    pub api_host: String,
    pub auth: Auth,
}

impl Client {
    #[allow(dead_code)]
    pub fn new(api_host: String) -> Client {
        let auth = Auth::new();

        Client { api_host, auth }
    }

    // TODO
    // redo this client to use connection pooling in reqwest client
    pub fn get_accounts(mut self) -> Result<Accounts, StatusCode> {
        let client = reqwest::blocking::Client::new();

        let reqwest_uri = format!("{}/accounts/", self.api_host);
        info!("sending reqwest GET {}", &reqwest_uri);

        let resp = client
            .get(reqwest_uri)
            .bearer_auth(self.auth.get_token())
            .send()
            .unwrap();

        match resp.status() {
            StatusCode::OK => {
                let body = resp.json::<Accounts>().unwrap();
                Ok(body)
            }
            e => Err(e),
        }
    }

    pub fn submit_account_balances(
        mut self,
        balances: NewAccountBalancesPayload,
    ) -> Result<bool, String> {
        let client = reqwest::blocking::Client::new();

        let reqwest_uri = format!("{}/accounts/balance/", self.api_host);
        info!("sending reqwest PUT {}", &reqwest_uri);

        let resp = client
            .put(reqwest_uri)
            .bearer_auth(self.auth.get_token())
            .json(&balances)
            .send()
            .unwrap();

        match resp.status() {
            StatusCode::OK => Ok(true),
            e => Err(e.to_string()),
        }
    }
}
