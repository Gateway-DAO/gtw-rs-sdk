use async_trait::async_trait;
use helper_generic_paginated_response::HelperGenericPaginatedResponse;
use reqwest::Client;

use crate::{
    models::*,
    utils::{error::*, handle_response::handle_response},
    BASE_URL,
};

pub struct DataModelOperationsClient {
    client: Client,
}

impl DataModelOperationsClient {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
pub trait DataModelOperation {
    async fn get_all(
        &self,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<HelperGenericPaginatedResponse<Vec<ModelDataModel>>, GTWError>;
    async fn get_my(
        &self,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<HelperGenericPaginatedResponse<Vec<ModelDataModel>>, GTWError>;

    async fn get_by_id(&self, id: u64) -> Result<ModelDataModel, GTWError>;
}

#[async_trait]
impl DataModelOperation for DataModelOperationsClient {
    async fn get_all(
        &self,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<HelperGenericPaginatedResponse<Vec<ModelDataModel>>, GTWError> {
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

    async fn get_my(
        &self,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<HelperGenericPaginatedResponse<Vec<ModelDataModel>>, GTWError> {
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

    async fn get_by_id(&self, id: u64) -> Result<ModelDataModel, GTWError> {
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
