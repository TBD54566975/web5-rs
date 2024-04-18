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

#[derive(Serialize, Deserialize, Debug)]
pub struct JwsHeader {
    alg: String,
    kid: String,
    typ: String,
}

impl JwsHeader {
    pub fn new(alg: String, kid: String, typ: String) -> Self {
        Self { alg, kid, typ }
    }

    pub fn from_compact_jws(compact_jws: &str) -> Result<Self, JwsError> {
        let parts = splice_parts(compact_jws)?;
        let decoded_bytes = general_purpose::URL_SAFE_NO_PAD
            .decode(&parts[0])
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
        Ok(Self { alg, kid, typ: typ.to_string() })
    }

    // todo getter methods

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
    let jws_header = JwsHeader::from_compact_jws(&parts[0])?;
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
    let alg = jws_header.alg.clone();
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
