use apis::{account::AccountOperationsClient, auth::AuthOperationsClient};
use reqwest::Client;
pub mod apis;
pub mod models;
mod utils;
pub const BASE_URL: &str = "https://dev.api.gateway.tech";

pub struct GtwSDK {
    client: Client,
    pub account: AccountOperationsClient,
    pub auth: AuthOperationsClient,
}

impl GtwSDK {
    pub fn new(bearer_token: Option<String>) -> Result<Self, reqwest::Error> {
        let mut headers = reqwest::header::HeaderMap::new();
        if let Some(bearer_token) = bearer_token {
            headers.insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", bearer_token).parse().unwrap(),
            );
        }
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        let account = AccountOperationsClient::new(client.clone());
        let auth = AuthOperationsClient::new(client.clone());

        Ok(GtwSDK {
            client,
            account,
            auth,
        })
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn create_client_without_header() {
//         let gtw_client = GtwSDK::new(None);
//         assert_eq!(gtw_client.is_err(), false);
//     }
//     #[test]
//     fn create_client_with_header() {
//         let bearer_token = "Bearer token".to_string();
//         let gtw_client = GtwSDK::new(Some(bearer_token.clone()));

//         assert_eq!(gtw_client.is_err(), false);
//     }
// }
