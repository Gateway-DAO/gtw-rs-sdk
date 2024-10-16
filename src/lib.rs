use apis::{
    account::{AccountOperations, AccountOperationsClient},
    auth::{AuthOperations, AuthOperationsClient},
    data_asset::{DataAssestOperation, DataAssestOperationsClient},
    data_model::{DataModelOperation, DataModelOperationsClient},
};
use reqwest::Client;
pub mod apis;
pub mod models;
mod utils;

pub const BASE_URL: &str = "https://dev.api.gateway.tech";

pub struct GtwSDK {
    #[allow(dead_code)]
    client: Client,
    pub account: Box<dyn AccountOperations>,
    pub auth: Box<dyn AuthOperations>,
    pub data_model: Box<dyn DataModelOperation>,
    pub data_assest: Box<dyn DataAssestOperation>,
}

impl GtwSDK {
    pub fn new(bearer_token: Option<String>) -> Result<Self, reqwest::Error> {
        let mut headers = reqwest::header::HeaderMap::new();
        if let Some(bearer_token) = bearer_token {
            headers.insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", bearer_token)
                    .parse()
                    .expect("Failed to parse authorization header"),
            );
        }
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json"
                .parse()
                .expect("Failed to parse content type header"),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        let account = Box::new(AccountOperationsClient::new(client.clone())); // Box the clients
        let auth = Box::new(AuthOperationsClient::new(client.clone())); // Box the clients
        let data_model = Box::new(DataModelOperationsClient::new(client.clone())); // Box the clients

        let data_assest = Box::new(DataAssestOperationsClient::new(client.clone()));

        Ok(GtwSDK {
            client,
            account,
            auth,
            data_model,
            data_assest,
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
