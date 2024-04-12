use base64::{engine::general_purpose, Engine as _};
use dids::{
    bearer::{BearerDid, BearerDidError},
    document::{DocumentError, KeySelector},
};
use josekit::JoseError;
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

pub trait JwsString {
    fn decode(&self) -> Result<Decoded, JwsError>;
    fn verify(&self) -> Result<Decoded, JwsError>;
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

    fn verify(&self) -> Result<Decoded, JwsError> {
        // let decoded = self.decode()?;
        // let resolution_result = Resolver::resolve_uri(&decoded.header.kid);
        // let public_key_jwk = resolution_result

        // TODO cryptographic verification
        // resolve did
        // select public key jwk from did
        // pass into functions below

        //   let verifier: Box<dyn JwsVerifier> = match decoded.header.alg {
        //       &"ES256K" => &EcdsaJwsAlgorithm::Es256k,
        //       &"EdDSA" => &EddsaJwsAlgorithm::Eddsa,

        //     Some("secp256k1") => Box::new(EcdsaJwsAlgorithm::Es256k.verifier_from_jwk(&self.0)?),
        //     Some("Ed25519") => Box::new(EddsaJwsAlgorithm::Eddsa.verifier_from_jwk(&self.0)?),
        //     _ => return Err(KeyError::AlgorithmNotFound),
        // };

        // verifier.verify(payload, signature).map_err(KeyError::from)

        // Ok()
        unimplemented!()
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
    // todo options
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
