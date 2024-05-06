use crate::JwsError;
use base64::{engine::general_purpose, Engine as _};
use crypto::{ed25519::Ed25519, secp256k1::Secp256k1, CurveOperations};
use dids::{
    bearer::BearerDid,
    document::{DocumentError, KeyIdFragment, KeySelector},
    resolver::Resolver,
};
use serde::{Deserialize, Serialize};

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
        let header_json_string = serde_json::to_string(header)
            .map_err(|e| JwsError::SerializationError(e.to_string()))?;
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

        let decoded_header = general_purpose::URL_SAFE_NO_PAD
            .decode(&parts[0])
            .map_err(|e| JwsError::DecodingError(e.to_string()))?;
        let header: JwsHeader = serde_json::from_slice(&decoded_header)
            .map_err(|e| JwsError::DeserializationError(e.to_string()))?;

        let decoded_payload = general_purpose::URL_SAFE_NO_PAD
            .decode(&parts[1])
            .map_err(|e| JwsError::DecodingError(e.to_string()))?;

        Ok(JwsDecoded {
            header,
            payload: decoded_payload,
            signature: parts[2].to_string(),
            parts,
        })
    }

    pub async fn verify(compact_jws: &str) -> Result<JwsDecoded, JwsError> {
        let jws_decoded = CompactJws::decode(compact_jws)?;

        // TODO https://github.com/TBD54566975/web5-rs/issues/149

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
        let decoded_signature = general_purpose::URL_SAFE_NO_PAD
            .decode(&jws_decoded.parts[2])
            .map_err(|e| JwsError::DecodingError(e.to_string()))?;
        match alg.as_str() {
            "EdDSA" => Ed25519::verify(&public_key, &to_verify.into_bytes(), &decoded_signature),
            "ES256K" => Secp256k1::verify(&public_key, &to_verify.into_bytes(), &decoded_signature),
            _ => return Err(JwsError::AlgorithmNotFound(alg)),
        }?;
        Ok(jws_decoded)
    }
}
