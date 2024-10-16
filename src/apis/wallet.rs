use crate::{
    models::*,
    utils::{error::*, handle_response::handle_response},
    BASE_URL,
};
use reqwest::Client;
use serde_json::json;

use async_trait::async_trait;

#[async_trait]
pub trait WalletOperations {
    async fn add(&self, address: &str) -> Result<ModelMyAccountResponse, GTWError>;
    async fn remove(&self, address: &str) -> Result<ModelMyAccountResponse, GTWError>;
}

pub struct WalletOperationsClient {
    client: Client,
}

impl WalletOperationsClient {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl WalletOperations for WalletOperationsClient {
    async fn add(&self, address: &str) -> Result<ModelMyAccountResponse, GTWError> {
        let url = format!("{}/accounts/me/wallets", BASE_URL);

        let body = json!({
          "address" : address
        });

        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(GTWError::NetworkError)?;

        handle_response(response).await
    }

    async fn remove(&self, address: &str) -> Result<ModelMyAccountResponse, GTWError> {
        let url = format!("{}/accounts/me/wallets/{}", BASE_URL, address);

        let response = self
            .client
            .delete(&url)
            .send()
            .await
            .map_err(GTWError::NetworkError)?;

        handle_response(response).await
    }
}
