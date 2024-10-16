use async_trait::async_trait;
use helper_generic_paginated_response::HelperGenericPaginatedResponse;
use reqwest::Client;

use crate::{
    models::*,
    utils::{error::*, handle_response::handle_response},
    BASE_URL,
};

use super::account::AccountOperationsClient;

pub struct DataModelOperationsClient {
    client: Client,
    pub acl: AccountOperationsClient,
}

impl DataModelOperationsClient {
    pub fn new(client: Client) -> Self {
        let acl = AccountOperationsClient::new(client.clone());
        Self { client, acl }
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

    async fn get(&self, id: u64) -> Result<DtoDataModelResponse, GTWError>;

    async fn create(
        &self,
        data_model_input: DtoDataModelCreateRequest,
    ) -> Result<DtoDataModelResponse, GTWError>;

    async fn update(
        &self,
        data_model_id: u64,
        data_model_input: DtoDataModelUpdateRequest,
    ) -> Result<DtoDataModelResponse, GTWError>;
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

    async fn create(
        &self,
        data_model_input: DtoDataModelCreateRequest,
    ) -> Result<DtoDataModelResponse, GTWError> {
        let url = format!("{}/data-models", BASE_URL);

        let response = self
            .client
            .post(&url)
            .json(&data_model_input)
            .send()
            .await
            .map_err(GTWError::NetworkError)?;

        handle_response(response).await
    }

    async fn update(
        &self,
        data_model_id: u64,
        data_model_input: DtoDataModelUpdateRequest,
    ) -> Result<DtoDataModelResponse, GTWError> {
        let url = format!("{}/data-models/{}", BASE_URL, data_model_id);

        let response = self
            .client
            .put(&url)
            .json(&data_model_input)
            .send()
            .await
            .map_err(GTWError::NetworkError)?;

        handle_response(response).await
    }

    async fn get(&self, id: u64) -> Result<DtoDataModelResponse, GTWError> {
        let url = format!("{}/data-models/{}", BASE_URL, id);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(GTWError::NetworkError)?;

        handle_response(response).await
    }

    async fn get_my(
        &self,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<HelperGenericPaginatedResponse<Vec<DtoDataModelResponse>>, GTWError> {
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
}
