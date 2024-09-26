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
#[doc = "ModelAccessLevel"]
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
pub enum ModelAccessLevel {
    #[serde(rename = "view")]
    View,
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "share")]
    Share,
}
impl From<&ModelAccessLevel> for ModelAccessLevel {
    fn from(value: &ModelAccessLevel) -> Self {
        value.clone()
    }
}
impl ToString for ModelAccessLevel {
    fn to_string(&self) -> String {
        match *self {
            Self::View => "view".to_string(),
            Self::Update => "update".to_string(),
            Self::Delete => "delete".to_string(),
            Self::Share => "share".to_string(),
        }
    }
}
impl std::str::FromStr for ModelAccessLevel {
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
impl std::convert::TryFrom<&str> for ModelAccessLevel {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ModelAccessLevel {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ModelAccessLevel {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "ModelAccountCreateRequest"]
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
pub struct ModelAccountCreateRequest {
    pub message: String,
    pub signature: String,
    pub username: String,
    pub wallet_address: String,
}
impl From<&ModelAccountCreateRequest> for ModelAccountCreateRequest {
    fn from(value: &ModelAccountCreateRequest) -> Self {
        value.clone()
    }
}
#[doc = "ModelAccountUpdateRequest"]
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
pub struct ModelAccountUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_picture: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}
impl From<&ModelAccountUpdateRequest> for ModelAccountUpdateRequest {
    fn from(value: &ModelAccountUpdateRequest) -> Self {
        value.clone()
    }
}
#[doc = "ModelAclRequest"]
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
#[doc = "        \"$ref\": \"#/definitions/model.AccessLevel\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ModelAclRequest {
    pub address: String,
    pub roles: Vec<ModelAccessLevel>,
}
impl From<&ModelAclRequest> for ModelAclRequest {
    fn from(value: &ModelAclRequest) -> Self {
        value.clone()
    }
}
#[doc = "ModelAuthRequest"]
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
pub struct ModelAuthRequest {
    pub message: String,
    pub signature: String,
    pub wallet_address: String,
}
impl From<&ModelAuthRequest> for ModelAuthRequest {
    fn from(value: &ModelAuthRequest) -> Self {
        value.clone()
    }
}
#[doc = "ModelCreateDataAssetRequest"]
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
#[doc = "        \"$ref\": \"#/definitions/model.ACLRequest\""]
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
pub struct ModelCreateDataAssetRequest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub acl: Vec<ModelAclRequest>,
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
impl From<&ModelCreateDataAssetRequest> for ModelCreateDataAssetRequest {
    fn from(value: &ModelCreateDataAssetRequest) -> Self {
        value.clone()
    }
}
#[doc = "ModelDataAssetIdRequestAndResponse"]
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
pub struct ModelDataAssetIdRequestAndResponse {
    pub id: i64,
}
impl From<&ModelDataAssetIdRequestAndResponse> for ModelDataAssetIdRequestAndResponse {
    fn from(value: &ModelDataAssetIdRequestAndResponse) -> Self {
        value.clone()
    }
}
#[doc = "ModelDataModel"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"created_at\","]
#[doc = "    \"created_by\","]
#[doc = "    \"description\","]
#[doc = "    \"id\","]
#[doc = "    \"schema\","]
#[doc = "    \"title\","]
#[doc = "    \"updated_at\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"created_at\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"created_by\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"deleted_at\": {"]
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
pub struct ModelDataModel {
    pub created_at: String,
    pub created_by: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    pub description: String,
    pub id: i64,
    pub schema: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    pub title: String,
    pub updated_at: String,
}
impl From<&ModelDataModel> for ModelDataModel {
    fn from(value: &ModelDataModel) -> Self {
        value.clone()
    }
}
#[doc = "ModelDataModelRequest"]
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
pub struct ModelDataModelRequest {
    pub description: String,
    pub schema: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    pub title: String,
}
impl From<&ModelDataModelRequest> for ModelDataModelRequest {
    fn from(value: &ModelDataModelRequest) -> Self {
        value.clone()
    }
}
#[doc = "ModelMessageResponse"]
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
pub struct ModelMessageResponse {
    pub message: String,
}
impl From<&ModelMessageResponse> for ModelMessageResponse {
    fn from(value: &ModelMessageResponse) -> Self {
        value.clone()
    }
}
#[doc = "ModelMyAccountResponse"]
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
pub struct ModelMyAccountResponse {
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
impl From<&ModelMyAccountResponse> for ModelMyAccountResponse {
    fn from(value: &ModelMyAccountResponse) -> Self {
        value.clone()
    }
}
#[doc = "ModelPublicAcl"]
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
pub struct ModelPublicAcl {
    pub address: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    pub data_asset_id: i64,
    pub roles: Vec<String>,
    pub solana_address: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}
impl From<&ModelPublicAcl> for ModelPublicAcl {
    fn from(value: &ModelPublicAcl) -> Self {
        value.clone()
    }
}
#[doc = "ModelPublicDataAsset"]
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
#[doc = "        \"$ref\": \"#/definitions/model.PublicACL\""]
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
pub struct ModelPublicDataAsset {
    pub acl: Vec<ModelPublicAcl>,
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
impl From<&ModelPublicDataAsset> for ModelPublicDataAsset {
    fn from(value: &ModelPublicDataAsset) -> Self {
        value.clone()
    }
}
#[doc = "ModelShareDataAssetRequest"]
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
pub struct ModelShareDataAssetRequest {
    pub addresses: Vec<String>,
}
impl From<&ModelShareDataAssetRequest> for ModelShareDataAssetRequest {
    fn from(value: &ModelShareDataAssetRequest) -> Self {
        value.clone()
    }
}
#[doc = "ModelTokenResponse"]
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
pub struct ModelTokenResponse {
    pub token: String,
}
impl From<&ModelTokenResponse> for ModelTokenResponse {
    fn from(value: &ModelTokenResponse) -> Self {
        value.clone()
    }
}
#[doc = "ModelUpdateDataAssetRequest"]
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
pub struct ModelUpdateDataAssetRequest {
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub claim: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl From<&ModelUpdateDataAssetRequest> for ModelUpdateDataAssetRequest {
    fn from(value: &ModelUpdateDataAssetRequest) -> Self {
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
#[doc = "ModelWalletCreateRequest"]
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
pub struct ModelWalletCreateRequest {
    pub address: String,
}
impl From<&ModelWalletCreateRequest> for ModelWalletCreateRequest {
    fn from(value: &ModelWalletCreateRequest) -> Self {
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
pub mod helper_generic_paginated_response;
