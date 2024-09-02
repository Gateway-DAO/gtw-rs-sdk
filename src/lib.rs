use reqwest::Client;
mod account;
pub mod models;
mod utils;

pub const BASE_URL: &str = "https://dev.api.gateway.tech";

pub struct GTW_API {
    client: Client,
    bearer_token: String,
}
