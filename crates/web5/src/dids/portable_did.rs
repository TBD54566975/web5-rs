use super::data_model::document::Document;
use crate::crypto::jwk::Jwk;
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeJsonError;

#[derive(Serialize, Deserialize, Clone)]
pub struct PortableDid {
    #[serde(rename = "uri")]
    pub did_uri: String,
    pub document: Document,
    #[serde(rename = "privateKeys")]
    pub private_jwks: Vec<Jwk>,
}

#[derive(thiserror::Error, Debug)]
pub enum PortableDidError {
    #[error("serde json error {0}")]
    SerdeJsonError(String),
}

impl From<SerdeJsonError> for PortableDidError {
    fn from(err: SerdeJsonError) -> Self {
        PortableDidError::SerdeJsonError(err.to_string())
    }
}

type Result<T> = std::result::Result<T, PortableDidError>;

impl PortableDid {
    pub fn new(json: &str) -> Result<Self> {
        let portable_did = serde_json::from_str::<Self>(json)?;
        Ok(portable_did)
    }
}
