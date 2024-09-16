use serde::{Deserialize, Serialize};

/// Metadata about a DID document as per the [W3C DID Core specification](https://www.w3.org/TR/did-core/).
///
/// This struct provides additional information about a resolved DID document, including timestamps,
/// versioning, and deactivation status. It helps track changes and updates to the DID document
/// over time.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DocumentMetadata {
    /// The timestamp when the DID document was created (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,

    /// The timestamp when the DID document was last updated (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,

    /// Indicates whether the DID document has been deactivated (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deactivated: Option<bool>,

    /// The timestamp for the next expected update of the DID document (optional).
    #[serde(rename = "nextUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_update: Option<String>,

    /// The version identifier of the current DID document (optional).
    #[serde(rename = "versionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,

    /// The version identifier of the next expected version of the DID document (optional).
    #[serde(rename = "nextVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_version_id: Option<String>,

    /// A list of equivalent IDs for the DID document (optional).
    #[serde(rename = "equivalentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equivalent_id: Option<Vec<String>>,

    /// The canonical ID for the DID document, if applicable (optional).
    #[serde(rename = "canonicalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_id: Option<String>,
}
