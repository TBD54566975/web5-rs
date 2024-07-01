use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DocumentMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deactivated: Option<bool>,
    #[serde(rename = "nextUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_update: Option<String>,
    #[serde(rename = "versionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    #[serde(rename = "nextVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_version_id: Option<String>,
    #[serde(rename = "equivalentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equivalent_id: Option<Vec<String>>,
    #[serde(rename = "canonicalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_id: Option<String>,
}
