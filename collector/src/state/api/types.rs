use chrono::prelude::*;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Auth {
    pub access_token: String,
    pub expiry: chrono::NaiveDateTime,
}

impl Auth {
    // the new factory. We intentionally set an expiry of 0 epoch so that its invalid from the start
    // it is intended for the consumer of this object to fetch its access_token through
    // the get_token helper, which checks to see if the current access_token is expired,
    // and renews it if it is.
    pub fn new() -> Auth {
        Auth {
            access_token: "".to_string(),
            expiry: NaiveDateTime::from_timestamp(0, 0),
        }
    }

    #[allow(dead_code)]
    pub fn from_response(response: AuthResponse) -> Auth {
        let now_epoch = chrono::Utc::now().timestamp();
        let expiry = NaiveDateTime::from_timestamp(now_epoch + response.expires_in, 0);

        Auth {
            access_token: response.access_token,
            expiry,
        }
    }

    // get_token is a helper that returns an Auth's access_token if not expired,
    //   otherwise renews the token and returns the new auth token
    pub fn get_token(&mut self) -> String {
        info!("fetching token");
        if chrono::Utc::now().timestamp() < self.expiry.timestamp() {
            info!("token is valid, reusing");
            return self.access_token.clone();
        }

        info!("token is invalid. Token expiration: {}", self.expiry);
        self.renew_token();
        info!("Expiry after renewal: {}", self.expiry);
        self.access_token.clone()
    }

    pub fn renew_token(&mut self) -> String {
        let client = reqwest::blocking::Client::new();

        let authority = get_authority();

        let reqwest_uri = format!("{}/oauth/token", authority);
        info!("sending reqwest POST {}", &reqwest_uri);

        let id = std::env::var("MATE_CLIENT_ID").expect("MATE_CLIENT_ID must be set");
        let client_id = std::env::var("MATE_CLIENT_KEY").expect("MATE_CLIENT_KEY must be set");
        let client_secret =
            std::env::var("MATE_CLIENT_SECRET").expect("MATE_CLIENT_SECRET must be set");

        let payload = AuthPayload {
            audience: id,
            grant_type: "client_credentials".to_string(),
            client_id,
            client_secret,
        };

        let resp = client.post(reqwest_uri).json(&payload).send().unwrap();

        let body = resp.json::<AuthResponse>().unwrap();
        let now_epoch = chrono::Utc::now().timestamp();
        let expiry = NaiveDateTime::from_timestamp(now_epoch + body.expires_in, 0);

        info!(
            "New token expires on: {}",
            NaiveDateTime::from_timestamp(now_epoch + body.expires_in, 0)
        );

        self.access_token = body.access_token.clone();
        self.expiry = expiry;
        body.access_token
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponse {
    #[serde(rename = "access_token")]
    pub access_token: String,
    #[serde(rename = "token_type")]
    pub token_type: String,
    #[serde(rename = "expires_in")]
    pub expires_in: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthPayload {
    pub audience: String,
    #[serde(rename = "grant_type")]
    pub grant_type: String,
    #[serde(rename = "client_id")]
    pub client_id: String,
    #[serde(rename = "client_secret")]
    pub client_secret: String,
}

pub fn get_authority() -> String {
    let mut authority = std::env::var("AUTHORITY").expect("AUTHORITY must be set");

    // ensure the host string doesn't end in / or normalize the string
    let last_char = &authority.to_string().pop().unwrap();

    let slash = "/".chars().next().unwrap();

    if last_char == &slash {
        authority.truncate(authority.len() - 1);
    }
    authority
}
