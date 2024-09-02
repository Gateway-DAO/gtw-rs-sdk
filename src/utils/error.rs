use thiserror::Error;

#[derive(Error, Debug)]

pub enum GTWError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
}
