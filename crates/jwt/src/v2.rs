use base64::{engine::general_purpose, Engine as _};
use dids::{bearer::BearerDid, document::KeySelector};
use jws::v2::{sign_compact_jws, verify_compact_jws, JwsError, JwsHeader};
use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum JwtError {
    #[error(transparent)]
    JwsError(#[from] JwsError),
    #[error("serialization error {0}")]
    SerializationError(String),
    #[error("deserialization error {0}")]
    DeserializationError(String),
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Claims {
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

impl Claims {
    pub fn new(
        issuer: Option<String>,
        subject: Option<String>,
        audience: Option<String>,
        expiration: Option<i64>,
        not_before: Option<i64>,
        issued_at: Option<i64>,
        jti: Option<String>,
    ) -> Self {
        Self {
            issuer,
            subject,
            audience,
            expiration,
            not_before,
            issued_at,
            jti,
        }
    }

    // todo uniffi will want a constructor so might we well make props private w/ getters only
    // todo this might be a good use case for dictionary's

    pub fn encode(&self) -> Result<String, JwtError> {
        let json_str = serde_json::to_string(&self)
            .map_err(|e| JwtError::SerializationError(e.to_string()))?;
        let encoded_str = general_purpose::URL_SAFE_NO_PAD.encode(json_str.as_bytes());
        Ok(encoded_str)
    }

    pub fn sign(
        &self,
        bearer_did: &BearerDid,
        key_selector: &KeySelector,
    ) -> Result<String, JwtError> {
        let encoded_payload = self.encode()?;
        let jws_header = JwsHeader::from_bearer_did(bearer_did, key_selector, "JWT")?;
        let compact_jws =
            jws_header.sign_compact_jws(bearer_did, key_selector, &encoded_payload)?;
        Ok(compact_jws)
    }
}

pub fn sign_jwt(
    bearer_did: &BearerDid,
    key_selector: &KeySelector,
    encoded_header: &str,
    encoded_payload: &str,
) -> Result<String, JwtError> {
    let compact_jws = sign_compact_jws(bearer_did, key_selector, encoded_header, encoded_payload)?;
    Ok(compact_jws)
}

pub async fn verify_jwt(jwt: &str) -> Result<(), JwtError> {
    let _ = verify_compact_jws(jwt).await?;
    Ok(())
}
