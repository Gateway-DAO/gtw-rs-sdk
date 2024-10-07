use crate::{
    models::*,
    utils::{error::*, handle_response::handle_response},
    GtwSDK, BASE_URL,
};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

#[async_trait]
pub trait AccountOperations {
    async fn create_account(
        &self,
        account_details: ModelAccountCreateRequest,
    ) -> Result<ModelMyAccountResponse, GTWError>;

    async fn account_info(&self) -> Result<ModelMyAccountResponse, GTWError>;

    async fn update_account_info(
        &self,
        profile_picture: &str,
        username: &str,
    ) -> Result<ModelMyAccountResponse, GTWError>;
}

pub struct AccountOperationsClient {
    client: Client,
}

impl AccountOperationsClient {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl AccountOperations for AccountOperationsClient {
    async fn create_account(
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

    async fn account_info(&self) -> Result<ModelMyAccountResponse, GTWError> {
        let url = format!("{}/accounts/me", BASE_URL);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(GTWError::NetworkError)?;

        handle_response(response).await
    }

    async fn update_account_info(
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
}
