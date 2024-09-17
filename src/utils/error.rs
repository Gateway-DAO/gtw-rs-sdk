use thiserror::Error;

#[derive(Error, Debug)]
pub enum GTWError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("API error: Status {status}, Message: {message}")]
    ApiError { status: u16, message: String },

    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}
