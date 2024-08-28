use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new() -> Self {
        ApiClient {
            client: Client::new(),
            base_url: "https://dev.api.gateway.tech".to_string(),
        }
    }
}
