use base64::{engine::general_purpose, DecodeError, Engine as _};
use crypto::{ed25519::Ed25519, secp256k1::Secp256k1, CryptoError, CurveOperations};
use dids::{
    bearer::{BearerDid, BearerDidError},
    document::{DocumentError, KeyIdFragment, KeySelector},
    resolver::{ResolutionError, Resolver},
};
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeJsonError;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum JwsError {
    #[error(transparent)]
    BearerDidError(#[from] BearerDidError),
    #[error("incorrect number of parts 3 expected {0}")]
    IncorrectPartsLength(String),
    #[error(transparent)]
    DocumentError(#[from] DocumentError),
    #[error(transparent)]
    ResolutionError(#[from] ResolutionError),
    #[error("algorithm not found {0}")]
    AlgorithmNotSupported(String),
    #[error(transparent)]
    CryptoError(#[from] CryptoError),
    #[error("serde json error {0}")]
    SerdeJsonError(String),
    #[error(transparent)]
    DecodeError(#[from] DecodeError),
    #[error("Malformed Header: {0}")]
    MalformedHeader(String),
}

impl From<SerdeJsonError> for JwsError {
    fn from(err: SerdeJsonError) -> Self {
        JwsError::SerdeJsonError(err.to_string())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct JwsHeader {
    pub alg: String,
    pub kid: String,
    pub typ: String,
}

pub struct JwsDecoded {
    pub header: JwsHeader,
    pub payload: Vec<u8>,
    pub signature: String,
    pub parts: Vec<String>,
}

pub struct CompactJws;

impl CompactJws {
    pub fn sign(
        bearer_did: &BearerDid,
        key_selector: &KeySelector,
        header: &JwsHeader,
        payload: &[u8], // JSON string as a byte array, TODO add a doc comment for this
    ) -> Result<String, JwsError> {
        let header_json_string = serde_json::to_string(header)?;
        let encoded_header = general_purpose::URL_SAFE_NO_PAD.encode(header_json_string.as_bytes());
        let encoded_payload = general_purpose::URL_SAFE_NO_PAD.encode(payload);

        let to_sign = format!("{}.{}", encoded_header, encoded_payload);
        let signature = bearer_did.sign(key_selector, &to_sign.into_bytes())?;
        let encoded_signature = general_purpose::URL_SAFE_NO_PAD.encode(signature);
        let compact_jws = format!(
            "{}.{}.{}",
            encoded_header, encoded_payload, encoded_signature
        );
        Ok(compact_jws)
    }

    pub fn decode(compact_jws: &str) -> Result<JwsDecoded, JwsError> {
        let parts: Vec<String> = compact_jws.split('.').map(|x| x.to_string()).collect();
        if parts.len() != 3 {
            return Err(JwsError::IncorrectPartsLength(compact_jws.to_string()));
        }

        let decoded_header = general_purpose::URL_SAFE_NO_PAD.decode(&parts[0])?;
        let header = serde_json::from_slice::<JwsHeader>(&decoded_header)?;

        let decoded_payload = general_purpose::URL_SAFE_NO_PAD.decode(&parts[1])?;

        Ok(JwsDecoded {
            header,
            payload: decoded_payload,
            signature: parts[2].to_string(),
            parts,
        })
    }

    pub async fn verify(compact_jws: &str) -> Result<JwsDecoded, JwsError> {
        let jws_decoded = CompactJws::decode(compact_jws)?;

        // Validate header fields
        if jws_decoded.header.alg.is_empty() {
            return Err(JwsError::MalformedHeader(
                "alg field is required".to_string(),
            ));
        }

        if jws_decoded.header.kid.is_empty() {
            return Err(JwsError::MalformedHeader(
                "kid field is required for verification processing".to_string(),
            ));
        }

        let key_id = jws_decoded.header.kid.clone();
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
        let to_verify = format!("{}.{}", jws_decoded.parts[0], jws_decoded.parts[1]);
        let alg = jws_decoded.header.alg.clone();
        let decoded_signature = general_purpose::URL_SAFE_NO_PAD.decode(&jws_decoded.parts[2])?;
        match alg.as_str() {
            "EdDSA" => Ed25519::verify(&public_key, &to_verify.into_bytes(), &decoded_signature),
            "ES256K" => Secp256k1::verify(&public_key, &to_verify.into_bytes(), &decoded_signature),
            _ => return Err(JwsError::AlgorithmNotSupported(alg)),
        }?;
        Ok(jws_decoded)
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;
    use crypto::Curve;
    use dids::method::jwk::{DidJwk, DidJwkCreateOptions};
    use dids::method::Create;
    use keys::key_manager::local_key_manager::LocalKeyManager;
    use serde_json::json;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_jws_sign_and_verify() {
        let key_manager = LocalKeyManager::new_in_memory();
        let bearer_did = DidJwk::create(
            Arc::new(key_manager),
            DidJwkCreateOptions {
                curve: Curve::Ed25519,
            },
        )
        .expect("failed to create bearer did");

        let key_id = bearer_did.document.verification_method[0].id.clone();

        let header = JwsHeader {
            alg: "EdDSA".to_string(),
            kid: key_id.clone(),
            typ: "JWT".to_string(),
        };
        let payload = json!({
            "sub": "1234567890",
            "name": "John Doe",
            "iat": 1516239022
        })
        .to_string()
        .into_bytes();

        let compact_jws = CompactJws::sign(
            &bearer_did,
            &KeySelector::KeyId {
                key_id: key_id.clone(),
            },
            &header,
            &payload,
        )
        .unwrap();

        let verified_jws = CompactJws::verify(&compact_jws).await.unwrap();

        assert_eq!(verified_jws.header, header);
        assert_eq!(verified_jws.payload, payload);
    }

    #[test]
    fn test_jws_decode() {
        let key_manager = LocalKeyManager::new_in_memory();
        let bearer_did = DidJwk::create(
            Arc::new(key_manager),
            DidJwkCreateOptions {
                curve: Curve::Ed25519,
            },
        )
        .expect("failed to create bearer did");

        let key_id = bearer_did.document.verification_method[0].id.clone();

        let header = JwsHeader {
            alg: "EdDSA".to_string(),
            kid: key_id.clone(),
            typ: "JWT".to_string(),
        };
        let payload = json!({
            "sub": "1234567890",
            "name": "John Doe",
            "iat": 1516239022
        })
        .to_string()
        .into_bytes();

        let compact_jws = CompactJws::sign(
            &bearer_did,
            &KeySelector::KeyId {
                key_id: key_id.clone(),
            },
            &header,
            &payload,
        )
        .unwrap();

        let decoded_jws = CompactJws::decode(&compact_jws).unwrap();

        assert_eq!(decoded_jws.header, header);
        assert_eq!(decoded_jws.payload, payload);
        assert_eq!(decoded_jws.signature.is_empty(), false);
        assert_eq!(decoded_jws.parts.len(), 3);
    }

    #[test]
    fn test_jws_decode_incorrect_parts_error() {
        let invalid_jws = "invalid.jws";
        let result = CompactJws::decode(invalid_jws);
        assert!(matches!(result, Err(JwsError::IncorrectPartsLength(_))));
    }

    #[tokio::test]
    async fn test_jws_verify_malformed_header_alg_error() {
        let key_manager = LocalKeyManager::new_in_memory();
        let bearer_did = DidJwk::create(
            Arc::new(key_manager),
            DidJwkCreateOptions {
                curve: Curve::Ed25519,
            },
        )
        .expect("failed to create bearer did");

        let key_id = bearer_did.document.verification_method[0].id.clone();

        let header = JwsHeader {
            alg: "".to_string(),
            kid: key_id.clone(),
            typ: "JWT".to_string(),
        };
        let payload = json!({
            "sub": "1234567890",
            "name": "John Doe",
            "iat": 1516239022
        })
        .to_string()
        .into_bytes();

        let compact_jws = CompactJws::sign(
            &bearer_did,
            &KeySelector::KeyId {
                key_id: key_id.clone(),
            },
            &header,
            &payload,
        )
        .unwrap();

        let result = CompactJws::verify(&compact_jws).await;

        assert!(matches!(result, Err(JwsError::MalformedHeader(_))));
    }

    #[tokio::test]
    async fn test_jws_verify_malformed_header_kid_error() {
        let key_manager = LocalKeyManager::new_in_memory();
        let bearer_did = DidJwk::create(
            Arc::new(key_manager),
            DidJwkCreateOptions {
                curve: Curve::Ed25519,
            },
        )
        .expect("failed to create bearer did");

        let key_id = bearer_did.document.verification_method[0].id.clone();

        let header = JwsHeader {
            alg: "EdDSA".to_string(),
            kid: "".to_string(),
            typ: "JWT".to_string(),
        };
        let payload = json!({
            "sub": "1234567890",
            "name": "John Doe",
            "iat": 1516239022
        })
        .to_string()
        .into_bytes();

        let compact_jws = CompactJws::sign(
            &bearer_did,
            &KeySelector::KeyId {
                key_id: key_id.clone(),
            },
            &header,
            &payload,
        )
        .unwrap();

        let result = CompactJws::verify(&compact_jws).await;

        assert!(matches!(result, Err(JwsError::MalformedHeader(_))));
    }

    #[tokio::test]
    async fn test_jws_verify_algorithm_not_supported_error() {
        let key_manager = LocalKeyManager::new_in_memory();
        let bearer_did = DidJwk::create(
            Arc::new(key_manager),
            DidJwkCreateOptions {
                curve: Curve::Ed25519,
            },
        )
        .expect("failed to create bearer did");

        let key_id = bearer_did.document.verification_method[0].id.clone();

        let header = JwsHeader {
            alg: "UNSUPPORTED_ALG".to_string(),
            kid: key_id.clone(),
            typ: "JWT".to_string(),
        };
        let payload = json!({
            "sub": "1234567890",
            "name": "John Doe",
            "iat": 1516239022
        })
        .to_string()
        .into_bytes();

        let compact_jws = CompactJws::sign(
            &bearer_did,
            &KeySelector::KeyId {
                key_id: key_id.clone(),
            },
            &header,
            &payload,
        )
        .unwrap();

        let result = CompactJws::verify(&compact_jws).await;

        assert!(matches!(result, Err(JwsError::AlgorithmNotSupported(_))));
    }

    #[tokio::test]
    async fn test_jws_verify_resolution_error() {
        let key_manager = LocalKeyManager::new_in_memory();
        let bearer_did = DidJwk::create(
            Arc::new(key_manager),
            DidJwkCreateOptions {
                curve: Curve::Ed25519,
            },
        )
        .expect("failed to create bearer did");

        let key_id = bearer_did.document.verification_method[0].id.clone();

        let header = JwsHeader {
            alg: "EdDSA".to_string(),
            kid: "did:jwk:123#123".to_string(),
            typ: "JWT".to_string(),
        };
        let payload = json!({
            "sub": "1234567890",
            "name": "John Doe",
            "iat": 1516239022
        })
        .to_string()
        .into_bytes();

        let compact_jws = CompactJws::sign(
            &bearer_did,
            &KeySelector::KeyId {
                key_id: key_id.clone(),
            },
            &header,
            &payload,
        )
        .unwrap();

        let result = CompactJws::verify(&compact_jws).await;

        assert!(matches!(result, Err(JwsError::ResolutionError(_))));
    }

    #[tokio::test]
    async fn test_jws_verify_document_verification_method_error() {
        let key_manager = LocalKeyManager::new_in_memory();
        let bearer_did = DidJwk::create(
            Arc::new(key_manager),
            DidJwkCreateOptions {
                curve: Curve::Ed25519,
            },
        )
        .expect("failed to create bearer did");

        let key_id = bearer_did.document.verification_method[0].id.clone();

        let header = JwsHeader {
            alg: "EdDSA".to_string(),
            kid: key_id.clone() + "123",
            typ: "JWT".to_string(),
        };
        let payload = json!({
            "sub": "1234567890",
            "name": "John Doe",
            "iat": 1516239022
        })
        .to_string()
        .into_bytes();

        let compact_jws = CompactJws::sign(
            &bearer_did,
            &KeySelector::KeyId {
                key_id: key_id.clone(),
            },
            &header,
            &payload,
        )
        .unwrap();

        let result = CompactJws::verify(&compact_jws).await;

        assert!(matches!(result, Err(JwsError::DocumentError(_))));
    }

    // TODO https://github.com/TBD54566975/web5-rs/issues/166
    // - not base64 encoded signature
    // - base64 encoded signature but not valid cryptographic signature
}
