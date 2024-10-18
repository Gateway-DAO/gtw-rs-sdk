use crate::{
    models::*,
    utils::{error::*, handle_response::handle_response},
    BASE_URL,
};
use serde_json::json;
use surf::Client;

pub struct WalletOperationsClient {
    client: Client,
}

impl WalletOperationsClient {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

impl WalletOperationsClient {
    async fn add(&self, address: &str) -> Result<DtoMyAccountResponse, GTWError> {
        let url = format!("{}/accounts/me/wallets", BASE_URL);

        let body = json!({ "address": address });

        let response = self
            .client
            .post(&url)
            .body_json(&body)
            .map_err(GTWError::NetworkError)?
            .await
            .map_err(GTWError::NetworkError)?;

        handle_response(response).await
    }

    async fn remove(&self, address: &str) -> Result<DtoMyAccountResponse, GTWError> {
        let url = format!("{}/accounts/me/wallets/{}", BASE_URL, address);

        let response = self
            .client
            .delete(&url)
            .await
            .map_err(GTWError::NetworkError)?;

        handle_response(response).await
    }
}
