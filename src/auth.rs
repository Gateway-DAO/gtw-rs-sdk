use super::{models, BASE_URL, GTW_API};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GTWError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
}

impl GTW_API {
    pub fn new(bearer_token: String) -> Result<Self, reqwest::Error> {
        let client = reqwest::Client::builder().build()?;

        Ok(GTW_API {
            client,
            bearer_token,
        })
    }

    pub async fn me(&self) -> Result<models::ModelMyAccountResponse, GTWError> {
        let url = format!("{}/me", BASE_URL);
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

    pub async fn add_funds(
        &self,
        account_id: f64,
        amount: f64,
    ) -> Result<models::ModelAccountLedgerAddRequest, GTWError> {
        let url = format!("{}/accounts/{}/add-funds", BASE_URL, account_id);

        let body = json!({ "amount": amount });

        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.bearer_token)
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await?;

        let body = response.text().await?;
        println!("Response Body: {}", body);

        let json_response: models::ModelAccountLedgerAddRequest = serde_json::from_str(&body)?;

        Ok(json_response)
    }
}
