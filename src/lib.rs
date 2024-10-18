use apis::account::AccountOperationsClient;
use apis::auth::AuthOperationsClient;
use apis::data_model::{self, DataModelOperationsClient};
use std::str::FromStr;
use surf::http::headers::{HeaderName, HeaderValue};
use surf::{Client, Config};

pub mod apis;
pub mod models;
mod services;
mod utils;

pub const BASE_URL: &str = "https://dev.api.gateway.tech";

pub struct GtwSDK {
    client: Client,
    pub auth: AuthOperationsClient,
    pub account: AccountOperationsClient,
    pub data_model: DataModelOperationsClient,
}

impl GtwSDK {
    pub async fn new(bearer_token: Option<String>) -> Result<Self, surf::Error> {
        let mut config = Config::new().set_base_url(surf::Url::parse(BASE_URL).unwrap());

        if let Some(bearer) = bearer_token {
            config = config.add_header(
                HeaderName::from_string("authorization".to_string()).unwrap(),
                HeaderValue::from_str(&format!("Bearer {}", bearer)).unwrap(),
            )?;
        }

        config = config.add_header(
            HeaderName::from_string("content-type".to_string()).unwrap(),
            HeaderValue::from_str("application/json").unwrap(),
        )?;

        let client: Client = config.try_into()?;

        let account = AccountOperationsClient::new(client.clone());
        let auth = AuthOperationsClient::new(client.clone());
        let data_model = DataModelOperationsClient::new(client.clone());

        Ok(GtwSDK {
            client,
            account,
            auth,
            data_model,
        })
    }
}
