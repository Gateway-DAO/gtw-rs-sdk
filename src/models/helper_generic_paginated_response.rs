use super::{HelperLinks, HelperMeta};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct HelperGenericPaginatedResponse<T> {
    pub data: T,
    pub links: HelperLinks,
    pub meta: HelperMeta,
}

impl<T: Clone> Clone for HelperGenericPaginatedResponse<T> {
    fn clone(&self) -> Self {
        HelperGenericPaginatedResponse {
            data: self.data.clone(),
            links: self.links.clone(),
            meta: self.meta.clone(),
        }
    }
}

impl<T: Clone> From<&HelperGenericPaginatedResponse<T>> for HelperGenericPaginatedResponse<T> {
    fn from(value: &HelperGenericPaginatedResponse<T>) -> Self {
        value.clone()
    }
}
