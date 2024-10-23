use super::{HelperLinks, HelperMeta};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HelperGenericPaginatedResponse<T> {
    pub data: T,
    pub links: HelperLinks,
    pub meta: HelperMeta,
}

impl<T> From<&HelperGenericPaginatedResponse<T>> for HelperGenericPaginatedResponse<T>
where
    T: Clone,
{
    fn from(value: &HelperGenericPaginatedResponse<T>) -> Self {
        value.clone()
    }
}
