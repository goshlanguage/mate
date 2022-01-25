use crate::types::{Accounts, NewAccountBalancesPayload};
use log::info;
use reqwest::StatusCode;
mod types;
use types::{AuthPayload, AuthResponse};

pub struct Client {
    pub api_host: String,
    pub auth: AuthResponse,
}

impl Client {
    #[allow(dead_code)]
    pub fn new(api_host: String) -> Client {
        let auth = get_auth().unwrap();

        Client {
            api_host,
            auth,
        }
    }

    // TODO
    // redo this client to use connection pooling in reqwest client
    pub fn get_accounts(self) -> Result<Accounts, StatusCode> {
        let client = reqwest::blocking::Client::new();

        let reqwest_uri = format!("{}/accounts/", self.api_host);
        info!("sending reqwest GET {}", &reqwest_uri);

        let resp = client.get(reqwest_uri)
            .bearer_auth(self.auth.access_token.to_owned())
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
        self,
        balances: NewAccountBalancesPayload,
    ) -> Result<bool, String> {
        let client = reqwest::blocking::Client::new();

        let reqwest_uri = format!("{}/accounts/balance/", self.api_host);
        info!("sending reqwest PUT {}", &reqwest_uri);

        let resp = client.put(reqwest_uri)
            .bearer_auth(self.auth.access_token.to_owned())
            .json(&balances)
            .send()
            .unwrap();

        match resp.status() {
            StatusCode::OK => Ok(true),
            e => Err(e.to_string()),
        }
    }
}

// TODO
// redo this client to use connection pooling in reqwest client
pub fn get_auth() -> Result<AuthResponse, StatusCode> {
    let client = reqwest::blocking::Client::new();

    let authority = std::env::var("AUTHORITY").expect("AUTHORITY must be set");

    let reqwest_uri = format!("{}/oauth/token", authority);
    info!("sending reqwest POST {}", &reqwest_uri);

    let id = std::env::var("MATE_CLIENT_ID").expect("MATE_CLIENT_ID must be set");
    let client_id = std::env::var("MATE_CLIENT_KEY").expect("MATE_CLIENT_KEY must be set");
    let client_secret = std::env::var("MATE_CLIENT_SECRET").expect("MATE_CLIENT_SECRET must be set");
    let payload = AuthPayload{
        audience: id,
        grant_type: "client_credentials".to_string(),
        client_id,
        client_secret,
    };

    let resp = client.post(reqwest_uri)
        .json(&payload)
        .send()
        .unwrap();

    match resp.status() {
        StatusCode::OK => {
            let body = resp.json::<AuthResponse>().unwrap();
            Ok(body)
        }
        e => Err(e),
    }
}
