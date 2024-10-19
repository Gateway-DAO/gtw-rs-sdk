use crate::{
    models::*,
    utils::{error::*, handle_response::handle_response},
    BASE_URL,
};
use async_trait::async_trait;
use reqwest::Client;



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
        acl_list: Vec<DtoAclRequest>,
    ) -> Result<Vec<DtoPublicAcl>, GTWError> {
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
        acl_list: Vec<DtoAclRequest>,
    ) -> Result<Vec<DtoPublicAcl>, GTWError> {
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
        acl_list: DtoDeleteAclRequest,
    ) -> Result<DtoMessageResponse, GTWError> {
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
