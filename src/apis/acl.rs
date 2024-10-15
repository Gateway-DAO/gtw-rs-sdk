use crate::{
    models::*,
    utils::{error::*, handle_response::handle_response},
    BASE_URL,
};
use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait AclOperations {
    async fn update(
        &self,
        id: u64,
        acl_list: Vec<ModelAclRequest>,
    ) -> Result<Vec<ModelPublicAcl>, GTWError>;

    async fn add(
        &self,
        id: u64,
        acl_list: Vec<ModelAclRequest>,
    ) -> Result<Vec<ModelPublicAcl>, GTWError>;

    async fn delete(
        &self,
        id: u64,
        acl_list: ModelDeleteAclRequest,
    ) -> Result<ModelMessageResponse, GTWError>;
}

pub struct AclOperationsClient {
    client: Client,
}

impl AclOperationsClient {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl AclOperations for AclOperationsClient {
    async fn update(
        &self,
        id: u64,
        acl_list: Vec<ModelAclRequest>,
    ) -> Result<Vec<ModelPublicAcl>, GTWError> {
        let url = format!("{}/data-assets/{}/acl", BASE_URL, id);

        let response = self
            .client
            .patch(&url)
            .json(&acl_list)
            .send()
            .await
            .map_err(GTWError::NetworkError)?;

        handle_response(response).await
    }

    async fn add(
        &self,
        id: u64,
        acl_list: Vec<ModelAclRequest>,
    ) -> Result<Vec<ModelPublicAcl>, GTWError> {
        let url = format!("{}/data-assets/{}/acl", BASE_URL, id);

        let response = self
            .client
            .post(&url)
            .json(&acl_list)
            .send()
            .await
            .map_err(GTWError::NetworkError)?;

        handle_response(response).await
    }

    async fn delete(
        &self,
        id: u64,
        acl_list: ModelDeleteAclRequest,
    ) -> Result<ModelMessageResponse, GTWError> {
        let url = format!("{}/data-assets/{}/acl/delete", BASE_URL, id);

        let response = self
            .client
            .patch(&url)
            .json(&vec![acl_list])
            .send()
            .await
            .map_err(GTWError::NetworkError)?;

        handle_response(response).await
    }
}
