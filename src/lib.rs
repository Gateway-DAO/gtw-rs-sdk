use reqwest::Client;
mod account;
mod auth;
mod data_model;
pub mod types;
mod utils;
pub const BASE_URL: &str = "https://dev.api.gateway.tech";

pub struct GtwApi {
    client: Client,
}

impl GtwApi {
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

        Ok(GtwApi { client })
    }
}
