use crate::utils::error::{GTWError, SurfErrorWrapper};
use serde::de::DeserializeOwned;
use surf::Response;

pub async fn handle_response<T: DeserializeOwned>(mut response: Response) -> Result<T, GTWError> {
    if response.status().is_success() {
        response
            .body_json::<T>()
            .await
            .map_err(|e| GTWError::NetworkError(SurfErrorWrapper(e))) // Treat any error as a network error
    } else {
        let error_body = response
            .body_string()
            .await
            .map_err(|e| GTWError::NetworkError(SurfErrorWrapper(e)))?;

        Err(GTWError::ApiError {
            status: response.status().into(),
            message: error_body,
        })
    }
}
