use crate::{
    models::*,
    utils::{error::*, handle_response::handle_response},
    BASE_URL,
};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

use super::wallet::{WalletOperations, WalletOperationsClient};

#[async_trait]
pub trait AccountOperations {
    async fn create(
        &self,
        account_details: ModelAccountCreateRequest,
    ) -> Result<ModelMyAccountResponse, GTWError>;

    async fn get_me(&self) -> Result<ModelMyAccountResponse, GTWError>;

    async fn update_me(
        &self,
        profile_picture: &str,
        username: &str,
    ) -> Result<ModelMyAccountResponse, GTWError>;

    fn wallet(&self) -> &dyn WalletOperations;
}

pub struct AccountOperationsClient {
    client: Client,
    pub wallet: WalletOperationsClient,
}

impl AccountOperationsClient {
    pub fn new(client: Client) -> Self {
        let wallet = WalletOperationsClient::new(client.clone());
        Self { client, wallet }
    }
}

#[async_trait]
impl AccountOperations for AccountOperationsClient {
    async fn create(
        &self,
        account_details: ModelAccountCreateRequest,
    ) -> Result<ModelMyAccountResponse, GTWError> {
        let url = format!("{}/accounts", BASE_URL);

        let body = json!({
            "message": account_details.message,
            "signature": account_details.signature,
            "username": account_details.username,
            "wallet_address": account_details.wallet_address
        });

        let response = self
            .client
            .post(&url)
            .body(body.to_string())
            .send()
            .await
            .map_err(GTWError::NetworkError)?;

        handle_response(response).await
    }

    async fn get_me(&self) -> Result<ModelMyAccountResponse, GTWError> {
        let url = format!("{}/accounts/me", BASE_URL);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(GTWError::NetworkError)?;

        handle_response(response).await
    }

    async fn update_me(
        &self,
        profile_picture: &str,
        username: &str,
    ) -> Result<ModelMyAccountResponse, GTWError> {
        let url = format!("{}/accounts/me", BASE_URL);

        let body = json!({
            "profile_picture": profile_picture,
            "username": username
        });

        let response = self
            .client
            .patch(&url)
            .body(body.to_string())
            .send()
            .await
            .map_err(GTWError::NetworkError)?;

        handle_response(response).await
    }

    fn wallet(&self) -> &dyn WalletOperations {
        &self.wallet
    }
}
