use surf::middleware::{Middleware, Next};
use surf::utils::async_trait;
use surf::{Client, Config, Request, Response, Result};

#[derive(Clone, Debug)]
pub struct HeaderMiddleware {
    pub(crate) bearer_token: Option<String>,
}

#[async_trait]
impl Middleware for HeaderMiddleware {
    async fn handle(&self, mut req: Request, client: Client, next: Next<'_>) -> Result<Response> {
        if let Some(token) = &self.bearer_token {
            req.insert_header("Authorization", format!("Bearer {}", token));
        }
        req.insert_header("Content-Type", "application/json");

        next.run(req, client).await
    }
}
