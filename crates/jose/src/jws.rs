use async_trait::async_trait;
use base64::{engine::general_purpose, Engine as _};
use dids::{
    bearer::{BearerDid, BearerDidError},
    document::{DocumentError, KeySelector},
    resolver::Resolver,
};
use josekit::{
    jws::{
        alg::{ecdsa::EcdsaJwsAlgorithm, eddsa::EddsaJwsAlgorithm},
        JwsVerifier,
    },
    JoseError,
};
use serde::{Deserialize, Serialize};
use serde_json::{from_slice, to_string};
use std::sync::Arc;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum JwsError {
    #[error(transparent)]
    BearerDidError(#[from] BearerDidError),
    #[error("serialization error {0}")]
    SerializationError(String),
    #[error("deserialization error {0}")]
    DeserializationError(String),
    #[error("josekit error")]
    JoseError(String),
    #[error("algorithm not found {0}")]
    AlgorithmNotFound(String),
    #[error(transparent)]
    DocumentError(#[from] DocumentError),
}

impl From<JoseError> for JwsError {
    fn from(error: JoseError) -> Self {
        JwsError::JoseError(error.to_string())
    }
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

#[async_trait]
pub trait JwsString {
    fn decode(&self) -> Result<Decoded, JwsError>;
    async fn verify(&self) -> Result<Decoded, JwsError>;
}

#[async_trait]
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

    async fn verify(&self) -> Result<Decoded, JwsError> {
        let decoded = self.decode()?;
        let key_id = decoded.header.kid.clone();
        let resolution_result = Resolver::resolve_uri(&key_id).await;
        let verification_method = match resolution_result.did_document {
            Some(document) => document.get_verification_method(&KeySelector::KeyId(key_id)),
            None => {
                return Err(JwsError::DocumentError(
                    DocumentError::VerificationMethodNotFound,
                ))
            }
        }?;
        let public_key = verification_method.public_key_jwk.clone();

        let alg = decoded.header.alg.clone();
        let verifier: Arc<dyn JwsVerifier> = match alg.as_str() {
            "ES256K" => Arc::new(EcdsaJwsAlgorithm::Es256k.verifier_from_jwk(&public_key)?),
            "EdDSA" => Arc::new(EddsaJwsAlgorithm::Eddsa.verifier_from_jwk(&public_key)?),
            _ => return Err(JwsError::AlgorithmNotFound(alg)),
        };

        verifier.verify(&decoded.payload, &decoded.signature)?;

        Ok(decoded)
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
    let signer = bearer_did.get_jws_signer(key_selector)?;

    let kid = verification_method.id;
    let alg = match verification_method.public_key_jwk.curve() {
        Some("secp256k1") => "ES256K".to_string(),
        Some("Ed25519") => "EdDSA".to_string(),
        _ => return Err(JwsError::AlgorithmNotFound(kid)),
    };
    let typ = options.r#type.unwrap_or_else(|| "JWT".to_string());
    let header = Header { alg, kid, typ };
    let encoded_header = header.encode()?;
    let to_sign = format!("{}.{}", encoded_header, encoded_payload);

    let signature = signer.sign(&to_sign.into_bytes())?;
    let encoded_signature = general_purpose::URL_SAFE_NO_PAD.encode(signature);

    let jws_token = format!(
        "{}.{}.{}",
        encoded_header, encoded_payload, encoded_signature
    );
    Ok(jws_token)
}
