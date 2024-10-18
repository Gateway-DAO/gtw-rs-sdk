use crate::{
    models::*,
    utils::{error::*, handle_response::handle_response},
};
use serde_json::json;
use surf::Client;

pub struct AuthOperationsClient {
    client: Client,
}

impl AuthOperationsClient {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn login(
        &self,
        login_credentials: DtoAuthRequest,
    ) -> Result<DtoTokenResponse, GTWError> {
        let url = format!("/auth");

        let body = json!({
            "message": login_credentials.message,
            "signature": login_credentials.signature,
            "wallet_address": login_credentials.wallet_address
        });

        let response = self
            .client
            .post(&url)
            .body(body.to_string())
            .await
            .map_err(|e| GTWError::NetworkError(SurfErrorWrapper(e)))?; 

        handle_response(response).await
    }

    pub async fn get_message(&self) -> Result<DtoMessageResponse, GTWError> {
        let url = format!("/auth/message");

        let response = self
            .client
            .get(&url)
            .await
            .map_err(|e| GTWError::NetworkError(SurfErrorWrapper(e)))?; // Wrap surf::Error

        handle_response(response).await
    }

    pub async fn get_refresh_token(&self) -> Result<DtoTokenResponse, GTWError> {
        let url = format!("/auth/refresh-token");

        let response = self
            .client
            .get(&url)
            .await
            .map_err(|e| GTWError::NetworkError(SurfErrorWrapper(e)))?; // Wrap surf::Error

        handle_response(response).await
    }
}
