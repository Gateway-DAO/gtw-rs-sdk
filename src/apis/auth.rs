use crate::{
    models::*,
    utils::{error::*, handle_response::handle_response},
    BASE_URL,
};
use reqwest::Client;
use serde_json::json;

use async_trait::async_trait;

#[async_trait]

pub trait AuthOperations {
    async fn login(
        &self,
        login_credetials: ModelAuthRequest,
    ) -> Result<ModelTokenResponse, GTWError>;

    async fn get_message(&self) -> Result<ModelMessageResponse, GTWError>;
    async fn get_refresh_token(&self) -> Result<ModelTokenResponse, GTWError>;
}

pub struct AuthOperationsClient {
    client: Client,
}

impl AuthOperationsClient {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]

impl AuthOperations for AuthOperationsClient {
    async fn login(
        &self,
        login_credetials: ModelAuthRequest,
    ) -> Result<ModelTokenResponse, GTWError> {
        let url = format!("{}/auth", BASE_URL);

        let body = json!({
            "message": login_credetials.message,
            "signature": login_credetials.signature,
            "wallet_address": login_credetials.wallet_address
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

    async fn get_message(&self) -> Result<ModelMessageResponse, GTWError> {
        let url = format!("{}/auth/message", BASE_URL);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(GTWError::NetworkError)?;

        handle_response(response).await
    }

    async fn get_refresh_token(&self) -> Result<ModelTokenResponse, GTWError> {
        let url = format!("{}/auth/refresh-token", BASE_URL);
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(GTWError::NetworkError)?;

        handle_response(response).await
    }
}
