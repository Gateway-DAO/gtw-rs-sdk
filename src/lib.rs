use apis::account::AccountOperationsClient;
use apis::auth::AuthOperationsClient;
use apis::data_model::DataModelOperationsClient;
use middleware::header_middlware::HeaderMiddleware;
use surf::{Client, Config, Result};

pub mod apis;
mod middleware;
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
    pub async fn new(bearer_token: Option<String>) -> Result<Self> {
        let config = Config::new().set_base_url(surf::Url::parse(BASE_URL)?);
        let client: Client = config.try_into()?;

        let client = client.with(HeaderMiddleware { bearer_token });

        let account = AccountOperationsClient::new(client.clone());
        let auth = AuthOperationsClient::new(client.clone());
        let data_model = DataModelOperationsClient::new(client.clone());

        Ok(GtwSDK {
            client,
            auth,
            account,
            data_model,
        })
    }
}
