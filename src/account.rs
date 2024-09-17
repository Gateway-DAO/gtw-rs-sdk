use super::{models, BASE_URL, GTW_API};
use crate::utils::error::GTWError;
use reqwest::Response;
use serde_json::json;

impl GTW_API {
    pub fn new(bearer_token: String) -> Result<Self, reqwest::Error> {
        let client = reqwest::Client::builder()
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    reqwest::header::AUTHORIZATION,
                    format!("Bearer {}", bearer_token).parse().unwrap(),
                );
                headers.insert(
                    reqwest::header::CONTENT_TYPE,
                    "application/json".parse().unwrap(),
                );
                headers
            })
            .build()?;

        Ok(GTW_API {
            client,
            bearer_token,
        })
    }

    async fn handle_response<T: serde::de::DeserializeOwned>(
        response: Response,
    ) -> Result<T, GTWError> {
        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| GTWError::UnexpectedError(e.to_string()))?;

        if !status.is_success() {
            return Err(GTWError::ApiError {
                status: status.as_u16(),
                message: body,
            });
        }

        serde_json::from_str(&body).map_err(GTWError::JsonError)
    }

    pub async fn create_account(
        &self,
        account_details: models::ModelAccountCreateRequest,
    ) -> Result<models::ModelMyAccountResponse, GTWError> {
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

        Self::handle_response(response).await
    }

    pub async fn account_info(&self) -> Result<models::ModelMyAccountResponse, GTWError> {
        let url = format!("{}/accounts/me", BASE_URL);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(GTWError::NetworkError)?;

        Self::handle_response(response).await
    }

    pub async fn update_account_info(
        &self,
        profile_picture: String,
        username: String,
    ) -> Result<models::ModelMyAccountResponse, GTWError> {
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

        Self::handle_response(response).await
    }
}
