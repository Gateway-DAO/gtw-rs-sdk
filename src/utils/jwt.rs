use crate::{
    apis::auth::AuthOperationsClient, models::DtoAuthRequest, services::wallet::WalletService,
};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};
use surf::Client;

use super::error::GTWError;

pub async fn is_token_expired(token: &str) -> bool {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return true;
    }

    let exp = (|| {
        let payload = URL_SAFE.decode(parts[1]).ok()?;
        let payload_str = String::from_utf8(payload).ok()?;
        let claims: Value = serde_json::from_str(&payload_str).ok()?;
        claims["exp"].as_i64()
    })();

    if let Some(exp) = exp {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        return exp <= now;
    }

    true
}

pub async fn issue_jwt_token(client: Client, wallet: &WalletService) -> Result<String, GTWError> {
    let auth = AuthOperationsClient::new(client.clone());

    let sign_message = auth
        .get_message()
        .await
        .map_err(|e| GTWError::UnexpectedError(format!("Network error: {}", e)))?
        .message;

    let signature_details = wallet
        .sign_message(&sign_message)
        .await
        .map_err(|e| GTWError::UnexpectedError(format!("Failed to sign message: {}", e)))?;

    let login_credentials = DtoAuthRequest {
        message: sign_message,
        signature: signature_details.signature,
        wallet_address: signature_details.signing_key.to_string(),
    };

    match auth.login(login_credentials).await {
        Ok(jwt) => Ok(jwt.token),
        Err(e) => {
            let message = format!("Login failed: {}", e);
            Err(GTWError::UnexpectedError(message))
        }
    }
}
