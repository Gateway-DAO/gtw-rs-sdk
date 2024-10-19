use helper_generic_paginated_response::HelperGenericPaginatedResponse;
use serde_json::json;
use surf::Client;

use crate::{
    models::*,
    utils::{error::*, handle_response::handle_response},
};

pub struct DataModelOperationsClient {
    client: Client,
}

impl DataModelOperationsClient {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn get_all(
        &self,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<HelperGenericPaginatedResponse<Vec<DtoDataModelResponse>>, GTWError> {
        let url = format!("/data-models");

        let page = page.unwrap_or(1);
        let page_size = page_size.unwrap_or(10);

        let response = self
            .client
            .get(&url)
            .query(&[("page", page), ("page_size", page_size)])
            .map_err(|e| GTWError::NetworkError(SurfErrorWrapper(e)))?
            .await
            .map_err(|e| GTWError::NetworkError(SurfErrorWrapper(e)))?;

        handle_response(response).await
    }

    pub async fn create(
        &self,
        data_model_input: DtoDataModelCreateRequest,
    ) -> Result<DtoDataModelResponse, GTWError> {
        let url = format!("/data-models");

        let body = json!({
            "description": data_model_input.description,
            "schema": data_model_input.schema,
            "tags": data_model_input.tags,
            "title": data_model_input.title
        });

        let response = self
            .client
            .post(&url)
            .body(body.to_string())
            .await
            .map_err(|e| GTWError::NetworkError(SurfErrorWrapper(e)))?;

        handle_response(response).await
    }

    pub async fn update(
        &self,
        data_model_id: u64,
        data_model_input: DtoDataModelUpdateRequest,
    ) -> Result<DtoDataModelResponse, GTWError> {
        let url = format!("/data-models/{}", data_model_id);
        let request_builder = self.client.put(&url).body_json(&data_model_input);

        let response = request_builder
            .map_err(|e| GTWError::NetworkError(SurfErrorWrapper(e)))?
            .await
            .map_err(|e| GTWError::NetworkError(SurfErrorWrapper(e)))?;

        handle_response(response).await
    }

    pub async fn get_my(
        &self,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<HelperGenericPaginatedResponse<Vec<DtoDataModelResponse>>, GTWError> {
        let url = format!("/data-models/me");

        let page = page.unwrap_or(1);
        let page_size = page_size.unwrap_or(10);

        let mut request_builder = self.client.get(&url);

        request_builder = request_builder
            .query(&[("page", page), ("page_size", page_size)])
            .map_err(|e| GTWError::NetworkError(SurfErrorWrapper(e)))?;

        let response = request_builder
            .await
            .map_err(|e| GTWError::NetworkError(SurfErrorWrapper(e)))?;

        handle_response(response).await
    }

    pub async fn get(&self, id: u64) -> Result<DtoDataModelResponse, GTWError> {
        let url = format!("/data-models/{}", id);

        let response = self
            .client
            .get(&url)
            .await
            .map_err(|e| GTWError::NetworkError(SurfErrorWrapper(e)))?;

        handle_response(response).await
    }

    pub async fn get_data_assets(
        &self,
        data_model_id: u64,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<HelperGenericPaginatedResponse<Vec<DtoPublicDataAsset>>, GTWError> {
        let url = format!("/data-models/{}/data-assets", data_model_id);

        let page = page.unwrap_or(1);
        let page_size = page_size.unwrap_or(10);

        let response = self
            .client
            .get(&url)
            .query(&[("page", page), ("page_size", page_size)])
            .map_err(|e| GTWError::NetworkError(SurfErrorWrapper(e)))?
            .await
            .map_err(|e| GTWError::NetworkError(SurfErrorWrapper(e)))?;

        handle_response(response).await
    }
}
