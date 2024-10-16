use async_trait::async_trait;
use helper_generic_paginated_response::HelperGenericPaginatedResponse;
use reqwest::multipart::{Form, Part};
use reqwest::Client;
use serde_json::json;

use crate::utils::validations::validate_file_name;
use crate::{
    models::*,
    utils::{error::*, handle_response::handle_response},
    BASE_URL,
};

pub struct FileDownload {
    pub file: Vec<u8>,
    pub file_name: String,
}

pub struct DataAssetOperationsClient {
    client: Client,
}

impl DataAssetOperationsClient {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
pub trait DataAssetOperation {
    async fn upload(
        &self,
        upload_body: DtoCreateDataAssetRequest,
    ) -> Result<DtoDataAssetIdRequestAndResponse, GTWError>;

    async fn upload_file(
        &self,
        file_name: &str,
        file_buffer: Vec<u8>,
        acl_list: Option<DtoAclRequest>,
        expiration_date: Option<String>,
    ) -> Result<DtoDataAssetIdRequestAndResponse, GTWError>;

    async fn get_created_by_me(
        &self,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<HelperGenericPaginatedResponse<Vec<DtoPublicDataAsset>>, GTWError>;

    async fn get_received_by_me(
        &self,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<HelperGenericPaginatedResponse<Vec<DtoPublicDataAsset>>, GTWError>;

    async fn get(&self, id: u64) -> Result<DtoPublicDataAsset, GTWError>;

    async fn update(
        &self,
        id: u64,
        update_asset_body: DtoCreateDataAssetRequest,
    ) -> Result<DtoPublicDataAsset, GTWError>;

    async fn update_file(
        &self,
        id: u64,
        file_name: &str,
        file_buffer: Vec<u8>,
        acl_list: Option<Vec<DtoAclRequest>>,
        expiration_date: Option<String>,
    ) -> Result<DtoPublicDataAsset, GTWError>;

    async fn delete(&self, id: u64) -> Result<DtoMessageResponse, GTWError>;

    async fn download(&self, id: u64) -> Result<FileDownload, GTWError>;

    async fn share(
        &self,
        id: u64,
        wallet_address_list: Vec<String>,
    ) -> Result<Vec<DtoPublicAcl>, GTWError>;
}

#[async_trait]
impl DataAssetOperation for DataAssetOperationsClient {
    async fn upload(
        &self,
        upload_body: DtoCreateDataAssetRequest,
    ) -> Result<DtoDataAssetIdRequestAndResponse, GTWError> {
        let url = format!("{}/data-assets", BASE_URL);

        let response = self
            .client
            .post(&url)
            .json(&upload_body)
            .send()
            .await
            .map_err(GTWError::NetworkError)?;

        handle_response(response).await
    }

    async fn upload_file(
        &self,
        file_name: &str,
        file_buffer: Vec<u8>,
        acl_list: Option<DtoAclRequest>,
        expiration_date: Option<String>,
    ) -> Result<DtoDataAssetIdRequestAndResponse, GTWError> {
        let extension =
            validate_file_name(file_name).map_err(|e| GTWError::UnexpectedError(e.to_string()))?;

        let file_part = Part::bytes(file_buffer)
            .file_name(file_name.to_string())
            .mime_str(&extension)
            .map_err(|e| GTWError::UnexpectedError(e.to_string()))?;

        let mut form = Form::new().part("data", file_part);

        if let Some(acl) = acl_list {
            let acl_json = serde_json::to_string(&acl)?;
            form = form.text("acl", acl_json);
        }

        if let Some(expiration) = expiration_date {
            form = form.text("expiration_date", expiration);
        }

        let url = format!("{}/data-assets/upload", BASE_URL);

        let response = self.client.post(&url).multipart(form).send().await?;

        handle_response(response).await
    }

    async fn get_created_by_me(
        &self,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<HelperGenericPaginatedResponse<Vec<DtoPublicDataAsset>>, GTWError> {
        let url = format!("{}/data-assets/created", BASE_URL);

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

    async fn get_received_by_me(
        &self,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<HelperGenericPaginatedResponse<Vec<DtoPublicDataAsset>>, GTWError> {
        let url = format!("{}/data-assets/received", BASE_URL);

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

    async fn get(&self, id: u64) -> Result<DtoPublicDataAsset, GTWError> {
        let url = format!("{}/data-assets/{}", BASE_URL, id);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(GTWError::NetworkError)?;

        handle_response(response).await
    }

    async fn update(
        &self,
        id: u64,
        update_asset_body: DtoCreateDataAssetRequest,
    ) -> Result<DtoPublicDataAsset, GTWError> {
        let url = format!("{}/data-assets/{}", BASE_URL, id);

        let response = self
            .client
            .put(&url)
            .json(&update_asset_body)
            .send()
            .await
            .map_err(GTWError::NetworkError)?;

        handle_response(response).await
    }

    async fn update_file(
        &self,
        id: u64,
        file_name: &str,
        file_buffer: Vec<u8>,
        acl_list: Option<Vec<DtoAclRequest>>,
        expiration_date: Option<String>,
    ) -> Result<DtoPublicDataAsset, GTWError> {
        let extension =
            validate_file_name(file_name).map_err(|e| GTWError::UnexpectedError(e.to_string()))?;

        let file_part = Part::bytes(file_buffer)
            .file_name(file_name.to_string())
            .mime_str(&extension)?;

        let mut form = Form::new().part("data", file_part);

        if let Some(acl) = acl_list {
            let acl_json = serde_json::to_string(&acl).map_err(GTWError::JsonError)?;
            form = form.text("acl", acl_json);
        }

        if let Some(expiration) = expiration_date {
            form = form.text("expiration_date", expiration);
        }

        let url = format!("{}/data-assets/{}", BASE_URL, id);
        let response = self
            .client
            .put(&url)
            .multipart(form)
            .send()
            .await
            .map_err(GTWError::NetworkError)?;

        handle_response(response).await
    }

    async fn delete(&self, id: u64) -> Result<DtoMessageResponse, GTWError> {
        let url = format!("{}/data-assets/{}", BASE_URL, id);

        let response = self
            .client
            .delete(&url)
            .send()
            .await
            .map_err(GTWError::NetworkError)?;

        handle_response::<DtoMessageResponse>(response).await
    }

    async fn download(&self, id: u64) -> Result<FileDownload, GTWError> {
        let model_public_data_asset = self.get(id).await?;
        let file_name = model_public_data_asset.name;

        let url = format!("{}/data-assets/{}/download", BASE_URL, id);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(GTWError::NetworkError)?;

        if response.status().is_success() {
            let file_data = response.bytes().await.map_err(GTWError::NetworkError)?;

            Ok(FileDownload {
                file: file_data.to_vec(),
                file_name,
            })
        } else {
            let status = response.status().as_u16();
            let error_message = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(GTWError::ApiError {
                status,
                message: error_message,
            })
        }
    }

    async fn share(
        &self,
        id: u64,
        wallet_address_list: Vec<String>,
    ) -> Result<Vec<DtoPublicAcl>, GTWError> {
        let url = format!("{}/data-assets/{}/share", BASE_URL, id);
        let body = json!({
            "addresses": wallet_address_list,
        });
        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(GTWError::NetworkError)?;

        handle_response(response).await
    }
}
