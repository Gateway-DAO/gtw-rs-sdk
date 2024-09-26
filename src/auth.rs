use super::{types::*, GtwApi, BASE_URL};
use crate::utils::{error::GTWError, handle_response::handle_response};
use serde_json::json;

impl GtwApi {
    pub async fn login(
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

    pub async fn generate_message(&self) -> Result<ModelMessageResponse, GTWError> {
        let url = format!("{}/auth/message", BASE_URL);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(GTWError::NetworkError)?;

        handle_response(response).await
    }

    pub async fn generate_refresh_token(&self) -> Result<ModelTokenResponse, GTWError> {
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
