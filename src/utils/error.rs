use std::fmt;
use surf::Error as SurfError;
use thiserror::Error;

#[derive(Debug)]
pub struct SurfErrorWrapper(pub SurfError); // Wrapper around surf::Error

impl fmt::Display for SurfErrorWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Surf error: {}", self.0)
    }
}

impl std::error::Error for SurfErrorWrapper {}

#[derive(Error, Debug)]
pub enum GTWError {
    #[error("Network error: {0}")]
    NetworkError(#[from] SurfErrorWrapper), // Wrap Surf errors

    #[error("API error: Status {status}, Message: {message}")]
    ApiError { status: u16, message: String },

    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}
