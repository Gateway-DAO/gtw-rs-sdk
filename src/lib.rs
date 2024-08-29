use reqwest::Client;
mod auth;
pub mod models;

pub const BASE_URL: &str = "https://dev.api.gateway.tech";

pub struct GTW_API {
    client: Client,
    bearer_token: String,
}
