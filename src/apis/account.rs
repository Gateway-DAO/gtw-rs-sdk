use crate::{
    models::*,
    utils::{error::*, handle_response::handle_response},
};
use serde_json::json;
use surf::Client;

use super::wallet::WalletOperationsClient;

pub struct AccountOperationsClient {
    client: Client,
    pub wallet: WalletOperationsClient,
}

impl AccountOperationsClient {
    pub fn new(client: Client) -> Self {
        let wallet = WalletOperationsClient::new(client.clone());
        Self { client, wallet }
    }

    pub async fn create(
        &self,
        account_details: DtoAccountCreateRequest,
    ) -> Result<DtoMyAccountResponse, GTWError> {
        let url = "/accounts";

        let body = json!({
            "message": account_details.message,
            "signature": account_details.signature,
            "username": account_details.username,
            "wallet_address": account_details.wallet_address
        });

        let response = self
            .client
            .post(url)
            .body_json(&body)
            .map_err(|e| GTWError::NetworkError(SurfErrorWrapper(e)))?
            .await
            .map_err(|e| GTWError::NetworkError(SurfErrorWrapper(e)))?;

        handle_response(response).await
    }

    pub async fn get_me(&self) -> Result<DtoMyAccountResponse, GTWError> {
        let url = "/accounts/me";

        let response = self
            .client
            .get(url)
            .await
            .map_err(|e| GTWError::NetworkError(SurfErrorWrapper(e)))?;

        handle_response(response).await
    }

    pub async fn update_me(
        &self,
        profile_picture: &str,
        username: &str,
    ) -> Result<DtoMyAccountResponse, GTWError> {
        let url = "/accounts/me";

        let body = json!({
            "profile_picture": profile_picture,
            "username": username
        });

        let response = self
            .client
            .patch(url)
            .body_json(&body)
            .map_err(|e| GTWError::NetworkError(SurfErrorWrapper(e)))?
            .await
            .map_err(|e| GTWError::NetworkError(SurfErrorWrapper(e)))?;

        handle_response(response).await
    }

    pub async fn get_account(&self, did: &str) -> Result<DtoPublicAccountResponse, GTWError> {
        let url = format!("/accounts/{}", did);

        let response = self
            .client
            .get(&url)
            .await
            .map_err(|e| GTWError::NetworkError(SurfErrorWrapper(e)))?;

        handle_response(response).await
    }
}
