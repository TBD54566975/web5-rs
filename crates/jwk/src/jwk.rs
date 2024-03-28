use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use base64::{Engine as _, engine::general_purpose};
use std::fmt;

#[derive(Debug, uniffi::Error)]
pub enum JWKError {
    SerializationError { errorMessage: String },
}

impl fmt::Display for JWKError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JWKError::SerializationError { errorMessage } => write!(f, "Serialization error: {}", errorMessage),
        }
    }
}

impl std::error::Error for JWKError {}

#[derive(Serialize, Deserialize, Debug, Clone, uniffi::Record)]
pub struct JWK {
    #[serde(rename = "alg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alg: Option<String>,

    #[serde(rename = "kty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kty: Option<String>,

    #[serde(rename = "crv")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crv: Option<String>,

    #[serde(rename = "d")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d: Option<String>,

    #[serde(rename = "x")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<String>,

    #[serde(rename = "y")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<String>,
}

#[uniffi::export]
fn new_jwk() -> JWK {
    JWK {
        alg: None,
        kty: None,
        crv: None,
        d: None,
        x: None,
        y: None,
    }
}

#[uniffi::export]
fn compute_thumbprint(jwk: &JWK) -> Result<String, JWKError> {
    let mut thumbprint_payload = serde_json::json!({
        "crv": jwk.crv,
        "kty": jwk.kty,
        "x": jwk.x,
    });

    if let Some(y) = &jwk.y {
        thumbprint_payload["y"] = serde_json::Value::String(y.to_string());
    }

    let bytes = serde_json::to_vec(&thumbprint_payload)
        .map_err(|err| JWKError::SerializationError { errorMessage: format!("Failed to serialize thumbprint payload: {}", err) })?;

    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let digest = hasher.finalize();

    let thumbprint = general_purpose::URL_SAFE_NO_PAD.encode(&digest);

    Ok(thumbprint)
}