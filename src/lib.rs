use apis::account::AccountOperationsClient;
use apis::auth::AuthOperationsClient;
use apis::data_model::DataModelOperationsClient;
use middleware::{auth_middleware::AuthMiddleware, header_middlware::HeaderMiddleware};
use services::wallet::{WalletService, WalletType};
use surf::{Client, Config, Result};

pub mod apis;
mod middleware;
pub mod models;
pub mod services;
mod utils;

pub struct GtwSDK {
    pub auth: AuthOperationsClient,
    pub account: AccountOperationsClient,
    pub data_model: DataModelOperationsClient,
}

#[derive(Clone, Debug)]
pub struct SdkConfig {
    pub api_url: Option<String>,
    pub bearer_token: Option<String>,
    pub wallet: Option<WalletType>,
    pub private_key: Option<String>,
}

impl Default for SdkConfig {
    fn default() -> Self {
        Self {
            api_url: Some("https://dev.api.gateway.tech".to_string()),
            bearer_token: None,
            wallet: None,
            private_key: None,
        }
    }
}

impl GtwSDK {
    pub async fn new(sdk_config: SdkConfig) -> Result<Self> {
        let api_url = sdk_config
            .api_url
            .unwrap_or_else(|| "https://dev.api.gateway.tech".to_string());

        let config = Config::new().set_base_url(surf::Url::parse(&api_url).map_err(|e| {
            surf::Error::from_str(
                surf::StatusCode::BadRequest,
                format!("Invalid API URL: {}", e),
            )
        })?);

        let mut client: Client = config.try_into()?;

        if let Some(ref bearer_token) = sdk_config.bearer_token {
            client = client.with(HeaderMiddleware {
                bearer_token: Some(bearer_token.clone()),
            });
        }

        let wallet = if sdk_config.bearer_token.is_none() {
            match (sdk_config.wallet, sdk_config.private_key) {
                (Some(wallet_type), Some(private_key)) => {
                    match WalletService::new(private_key, Some(wallet_type)) {
                        Ok(wallet_service) => Some(wallet_service),
                        Err(e) => {
                            eprintln!("Failed to create wallet service: {}", e);
                            None
                        }
                    }
                }
                _ => None,
            }
        } else {
            None
        };

        if let Some(wallet) = wallet {
            client = client.with(AuthMiddleware { wallet });
        }

        let account = AccountOperationsClient::new(client.clone());
        let auth = AuthOperationsClient::new(client.clone());
        let data_model = DataModelOperationsClient::new(client);

        Ok(GtwSDK {
            auth,
            account,
            data_model,
        })
    }
}
