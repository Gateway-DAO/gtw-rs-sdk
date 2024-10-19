use surf::middleware::{Middleware, Next};
use surf::utils::async_trait;
use surf::{Client, Request, Response, Result};

use crate::services::wallet::WalletService;
use crate::utils::jwt::{is_token_expired, issue_jwt_token};

pub struct AuthMiddleware {
    pub(crate) wallet: WalletService,
}

#[async_trait]
impl Middleware for AuthMiddleware {
    async fn handle(&self, mut req: Request, client: Client, next: Next<'_>) -> Result<Response> {
        let mut access_token = req
            .header("Authorization")
            .and_then(|h| h.get(0))
            .map(|s| s.to_string())
            .unwrap_or_default();

        if access_token.is_empty() {
            access_token = issue_jwt_token(client.clone(), &self.wallet)
                .await
                .map_err(|e| {
                    let err_msg = format!("failed to issue new token: {}", e);
                    surf::Error::from_str(500, err_msg)
                })?;
        } else {
            let is_valid = is_token_expired(&access_token).await;

            if !is_valid {
                access_token = issue_jwt_token(client.clone(), &self.wallet)
                    .await
                    .map_err(|e| {
                        let err_msg = format!("failed to issue new token: {}", e);
                        surf::Error::from_str(500, err_msg)
                    })?;
            }
        }
        req.insert_header("Authorization", access_token);

        next.run(req, client).await
    }
}
