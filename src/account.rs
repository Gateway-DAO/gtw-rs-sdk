use super::{models, BASE_URL, GTW_API};
use crate::utils::error::GTWError;
use serde_json::json;

impl GTW_API {
    pub fn new(bearer_token: String) -> Result<Self, reqwest::Error> {
        let client = reqwest::Client::builder().build()?;

        Ok(GTW_API {
            client,
            bearer_token,
        })
    }

    pub async fn account_info(&self) -> Result<models::ModelMyAccountResponse, GTWError> {
        let url = format!("{}/accounts/me", BASE_URL);
        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.bearer_token)
            .send()
            .await?;

        let body = response.text().await?;
        let json_response: models::ModelMyAccountResponse = serde_json::from_str(&body)?;

        Ok(json_response)
    }

    pub async fn update_account_info(
        &self,
        profile_picture: String,
        username: String,
    ) -> Result<models::ModelMyAccountResponse, GTWError> {
        let url = format!("{}/accounts/me", BASE_URL);

        let body = json!({"profile_picture" : profile_picture , "username" : username});

        let response = self
            .client
            .patch(&url)
            .bearer_auth(&self.bearer_token)
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await?;

        let body = response.text().await?;
        let json_response: models::ModelMyAccountResponse = serde_json::from_str(&body)?;

        Ok(json_response)
    }
}
