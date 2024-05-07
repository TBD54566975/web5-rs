use dids::document::DocumentError;
use jws::v2::JwsError;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Error as SerdeJsonError;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum JwtError {
    #[error(transparent)]
    JwsError(#[from] JwsError),
    #[error(transparent)]
    DocumentError(#[from] DocumentError),
    #[error("serde json error {0}")]
    SerdeJsonError(String),
}

impl From<SerdeJsonError> for JwtError {
    fn from(err: SerdeJsonError) -> Self {
        JwtError::SerdeJsonError(err.to_string())
    }
}

pub trait Claims: Serialize + DeserializeOwned {}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BaseClaims {
    #[serde(rename = "iss", skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(rename = "sub", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(rename = "aud", skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[serde(rename = "exp", skip_serializing_if = "Option::is_none")]
    pub expiration: Option<i64>,
    #[serde(rename = "nbf", skip_serializing_if = "Option::is_none")]
    pub not_before: Option<i64>,
    #[serde(rename = "iat", skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<i64>,
    #[serde(rename = "jti", skip_serializing_if = "Option::is_none")]
    pub jti: Option<String>,
}

impl Claims for BaseClaims {}
