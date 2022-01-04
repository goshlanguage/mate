pub mod kraken;
use kraken::KrakenAccount;
pub mod tdameritrade;
use tdameritrade::TDAmeritradeAccount;
pub mod traits;
pub mod types;
use types::AccountType;

use std::env;

/// new_account is an account factory
pub fn new_account(
    name: &str,
    ac_type: &str,
    id: &str,
    key: &str,
    secret: &str,
) -> Result<AccountType, &'static str> {
    let mut client_id = String::new();
    let mut client_secret = String::new();

    match ac_type.to_lowercase().as_str() {
        "tdameritrade" => {
            if key.is_empty() || secret.is_empty() {
                let (env_id, env_secret) = get_tdameritrade_creds();
                client_id = env_id;
                client_secret = env_secret;
            }
            Ok(AccountType::TDAmeritradeAccount(TDAmeritradeAccount::new(
                name,
                id,
                client_id.as_str(),
                client_secret.as_str(),
            )))
        }
        "kraken" => {
            if key.is_empty() || secret.is_empty() {
                let (env_id, env_secret) = get_kraken_creds();
                client_id = env_id;
                client_secret = env_secret;
            }
            Ok(AccountType::KrakenAccount(KrakenAccount::new(
                name,
                id,
                client_id.as_str(),
                client_secret.as_str(),
            )))
        }
        _ => Err("unsupported account type"),
    }
}

// get_creds looks for `TDA_CLIENT_ID` and `TDA_REFRESH_TOKEN` environment variables and panics if they aren't present.
fn get_tdameritrade_creds() -> (String, String) {
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

// get_creds looks for `KRAKEN_API_KEY` and `KRAKEN_API_SECRET` environment variables and panics if they aren't present.
fn get_kraken_creds() -> (String, String) {
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
