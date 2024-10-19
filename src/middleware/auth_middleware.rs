use surf::middleware::{Middleware, Next};
use surf::utils::async_trait;
use surf::{Client, Request, Response, Result};

use crate::services::wallet::WalletService;

pub struct AuthMiddleware {
    wallet: WalletService,
}

#[async_trait]
impl Middleware for AuthMiddleware {
    async fn handle(&self, mut req: Request, client: Client, next: Next<'_>) -> Result<Response> {
        next.run(req, client).await
    }
}
