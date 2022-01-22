use crate::types::{Accounts, NewAccountBalancesPayload};
use log::info;
use reqwest::StatusCode;

pub struct Client {
    pub api_host: String,
}

impl Client {
    #[allow(dead_code)]
    pub fn new(api_host: String) -> Client {
        Client { api_host }
    }

    // TODO
    // redo this client to use connection pooling in reqwest client
    pub fn get_accounts(self) -> Result<Accounts, StatusCode> {
        let reqwest_uri = format!("{}/accounts/", self.api_host);
        info!("sending reqwest GET {}", &reqwest_uri);

        let resp = reqwest::blocking::get(reqwest_uri).unwrap();

        match resp.status() {
            StatusCode::OK => {
                let body = resp.json::<Accounts>().unwrap();
                Ok(body)
            }
            e => Err(e),
        }
    }

    pub fn submit_account_balances(
        self,
        balances: NewAccountBalancesPayload,
    ) -> Result<bool, String> {
        let client = reqwest::blocking::Client::new();

        let reqwest_uri = format!("{}/accounts/balance/", self.api_host);
        info!("sending reqwest PUT {}", &reqwest_uri);

        let resp = client.put(reqwest_uri).json(&balances).send().unwrap();

        match resp.status() {
            StatusCode::OK => Ok(true),
            e => Err(e.to_string()),
        }
    }
}
