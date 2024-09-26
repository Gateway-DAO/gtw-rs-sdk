use super::{types::*, GtwApi, BASE_URL};
use crate::utils::{error::GTWError, handle_response::handle_response};

impl GtwApi {
    pub async fn get_data_models(
        &self,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<HelperPaginatedResponse<Vec<ModelDataModel>>, GTWError> {
        let url = format!("{}/data-models", BASE_URL);

        let mut query_params = vec![];
        if let Some(page) = page {
            query_params.push(("page", page.to_string()));
        }
        if let Some(page_size) = page_size {
            query_params.push(("page_size", page_size.to_string()));
        }

        let response = self
            .client
            .get(&url)
            .query(&query_params)
            .send()
            .await
            .map_err(GTWError::NetworkError)?;

        handle_response(response).await
    }

    pub async fn get_user_data_models(
        &self,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<HelperPaginatedResponse<Vec<ModelDataModel>>, GTWError> {
        let url = format!("{}/data-models/me", BASE_URL);

        let mut query_params = vec![];
        if let Some(page) = page {
            query_params.push(("page", page.to_string()));
        }
        if let Some(page_size) = page_size {
            query_params.push(("page_size", page_size.to_string()));
        }

        let response = self
            .client
            .get(&url)
            .query(&query_params)
            .send()
            .await
            .map_err(GTWError::NetworkError)?;

        handle_response(response).await
    }

    pub async fn get_data_model_by_id(&self, id: u64) -> Result<ModelDataModel, GTWError> {
        let url = format!("{}/data-models/{}", BASE_URL, id);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(GTWError::NetworkError)?;

        handle_response(response).await
    }
}
