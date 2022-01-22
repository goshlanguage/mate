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
    vendor: &str,
    id: &str,
    database_id: Option<i32>,
    key: &str,
    secret: &str,
) -> Result<AccountType, &'static str> {
    match vendor.to_lowercase().as_str() {
        "tdameritrade" => {
            let (client_id, client_secret) = get_creds("tdameritrade", key, secret);

            let mut db_id = None;
            if database_id.is_some() {
                let id_option = database_id;
                db_id = id_option;
            }

            Ok(AccountType::TDAmeritradeAccount(TDAmeritradeAccount::new(
                name,
                id,
                client_id.as_str(),
                client_secret.as_str(),
                db_id,
            )))
        }
        "kraken" => {
            let (client_id, client_secret) = get_creds("kraken", key, secret);

            let mut db_id = None;
            if database_id.is_some() {
                let id_option = database_id;
                db_id = id_option;
            }

            Ok(AccountType::KrakenAccount(KrakenAccount::new(
                name,
                id,
                client_id.as_str(),
                client_secret.as_str(),
                db_id,
            )))
        }
        _ => Err("unsupported account type"),
    }
}

/// get_creds returns the given key and secret if they are set, else checks for the existance of the vendor specific environment variables,
///
/// Vendor specific environment might include:
///     tdameritrade: `TDA_CLIENT_ID` and `TDA_REFRESH_TOKEN`
///     kraken: `KRAKEN_API_KEY` and `KRAKEN_API_SECRET`
fn get_creds(vendor: &str, key: &str, secret: &str) -> (String, String) {
    if key.is_empty() || secret.is_empty() {
        let (client_key, client_secret) = match vendor {
            "tdameritrade" => {
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
            "kraken" => {
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
            _ => panic!("Unsupported vendor"),
        };
        return (client_key, client_secret);
    }

    let client_id = key.to_string();
    let client_secret = secret.to_string();
    (client_id, client_secret)
}
