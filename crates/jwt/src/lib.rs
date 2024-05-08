pub mod jwe;
pub mod jws;
pub mod lib_v2;

use ::jws::{sign_compact_jws, verify_compact_jws, JwsError, JwsHeader};
use base64::{engine::general_purpose, Engine as _};
use dids::{bearer::BearerDid, document::KeySelector};
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
    verify_compact_jws(jwt).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use ::jws::splice_parts;
    use crypto::Curve;
    use dids::{
        document::VerificationMethodType,
        method::{
            jwk::{DidJwk, DidJwkCreateOptions},
            Method,
        },
    };
    use keys::key_manager::local_key_manager::LocalKeyManager;

    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_encode() {
        let claims = Claims {
            issuer: Some("did:example:123".to_string()),
            ..Default::default()
        };
        let encoded = claims.encode().expect("failed to encode");
        assert_ne!(0, encoded.len());
    }

    #[test]
    fn test_sign() {
        let key_manager = Arc::new(LocalKeyManager::new_in_memory());
        let options = DidJwkCreateOptions {
            curve: Curve::Ed25519,
        };
        let bearer_did = DidJwk::create(key_manager, options).unwrap();
        let key_selector = KeySelector::MethodType {
            verification_method_type: VerificationMethodType::VerificationMethod,
        };

        let claims = Claims {
            issuer: Some("did:example:123".to_string()),
            ..Default::default()
        };
        let signed = claims
            .sign(&bearer_did, &key_selector)
            .expect("failed to sign jwt");
        assert!(signed.len() > 0);

        let encoded_header = splice_parts(&signed).unwrap()[0].clone();
        let encoded_payload = claims.encode().expect("failed to encode");
        let signed2 = sign_jwt(
            &bearer_did,
            &key_selector,
            &encoded_header,
            &encoded_payload,
        )
        .expect("failed to sign jwt");
        assert!(signed2.len() > 0);
    }

    #[tokio::test]
    async fn test_verify() {
        let jwt = "eyJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkVSVFFTSXNJbU55ZGlJNklrVmtNalUxTVRraUxDSnJkSGtpT2lKUFMxQWlMQ0o0SWpvaVpIaHlUemhwWjJOaFVuQlRaRlZ0Ylc5QlRXaG1TRE5uVmtOV1kxTkpaWEp6WjBaYU1YUnFYMTlOVlNKOSMwIiwidHlwIjoiSldUIn0.eyJpc3MiOiJkaWQ6ZXhhbXBsZToxMjMifQ.aGu5KNVmNV1o35cyksJ5A1uCbDp5Z1moROPGwnsxNfTKC9aPbmAJVICaE9dB2lU79vIuVTgVFrs_octfB_wvAg";
        let result = verify_jwt(jwt).await;
        assert!(result.is_ok());
    }
}
