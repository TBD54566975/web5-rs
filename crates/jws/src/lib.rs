use base64::{engine::general_purpose, Engine as _};
use crypto::{ed25519::Ed25199, secp256k1::Secp256k1, CryptoError, CurveOperations};
use dids::{
    bearer::{BearerDid, BearerDidError},
    document::{DocumentError, KeyIdFragment, KeySelector},
    resolver::{ResolutionError, Resolver},
};
use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum JwsError {
    #[error(transparent)]
    BearerDidError(#[from] BearerDidError),
    #[error("serialization error {0}")]
    SerializationError(String),
    #[error("deserialization error {0}")]
    DeserializationError(String),
    #[error("decoding error {0}")]
    DecodingError(String),
    #[error("incorrect number of parts 3 expected {0}")]
    IncorrectPartsLength(String),
    #[error(transparent)]
    DocumentError(#[from] DocumentError),
    #[error(transparent)]
    ResolutionError(#[from] ResolutionError),
    #[error("algorithm not found {0}")]
    AlgorithmNotFound(String),
    #[error(transparent)]
    CryptoError(#[from] CryptoError),
}

pub fn splice_parts(compact_jws: &str) -> Result<Vec<String>, JwsError> {
    let parts: Vec<String> = compact_jws.split('.').map(|x| x.to_string()).collect();
    if parts.len() != 3 {
        return Err(JwsError::IncorrectPartsLength(compact_jws.to_string()));
    }
    Ok(parts)
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct JwsHeader {
    pub alg: String,
    pub kid: String,
    pub typ: String,
}

impl JwsHeader {
    pub fn new(alg: String, kid: String, typ: String) -> Self {
        Self { alg, kid, typ }
    }

    pub fn new_from_encoded(encoded_jws_header: &str) -> Result<Self, JwsError> {
        let decoded_bytes = general_purpose::URL_SAFE_NO_PAD
            .decode(encoded_jws_header)
            .map_err(|e| JwsError::DecodingError(e.to_string()))?;
        let jws_header = serde_json::from_slice(&decoded_bytes)
            .map_err(|e| JwsError::DeserializationError(e.to_string()))?;
        Ok(jws_header)
    }

    pub fn from_bearer_did(
        bearer_did: &BearerDid,
        key_selector: &KeySelector,
        typ: &str,
    ) -> Result<Self, JwsError> {
        let verification_method = bearer_did.document.get_verification_method(key_selector)?;
        let kid = verification_method.id;
        let alg = match verification_method.public_key_jwk.crv.as_str() {
            "secp256k1" => "ES256K".to_string(),
            "Ed25519" => "EdDSA".to_string(),
            _ => return Err(JwsError::AlgorithmNotFound(kid)),
        };
        Ok(Self {
            alg,
            kid,
            typ: typ.to_string(),
        })
    }

    pub fn encode(&self) -> Result<String, JwsError> {
        let json_str = serde_json::to_string(&self)
            .map_err(|e| JwsError::SerializationError(e.to_string()))?;
        let encoded_str = general_purpose::URL_SAFE_NO_PAD.encode(json_str.as_bytes());
        Ok(encoded_str)
    }

    pub fn sign_compact_jws(
        &self,
        bearer_did: &BearerDid,
        key_selector: &KeySelector,
        encoded_payload: &str,
    ) -> Result<String, JwsError> {
        let encoded_header = self.encode()?;
        let compact_jws =
            sign_compact_jws(bearer_did, key_selector, &encoded_header, encoded_payload)?;
        Ok(compact_jws)
    }
}

pub fn sign_compact_jws(
    bearer_did: &BearerDid,
    key_selector: &KeySelector,
    encoded_header: &str,
    encoded_payload: &str,
) -> Result<String, JwsError> {
    let to_sign = format!("{}.{}", encoded_header, encoded_payload);
    let signature = bearer_did.sign(key_selector, &to_sign.into_bytes())?;
    let encoded_signature = general_purpose::URL_SAFE_NO_PAD.encode(signature);
    let compact_jws = format!(
        "{}.{}.{}",
        encoded_header, encoded_payload, encoded_signature
    );
    Ok(compact_jws)
}

pub async fn verify_compact_jws(compact_jws: &str) -> Result<(), JwsError> {
    let parts = splice_parts(compact_jws)?;
    let jws_header = JwsHeader::new_from_encoded(&parts[0])?;
    let key_id = jws_header.kid.clone();
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
    let to_verify = format!("{}.{}", parts[0], parts[1]);
    let alg: String = jws_header.alg.clone();
    let decoded_signature = general_purpose::URL_SAFE_NO_PAD
        .decode(&parts[2])
        .map_err(|e| JwsError::DecodingError(e.to_string()))?;
    match alg.as_str() {
        "EdDSA" => Ed25199::verify(&public_key, &to_verify.into_bytes(), &decoded_signature),
        "ES256K" => Secp256k1::verify(&public_key, &to_verify.into_bytes(), &decoded_signature),
        _ => return Err(JwsError::AlgorithmNotFound(alg)),
    }?;
    Ok(())
}

#[cfg(test)]
mod test {
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
    fn test_from_encoded() {
        let compact_jws = "eyJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkVSVFFTSXNJbU55ZGlJNklrVmtNalUxTVRraUxDSnJkSGtpT2lKUFMxQWlMQ0o0SWpvaWNrRnZTek5EYkdGM1UwWlhTRkJ1T0VGMk5rbHFiR3BDVTNkc1pERnZhMEp5WDI0elkwMWFkVWQ0WXlKOSMwIiwidHlwIjoiSldUIn0.eyJpc3MiOiJkaWQ6ZXhhbXBsZToxMjMifQ.XhJrLvSoQL3N8AOM3OtLBq45K2IFJUiaAwWBPTscwkEKH3I1wExs1-AhTaCvyGwDpCGDmm7T21pKnNwPsoCTCw";
        let parts = splice_parts(compact_jws).unwrap();
        let jws_header =
            JwsHeader::new_from_encoded(&parts[0]).expect("failed to instantiate from compact jws");
        assert_eq!(jws_header.alg, "EdDSA".to_string());
        assert_eq!(jws_header.kid, "did:jwk:eyJhbGciOiJFZERTQSIsImNydiI6IkVkMjU1MTkiLCJrdHkiOiJPS1AiLCJ4IjoickFvSzNDbGF3U0ZXSFBuOEF2NklqbGpCU3dsZDFva0JyX24zY01adUd4YyJ9#0".to_string());
        assert_eq!(jws_header.typ, "JWT".to_string());
    }

    #[test]
    fn test_from_bearer_did() {
        let key_manager = Arc::new(LocalKeyManager::new_in_memory());
        let options = DidJwkCreateOptions {
            curve: Curve::Ed25519,
        };
        let bearer_did = DidJwk::create(key_manager, options).unwrap();
        let key_selector = KeySelector::MethodType {
            verification_method_type: VerificationMethodType::VerificationMethod,
        };
        let jws_header = JwsHeader::from_bearer_did(&bearer_did, &key_selector, "JWT")
            .expect("failed to instantiate JwsHeader from bearer did");

        assert_eq!(jws_header.alg, "EdDSA".to_string());
        assert_eq!(
            jws_header.kid,
            bearer_did
                .document
                .get_verification_method(&key_selector)
                .unwrap()
                .id
        );
        assert_eq!(jws_header.typ, "JWT".to_string());
    }

    #[test]
    fn test_encode() {
        let jws_header = JwsHeader::new("EdDSA".to_string(), "did:jwk:eyJhbGciOiJFZERTQSIsImNydiI6IkVkMjU1MTkiLCJrdHkiOiJPS1AiLCJ4IjoickFvSzNDbGF3U0ZXSFBuOEF2NklqbGpCU3dsZDFva0JyX24zY01adUd4YyJ9#0".to_string(), "JWT".to_string());
        let encoded = jws_header.encode().expect("failed to encode jws header");
        assert!(encoded.len() > 0);

        let new_jws_header = JwsHeader::new_from_encoded(&encoded).unwrap();
        assert_eq!(jws_header, new_jws_header);
    }

    #[test]
    fn test_sign_compact_jws_self() {
        let key_manager = Arc::new(LocalKeyManager::new_in_memory());
        let options = DidJwkCreateOptions {
            curve: Curve::Ed25519,
        };
        let bearer_did = DidJwk::create(key_manager, options).unwrap();
        let key_selector = KeySelector::MethodType {
            verification_method_type: VerificationMethodType::VerificationMethod,
        };
        let jws_header = JwsHeader::from_bearer_did(&bearer_did, &key_selector, "JWT")
            .expect("failed to instantiate JwsHeader from bearer did");

        let encoded_payload = "eyJpc3MiOiJkaWQ6ZXhhbXBsZToxMjMifQ";
        let signed = jws_header
            .sign_compact_jws(&bearer_did, &key_selector, encoded_payload)
            .unwrap();
        assert!(signed.len() > 0);
    }

    #[test]
    fn test_sign_compact_jws() {
        let key_manager = Arc::new(LocalKeyManager::new_in_memory());
        let options = DidJwkCreateOptions {
            curve: Curve::Ed25519,
        };
        let bearer_did = DidJwk::create(key_manager, options).unwrap();
        let key_selector = KeySelector::MethodType {
            verification_method_type: VerificationMethodType::VerificationMethod,
        };
        let jws_header = JwsHeader::from_bearer_did(&bearer_did, &key_selector, "JWT")
            .expect("failed to instantiate JwsHeader from bearer did");

        let encoded_header = jws_header.encode().unwrap();
        let encoded_payload = "eyJpc3MiOiJkaWQ6ZXhhbXBsZToxMjMifQ";
        let signed =
            sign_compact_jws(&bearer_did, &key_selector, &encoded_header, encoded_payload).unwrap();
        assert!(signed.len() > 0);
    }

    #[tokio::test]
    async fn test_verify() {
        let compact_jws = "eyJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkVSVFFTSXNJbU55ZGlJNklrVmtNalUxTVRraUxDSnJkSGtpT2lKUFMxQWlMQ0o0SWpvaWNXWnZObmN5VVRWaFExcDJZemxZUTFBd2FXWnRZWFJvYkdKSGRHVkxja1V5Y2xWR1lVSldkRVJqU1NKOSMwIiwidHlwIjoiSldUIn0.eyJpc3MiOiJkaWQ6ZXhhbXBsZToxMjMifQ.IHupSrKGXg-5q-769RAfomDre_gSv2P4_i9JjynRUHybSwTyvCRz7U-THx2KVsp4NCtyaWRXQz4f0GyZCSYxDA";
        let result = verify_compact_jws(compact_jws).await;
        assert!(result.is_ok());
    }
}
