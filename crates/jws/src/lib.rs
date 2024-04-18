use std::future::Future;

use base64::{engine::general_purpose, Engine as _};
use crypto::{ed25519::Ed25199, secp256k1::Secp256k1, CryptoError, CurveOperations};
use dids::{
    bearer::{BearerDid, BearerDidError},
    document::{DocumentError, KeyIdFragment, KeySelector},
    resolver::{ResolutionError, Resolver},
};
use serde::{Deserialize, Serialize};
use serde_json::{from_slice, to_string};

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum JwsError {
    #[error(transparent)]
    BearerDidError(#[from] BearerDidError),
    #[error("serialization error {0}")]
    SerializationError(String),
    #[error("deserialization error {0}")]
    DeserializationError(String),
    #[error("algorithm not found {0}")]
    AlgorithmNotFound(String),
    #[error(transparent)]
    DocumentError(#[from] DocumentError),
    #[error(transparent)]
    CryptoError(#[from] CryptoError),
    #[error(transparent)]
    ResolutionError(#[from] ResolutionError),
}

/// Represents a JWS (JSON Web Signature) header. See [Specification] for more details.
/// [Specification]: https://datatracker.ietf.org/doc/html/rfc7515#section-4
#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    /// Identifies the cryptographic algorithm used to secure the JWS. The JWS Signature value is not
    /// valid if the "alg" value does not represent a supported algorithm or if there is not a key for
    /// use with that algorithm associated with the party that digitally signed or MACed the content.
    ///
    /// "alg" values should either be registered in the IANA "JSON Web Signature and Encryption
    /// Algorithms" registry or be a value that contains a Collision-Resistant Name. The "alg" value is
    /// a case-sensitive ASCII string. This Header Parameter MUST be present and MUST be understood
    /// and processed by implementations.
    ///
    /// [Specification]: https://datatracker.ietf.org/doc/html/rfc7515#section-4.1.1
    pub alg: String,

    /// Key ID Header Parameter
    /// [Specification]: https://datatracker.ietf.org/doc/html/rfc7515#section-4.1.4
    pub kid: String,

    /// Type Header Parameter
    /// [Specification]: https://datatracker.ietf.org/doc/html/rfc7515#section-4.1.9
    pub typ: String,
}

impl Header {
    pub fn encode(&self) -> Result<String, JwsError> {
        let json_str = to_string(&self).map_err(|e| JwsError::SerializationError(e.to_string()))?;
        let encoded_str = general_purpose::URL_SAFE_NO_PAD.encode(json_str.as_bytes());
        Ok(encoded_str)
    }

    pub fn decode(jws_header: String) -> Result<Self, JwsError> {
        let decoded_bytes = general_purpose::URL_SAFE_NO_PAD
            .decode(jws_header)
            .map_err(|e| JwsError::DeserializationError(e.to_string()))?;
        let header = from_slice(&decoded_bytes)
            .map_err(|e| JwsError::DeserializationError(e.to_string()))?;
        Ok(header)
    }
}

pub struct Decoded {
    pub header: Header,
    pub payload: Vec<u8>,
    pub signature: Vec<u8>,
    pub parts: Vec<String>,
}

pub trait JwsString {
    fn decode(&self) -> Result<Decoded, JwsError>;
    fn verify(&self) -> impl Future<Output = Result<Decoded, JwsError>>;
}

impl JwsString for String {
    fn decode(&self) -> Result<Decoded, JwsError> {
        let parts: Vec<&str> = self.split('.').collect();
        if parts.len() != 3 {
            return Err(JwsError::DeserializationError(
                "incorrect number of segments".to_string(),
            ));
        }

        let header = Header::decode(parts[0].to_string())?;
        let payload = general_purpose::URL_SAFE_NO_PAD
            .decode(parts[1])
            .map_err(|e| JwsError::DeserializationError(e.to_string()))?;
        let signature = general_purpose::URL_SAFE_NO_PAD
            .decode(parts[2])
            .map_err(|e| JwsError::DeserializationError(e.to_string()))?;

        Ok(Decoded {
            header,
            payload,
            signature,
            parts: parts.iter().map(|s| s.to_string()).collect(),
        })
    }

    fn verify(&self) -> impl Future<Output = Result<Decoded, JwsError>> {
        async move {
            let decoded = self.decode()?;
            let key_id = decoded.header.kid.clone();
            let did_uri = KeyIdFragment(key_id.clone()).splice_uri();
            let resolution_result = Resolver::resolve_uri(&did_uri).await;
            if let Some(err) = resolution_result.did_resolution_metadata.error {
                return Err(JwsError::ResolutionError(err));
            }
            let verification_method = match resolution_result.did_document {
                Some(document) => document.get_verification_method(&KeySelector::KeyId { key_id }),
                None => {
                    return Err(JwsError::DocumentError(
                        DocumentError::VerificationMethodNotFound,
                    ))
                }
            }?;
            let public_key = verification_method.public_key_jwk.clone();
            let to_verify = format!("{}.{}", decoded.parts[0], decoded.parts[1]);
            let alg = decoded.header.alg.clone();
            match alg.as_str() {
                "EdDSA" => {
                    Ed25199::verify(&public_key, &to_verify.into_bytes(), &decoded.signature)
                }
                "ES256K" => {
                    Secp256k1::verify(&public_key, &to_verify.into_bytes(), &decoded.signature)
                }
                _ => return Err(JwsError::AlgorithmNotFound(alg)),
            }?;

            Ok(decoded)
        }
    }
}

#[derive(Default)]
pub struct JwsSignOptions {
    pub r#type: Option<String>,
}

pub fn sign_jws(
    bearer_did: &BearerDid,
    key_selector: &KeySelector,
    encoded_payload: String,
    options: JwsSignOptions,
) -> Result<String, JwsError> {
    let verification_method = bearer_did.document.get_verification_method(key_selector)?;

    let kid = verification_method.id;
    let alg = match verification_method.public_key_jwk.crv.as_str() {
        "secp256k1" => "ES256K".to_string(),
        "Ed25519" => "EdDSA".to_string(),
        _ => return Err(JwsError::AlgorithmNotFound(kid)),
    };
    let typ = options.r#type.unwrap_or_else(|| "JWT".to_string());
    let header = Header { alg, kid, typ };
    let encoded_header = header.encode()?;
    let to_sign = format!("{}.{}", encoded_header, encoded_payload);

    let signature = bearer_did.sign(key_selector, &to_sign.into_bytes())?;
    let encoded_signature = general_purpose::URL_SAFE_NO_PAD.encode(signature);

    let jws_token = format!(
        "{}.{}.{}",
        encoded_header, encoded_payload, encoded_signature
    );
    Ok(jws_token)
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
    fn header_encode_success() {
        let header = Header {
            alg: "ES256K".to_string(),
            kid: "key1".to_string(),
            typ: "JWT".to_string(),
        };
        let encoded = header.encode().unwrap();
        assert!(!encoded.is_empty());
    }

    #[test]
    fn header_decode_success() {
        let header = Header {
            alg: "ES256K".to_string(),
            kid: "key1".to_string(),
            typ: "JWT".to_string(),
        };
        let encoded = header.encode().unwrap();
        let decoded = Header::decode(encoded).unwrap();
        assert_eq!(decoded.alg, "ES256K");
        assert_eq!(decoded.kid, "key1");
        assert_eq!(decoded.typ, "JWT");
    }

    #[test]
    fn header_decode_failure() {
        let result = Header::decode("invalid_base64".to_string());
        assert!(result.is_err());
        match result.unwrap_err() {
            JwsError::DeserializationError(e) => assert!(!e.is_empty()),
            _ => panic!("Expected deserialization error"),
        }
    }

    #[tokio::test]
    async fn sign_jws_success() {
        let key_manager = Arc::new(LocalKeyManager::new_in_memory());
        let options = DidJwkCreateOptions {
            curve: Curve::Ed25519,
        };
        let bearer_did = DidJwk::create(key_manager, options).unwrap();
        let key_selector = KeySelector::MethodType {
            verification_method_type: VerificationMethodType::VerificationMethod,
        };
        let encoded_payload = general_purpose::URL_SAFE_NO_PAD.encode(b"some payload test");
        let options = JwsSignOptions {
            r#type: Some("JWT".to_string()),
        };

        let signed = sign_jws(&bearer_did, &key_selector, encoded_payload, options);
        assert!(signed.is_ok());

        let jws = signed.unwrap();
        let result = jws.verify().await;
        assert!(result.is_ok());
    }
}
