use super::error::GTWError;
use reqwest::Response;

pub async fn handle_response<T: serde::de::DeserializeOwned>(
    response: Response,
) -> Result<T, GTWError> {
    let status = response.status();
    let body = response
        .text()
        .await
        .map_err(|e| GTWError::UnexpectedError(e.to_string()))?;

    if !status.is_success() {
        return Err(GTWError::ApiError {
            status: status.as_u16(),
            message: body,
        });
    }

    serde_json::from_str(&body).map_err(GTWError::JsonError)
}
