// Code generated using `make generate`.
// Don't change it

use serde::{Deserialize, Serialize};

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a TryFrom or FromStr implementation."]
    pub struct ConversionError(std::borrow::Cow<'static, str>);
    impl std::error::Error for ConversionError {}
    impl std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "DtoAccountCreateRequest"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"message\","]
#[doc = "    \"signature\","]
#[doc = "    \"username\","]
#[doc = "    \"wallet_address\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"message\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"signature\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"username\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"wallet_address\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoAccountCreateRequest {
    pub message: String,
    pub signature: String,
    pub username: String,
    pub wallet_address: String,
}
impl From<&DtoAccountCreateRequest> for DtoAccountCreateRequest {
    fn from(value: &DtoAccountCreateRequest) -> Self {
        value.clone()
    }
}
#[doc = "DtoAccountUpdateRequest"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"profile_picture\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"username\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoAccountUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_picture: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}
impl From<&DtoAccountUpdateRequest> for DtoAccountUpdateRequest {
    fn from(value: &DtoAccountUpdateRequest) -> Self {
        value.clone()
    }
}
#[doc = "DtoAclRequest"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"address\","]
#[doc = "    \"roles\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"address\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"roles\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/types.AccessLevel\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoAclRequest {
    pub address: String,
    pub roles: Vec<TypesAccessLevel>,
}
impl From<&DtoAclRequest> for DtoAclRequest {
    fn from(value: &DtoAclRequest) -> Self {
        value.clone()
    }
}
#[doc = "DtoAuthRequest"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"message\","]
#[doc = "    \"signature\","]
#[doc = "    \"wallet_address\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"message\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"signature\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"wallet_address\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoAuthRequest {
    pub message: String,
    pub signature: String,
    pub wallet_address: String,
}
impl From<&DtoAuthRequest> for DtoAuthRequest {
    fn from(value: &DtoAuthRequest) -> Self {
        value.clone()
    }
}
#[doc = "DtoComputeRequestCreateRequest"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"compute_field_name\","]
#[doc = "    \"compute_operation\","]
#[doc = "    \"data_model_id\","]
#[doc = "    \"description\","]
#[doc = "    \"title\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"compute_field_name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"compute_operation\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"add\","]
#[doc = "        \"subtract\","]
#[doc = "        \"multiply\","]
#[doc = "        \"divide\","]
#[doc = "        \"less_than\","]
#[doc = "        \"equal\","]
#[doc = "        \"not_equal\","]
#[doc = "        \"sum\","]
#[doc = "        \"greater_than\","]
#[doc = "        \"greater_than_or_equal\""]
#[doc = "      ],"]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/types.ComputeOperation\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"data_model_id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoComputeRequestCreateRequest {
    pub compute_field_name: String,
    pub compute_operation: TypesComputeOperation,
    pub data_model_id: i64,
    pub description: String,
    pub title: String,
}
impl From<&DtoComputeRequestCreateRequest> for DtoComputeRequestCreateRequest {
    fn from(value: &DtoComputeRequestCreateRequest) -> Self {
        value.clone()
    }
}
#[doc = "DtoComputeRequestCreateResponse"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"compute_field_name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"compute_operation\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"created_at\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"created_by\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"data_model_id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"updated_at\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoComputeRequestCreateResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compute_field_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compute_operation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_model_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}
impl From<&DtoComputeRequestCreateResponse> for DtoComputeRequestCreateResponse {
    fn from(value: &DtoComputeRequestCreateResponse) -> Self {
        value.clone()
    }
}
#[doc = "DtoCreateDataAssetRequest"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"acl\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/dto.ACLRequest\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"claim\": {"]
#[doc = "      \"type\": \"object\""]
#[doc = "    },"]
#[doc = "    \"data_model_id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"expiration_date\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"tags\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoCreateDataAssetRequest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub acl: Vec<DtoAclRequest>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub claim: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_model_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}
impl From<&DtoCreateDataAssetRequest> for DtoCreateDataAssetRequest {
    fn from(value: &DtoCreateDataAssetRequest) -> Self {
        value.clone()
    }
}
#[doc = "DtoDataAssetIdRequestAndResponse"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoDataAssetIdRequestAndResponse {
    pub id: i64,
}
impl From<&DtoDataAssetIdRequestAndResponse> for DtoDataAssetIdRequestAndResponse {
    fn from(value: &DtoDataAssetIdRequestAndResponse) -> Self {
        value.clone()
    }
}
#[doc = "DtoDataModelCreateRequest"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"description\","]
#[doc = "    \"schema\","]
#[doc = "    \"title\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"schema\": {"]
#[doc = "      \"type\": \"object\""]
#[doc = "    },"]
#[doc = "    \"tags\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoDataModelCreateRequest {
    pub description: String,
    pub schema: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    pub title: String,
}
impl From<&DtoDataModelCreateRequest> for DtoDataModelCreateRequest {
    fn from(value: &DtoDataModelCreateRequest) -> Self {
        value.clone()
    }
}
#[doc = "DtoDataModelResponse"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"created_at\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"schema\": {"]
#[doc = "      \"type\": \"object\""]
#[doc = "    },"]
#[doc = "    \"tags\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"updated_at\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoDataModelResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub schema: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}
impl From<&DtoDataModelResponse> for DtoDataModelResponse {
    fn from(value: &DtoDataModelResponse) -> Self {
        value.clone()
    }
}
#[doc = "DtoDataModelUpdateRequest"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"schema\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"schema\": {"]
#[doc = "      \"type\": \"object\""]
#[doc = "    },"]
#[doc = "    \"tags\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoDataModelUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub id: i64,
    pub schema: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
impl From<&DtoDataModelUpdateRequest> for DtoDataModelUpdateRequest {
    fn from(value: &DtoDataModelUpdateRequest) -> Self {
        value.clone()
    }
}
#[doc = "DtoDeleteAclRequest"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"addresses\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"addresses\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoDeleteAclRequest {
    pub addresses: Vec<String>,
}
impl From<&DtoDeleteAclRequest> for DtoDeleteAclRequest {
    fn from(value: &DtoDeleteAclRequest) -> Self {
        value.clone()
    }
}
#[doc = "DtoMessageResponse"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"message\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"message\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoMessageResponse {
    pub message: String,
}
impl From<&DtoMessageResponse> for DtoMessageResponse {
    fn from(value: &DtoMessageResponse) -> Self {
        value.clone()
    }
}
#[doc = "DtoMyAccountResponse"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"created_at\","]
#[doc = "    \"did\","]
#[doc = "    \"storage_size\","]
#[doc = "    \"updated_at\","]
#[doc = "    \"username\","]
#[doc = "    \"username_updated_at\","]
#[doc = "    \"wallet_addresses\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"created_at\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"did\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"profile_picture\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"storage_size\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"updated_at\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"username\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"username_updated_at\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"wallet_addresses\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/model.WalletAddress\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoMyAccountResponse {
    pub created_at: String,
    pub did: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_picture: Option<String>,
    pub storage_size: i64,
    pub updated_at: String,
    pub username: String,
    pub username_updated_at: String,
    pub wallet_addresses: Vec<ModelWalletAddress>,
}
impl From<&DtoMyAccountResponse> for DtoMyAccountResponse {
    fn from(value: &DtoMyAccountResponse) -> Self {
        value.clone()
    }
}
#[doc = "DtoPublicAccountResponse"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"did\","]
#[doc = "    \"username\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"did\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"profile_picture\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"username\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoPublicAccountResponse {
    pub did: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_picture: Option<String>,
    pub username: String,
}
impl From<&DtoPublicAccountResponse> for DtoPublicAccountResponse {
    fn from(value: &DtoPublicAccountResponse) -> Self {
        value.clone()
    }
}
#[doc = "DtoPublicAcl"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"address\","]
#[doc = "    \"data_asset_id\","]
#[doc = "    \"roles\","]
#[doc = "    \"solana_address\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"address\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"created_at\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"data_asset_id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"did\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"is_authority\": {"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"roles\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"solana_address\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"updated_at\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoPublicAcl {
    pub address: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    pub data_asset_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub did: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_authority: Option<bool>,
    pub roles: Vec<String>,
    pub solana_address: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}
impl From<&DtoPublicAcl> for DtoPublicAcl {
    fn from(value: &DtoPublicAcl) -> Self {
        value.clone()
    }
}
#[doc = "DtoPublicDataAsset"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"acl\","]
#[doc = "    \"created_by\","]
#[doc = "    \"fid\","]
#[doc = "    \"id\","]
#[doc = "    \"name\","]
#[doc = "    \"size\","]
#[doc = "    \"tags\","]
#[doc = "    \"transaction_id\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"acl\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/dto.PublicACL\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"created_at\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"created_by\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"data_model_id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"expiration_date\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"fid\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"size\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"tags\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"transaction_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"updated_at\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoPublicDataAsset {
    pub acl: Vec<DtoPublicAcl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    pub created_by: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_model_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    pub fid: String,
    pub id: i64,
    pub name: String,
    pub size: i64,
    pub tags: Vec<String>,
    pub transaction_id: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}
impl From<&DtoPublicDataAsset> for DtoPublicDataAsset {
    fn from(value: &DtoPublicDataAsset) -> Self {
        value.clone()
    }
}
#[doc = "DtoShareDataAssetRequest"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"addresses\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"addresses\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoShareDataAssetRequest {
    pub addresses: Vec<String>,
}
impl From<&DtoShareDataAssetRequest> for DtoShareDataAssetRequest {
    fn from(value: &DtoShareDataAssetRequest) -> Self {
        value.clone()
    }
}
#[doc = "DtoTokenResponse"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"token\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"token\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoTokenResponse {
    pub token: String,
}
impl From<&DtoTokenResponse> for DtoTokenResponse {
    fn from(value: &DtoTokenResponse) -> Self {
        value.clone()
    }
}
#[doc = "DtoUpdateDataAssetRequest"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"claim\": {"]
#[doc = "      \"type\": \"object\""]
#[doc = "    },"]
#[doc = "    \"expiration_date\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoUpdateDataAssetRequest {
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub claim: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl From<&DtoUpdateDataAssetRequest> for DtoUpdateDataAssetRequest {
    fn from(value: &DtoUpdateDataAssetRequest) -> Self {
        value.clone()
    }
}
#[doc = "DtoWalletCreateRequest"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"address\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"address\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DtoWalletCreateRequest {
    pub address: String,
}
impl From<&DtoWalletCreateRequest> for DtoWalletCreateRequest {
    fn from(value: &DtoWalletCreateRequest) -> Self {
        value.clone()
    }
}
#[doc = "HelperLinks"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"first\","]
#[doc = "    \"last\","]
#[doc = "    \"next\","]
#[doc = "    \"previous\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"first\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"last\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"next\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"previous\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HelperLinks {
    pub first: String,
    pub last: String,
    pub next: String,
    pub previous: String,
}
impl From<&HelperLinks> for HelperLinks {
    fn from(value: &HelperLinks) -> Self {
        value.clone()
    }
}
#[doc = "HelperMeta"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"current_page\","]
#[doc = "    \"items_per_page\","]
#[doc = "    \"total_items\","]
#[doc = "    \"total_pages\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"current_page\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"items_per_page\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"total_items\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"total_pages\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HelperMeta {
    pub current_page: i64,
    pub items_per_page: i64,
    pub total_items: i64,
    pub total_pages: i64,
}
impl From<&HelperMeta> for HelperMeta {
    fn from(value: &HelperMeta) -> Self {
        value.clone()
    }
}
#[doc = "HelperPaginatedResponse"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"data\","]
#[doc = "    \"links\","]
#[doc = "    \"meta\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"data\": {},"]
#[doc = "    \"links\": {"]
#[doc = "      \"$ref\": \"#/definitions/helper.Links\""]
#[doc = "    },"]
#[doc = "    \"meta\": {"]
#[doc = "      \"$ref\": \"#/definitions/helper.Meta\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HelperPaginatedResponse {
    pub data: serde_json::Value,
    pub links: HelperLinks,
    pub meta: HelperMeta,
}
impl From<&HelperPaginatedResponse> for HelperPaginatedResponse {
    fn from(value: &HelperPaginatedResponse) -> Self {
        value.clone()
    }
}
#[doc = "ModelWalletAddress"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"account_id\","]
#[doc = "    \"address\","]
#[doc = "    \"chain\","]
#[doc = "    \"created_at\","]
#[doc = "    \"id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"account_id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"address\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"chain\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"created_at\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"updated_at\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ModelWalletAddress {
    pub account_id: i64,
    pub address: String,
    pub chain: String,
    pub created_at: String,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}
impl From<&ModelWalletAddress> for ModelWalletAddress {
    fn from(value: &ModelWalletAddress) -> Self {
        value.clone()
    }
}
#[doc = "ResponsesMessageResponse"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"message\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"message\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResponsesMessageResponse {
    pub message: String,
}
impl From<&ResponsesMessageResponse> for ResponsesMessageResponse {
    fn from(value: &ResponsesMessageResponse) -> Self {
        value.clone()
    }
}
#[doc = "TypesAccessLevel"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"view\","]
#[doc = "    \"update\","]
#[doc = "    \"delete\","]
#[doc = "    \"share\""]
#[doc = "  ],"]
#[doc = "  \"x-enum-varnames\": ["]
#[doc = "    \"RoleView\","]
#[doc = "    \"RoleUpdate\","]
#[doc = "    \"RoleDelete\","]
#[doc = "    \"RoleShare\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum TypesAccessLevel {
    #[serde(rename = "view")]
    View,
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "share")]
    Share,
}
impl From<&TypesAccessLevel> for TypesAccessLevel {
    fn from(value: &TypesAccessLevel) -> Self {
        value.clone()
    }
}
impl ToString for TypesAccessLevel {
    fn to_string(&self) -> String {
        match *self {
            Self::View => "view".to_string(),
            Self::Update => "update".to_string(),
            Self::Delete => "delete".to_string(),
            Self::Share => "share".to_string(),
        }
    }
}
impl std::str::FromStr for TypesAccessLevel {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "view" => Ok(Self::View),
            "update" => Ok(Self::Update),
            "delete" => Ok(Self::Delete),
            "share" => Ok(Self::Share),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for TypesAccessLevel {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TypesAccessLevel {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TypesAccessLevel {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "TypesComputeOperation"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"add\","]
#[doc = "    \"subtract\","]
#[doc = "    \"multiply\","]
#[doc = "    \"divide\","]
#[doc = "    \"sum\","]
#[doc = "    \"greater_than\","]
#[doc = "    \"greater_than_or_equal\","]
#[doc = "    \"less_than\","]
#[doc = "    \"equal\","]
#[doc = "    \"not_equal\""]
#[doc = "  ],"]
#[doc = "  \"x-enum-varnames\": ["]
#[doc = "    \"ComputeOperationAdd\","]
#[doc = "    \"ComputeOperationSubtract\","]
#[doc = "    \"ComputeOperationMultiply\","]
#[doc = "    \"ComputeOperationDivide\","]
#[doc = "    \"ComputeOperationSum\","]
#[doc = "    \"ComputeOperationGreaterThan\","]
#[doc = "    \"ComputeOperationGreaterThanOrEqual\","]
#[doc = "    \"ComputeOperationLessThan\","]
#[doc = "    \"ComputeOperationEqual\","]
#[doc = "    \"ComputeOperationNotEqual\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum TypesComputeOperation {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "subtract")]
    Subtract,
    #[serde(rename = "multiply")]
    Multiply,
    #[serde(rename = "divide")]
    Divide,
    #[serde(rename = "sum")]
    Sum,
    #[serde(rename = "greater_than")]
    GreaterThan,
    #[serde(rename = "greater_than_or_equal")]
    GreaterThanOrEqual,
    #[serde(rename = "less_than")]
    LessThan,
    #[serde(rename = "equal")]
    Equal,
    #[serde(rename = "not_equal")]
    NotEqual,
}
impl From<&TypesComputeOperation> for TypesComputeOperation {
    fn from(value: &TypesComputeOperation) -> Self {
        value.clone()
    }
}
impl ToString for TypesComputeOperation {
    fn to_string(&self) -> String {
        match *self {
            Self::Add => "add".to_string(),
            Self::Subtract => "subtract".to_string(),
            Self::Multiply => "multiply".to_string(),
            Self::Divide => "divide".to_string(),
            Self::Sum => "sum".to_string(),
            Self::GreaterThan => "greater_than".to_string(),
            Self::GreaterThanOrEqual => "greater_than_or_equal".to_string(),
            Self::LessThan => "less_than".to_string(),
            Self::Equal => "equal".to_string(),
            Self::NotEqual => "not_equal".to_string(),
        }
    }
}
impl std::str::FromStr for TypesComputeOperation {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "add" => Ok(Self::Add),
            "subtract" => Ok(Self::Subtract),
            "multiply" => Ok(Self::Multiply),
            "divide" => Ok(Self::Divide),
            "sum" => Ok(Self::Sum),
            "greater_than" => Ok(Self::GreaterThan),
            "greater_than_or_equal" => Ok(Self::GreaterThanOrEqual),
            "less_than" => Ok(Self::LessThan),
            "equal" => Ok(Self::Equal),
            "not_equal" => Ok(Self::NotEqual),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for TypesComputeOperation {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TypesComputeOperation {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TypesComputeOperation {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
pub mod helper_generic_paginated_response;
