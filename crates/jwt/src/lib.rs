pub mod v2;

use std::future::Future;

use base64::{engine::general_purpose, Engine as _};
use dids::{
    bearer::{BearerDid, BearerDidError},
    document::KeySelector,
};
use jws::{sign_jws, Header, JwsError, JwsSignOptions, JwsString};
use serde::{Deserialize, Serialize};
use serde_json::{from_slice, to_string, Value};

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum JwtError {
    #[error(transparent)]
    BearerDidError(#[from] BearerDidError),
    #[error(transparent)]
    JwsError(#[from] JwsError),
    #[error("serialization error {0}")]
    SerializationError(String),
    #[error("deserialization error {0}")]
    DeserializationError(String),
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Claims {
    /// The "iss" (issuer) claim identifies the principal that issued the JWT.
    /// Spec: https://datatracker.ietf.org/doc/html/rfc7519#section-4.1.1
    #[serde(rename = "iss", skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,

    /// The "sub" (subject) claim identifies the principal that is the subject of the JWT.
    /// Spec: https://datatracker.ietf.org/doc/html/rfc7519#section-4.1.2
    #[serde(rename = "sub", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,

    /// The "aud" (audience) claim identifies the recipients that the JWT is intended for.
    /// Spec: https://datatracker.ietf.org/doc/html/rfc7519#section-4.1.3
    #[serde(rename = "aud", skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,

    /// The "exp" (expiration time) claim identifies the expiration time on or after which the JWT must not be accepted for processing.
    /// Spec: https://datatracker.ietf.org/doc/html/rfc7519#section-4.1.4
    #[serde(rename = "exp", skip_serializing_if = "Option::is_none")]
    pub expiration: Option<i64>,

    /// The "nbf" (not before) claim identifies the time before which the JWT must not be accepted for processing.
    /// Spec: https://datatracker.ietf.org/doc/html/rfc7519#section-4.1.5
    #[serde(rename = "nbf", skip_serializing_if = "Option::is_none")]
    pub not_before: Option<i64>,

    /// The "iat" (issued at) claim identifies the time at which the JWT was issued.
    /// Spec: https://datatracker.ietf.org/doc/html/rfc7519#section-4.1.6
    #[serde(rename = "iat", skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<i64>,

    /// The "jti" (JWT ID) claim provides a unique identifier for the JWT.
    /// Spec: https://datatracker.ietf.org/doc/html/rfc7519#section-4.1.7
    #[serde(rename = "jti", skip_serializing_if = "Option::is_none")]
    pub jti: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vc: Option<Value>,
}

impl Claims {
    pub fn encode(&self) -> Result<String, JwtError> {
        let json_str = to_string(&self).map_err(|e| JwtError::SerializationError(e.to_string()))?;
        let encoded_str = general_purpose::URL_SAFE_NO_PAD.encode(json_str.as_bytes());
        Ok(encoded_str)
    }

    pub fn decode(jwt_claims: String) -> Result<Self, JwtError> {
        let decoded_bytes = general_purpose::URL_SAFE_NO_PAD
            .decode(jwt_claims)
            .map_err(|e| JwtError::DeserializationError(e.to_string()))?;
        let claims = from_slice(&decoded_bytes)
            .map_err(|e| JwtError::DeserializationError(e.to_string()))?;
        Ok(claims)
    }
}

pub struct Decoded {
    pub header: Header,
    pub claims: Claims,
    pub signature: Vec<u8>,
    pub parts: Vec<String>,
}

pub trait JwtString {
    fn decode(&self) -> Result<Decoded, JwtError>;
    fn verify(&self) -> impl Future<Output = Result<Decoded, JwtError>>;
}

impl JwtString for String {
    fn decode(&self) -> Result<Decoded, JwtError> {
        let parts: Vec<&str> = self.split('.').collect();
        if parts.len() != 3 {
            return Err(JwtError::DeserializationError(
                "incorrect number of segments".to_string(),
            ));
        }

        let decoded_jws = JwsString::decode(self)?;
        let decoded_claims = Claims::decode(parts[1].to_string())?;

        Ok(Decoded {
            header: decoded_jws.header,
            claims: decoded_claims,
            signature: decoded_jws.signature,
            parts: decoded_jws.parts,
        })
    }

    fn verify(&self) -> impl Future<Output = Result<Decoded, JwtError>> {
        async move {
            let decoded = JwtString::decode(self)?;
            JwsString::verify(self).await?;
            Ok(decoded)
        }
    }
}

pub fn sign_jwt(
    bearer_did: &BearerDid,
    key_selector: &KeySelector,
    claims: &Claims,
    options: Option<JwsSignOptions>,
) -> Result<String, JwtError> {
    let encoded_claims = claims.encode()?;
    let jwt = sign_jws(
        bearer_did,
        key_selector,
        encoded_claims,
        options.unwrap_or_default(),
    )?;
    Ok(jwt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto::Curve;
    use dids::{
        document::VerificationMethodType,
        method::{
            jwk::{DidJwk, DidJwkCreateOptions},
            Method,
        },
    };
    use keys::key_manager::local_key_manager::LocalKeyManager;
    use std::sync::Arc;

    #[test]
    fn test_sign() {
        let key_manager = Arc::new(LocalKeyManager::new_in_memory());
        let bearer_did = DidJwk::create(
            key_manager,
            DidJwkCreateOptions {
                curve: Curve::Secp256k1,
            },
        )
        .unwrap();

        let claims = Claims {
            issuer: Some(bearer_did.identifier.uri.clone()),
            ..Default::default()
        };

        let jwt = sign_jwt(
            &bearer_did,
            &KeySelector::MethodType {
                verification_method_type: VerificationMethodType::VerificationMethod,
            },
            &claims,
            None,
        )
        .unwrap();

        println!("Signed JWT: {:?}", jwt);
    }
}
