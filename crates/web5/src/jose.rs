use std::str::FromStr;

use crate::{
    crypto::dsa::{ed25519::Ed25519Verifier, Dsa, Verifier},
    dids::{
        bearer_did::BearerDid, data_model::document::FindVerificationMethodOptions,
        resolution::resolution_result::ResolutionResult,
    },
    errors::{Result, Web5Error},
    json::{FromJson, JsonObject, ToJson},
};
use base64::Engine;

pub struct Jwt {
    pub kid: String,
    pub parts: Vec<String>,
    pub header: JsonObject,
    pub claims: JsonObject,
    pub signature: Vec<u8>,
    pub compact_jws: String,
}

impl Jwt {
    pub fn from_claims(
        claims: &JsonObject,
        bearer_did: &BearerDid,
        verification_method_id: Option<String>,
    ) -> Result<Self> {
        let verification_method_id = verification_method_id
            .unwrap_or_else(|| bearer_did.document.verification_method[0].id.clone());

        let is_assertion_method =
            if let Some(assertion_methods) = &bearer_did.document.assertion_method {
                assertion_methods.contains(&verification_method_id)
            } else {
                false
            };

        if !is_assertion_method {
            return Err(Web5Error::Parameter(format!(
                "verification_method_id {} is not an assertion_method",
                verification_method_id
            )));
        }

        let verification_method =
            bearer_did
                .document
                .find_verification_method(FindVerificationMethodOptions {
                    verification_method_id: Some(verification_method_id.clone()),
                })?;
        let alg = verification_method
            .public_key_jwk
            .alg
            .ok_or(Web5Error::Parameter(
                "did document publicKeyJwk must have alg".to_string(),
            ))?;

        let mut header = JsonObject::new();
        header.insert("typ", &"JWT".to_string())?;
        header.insert("alg", &alg)?;
        header.insert("kid", &verification_method_id)?;

        let header_part =
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(header.to_json_string()?);

        let claims_part =
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(claims.to_json_string()?);

        let message = format!("{}.{}", header_part, claims_part);

        let signer = bearer_did.get_signer(&verification_method_id)?;
        let signature = signer.sign(message.as_bytes())?;

        let signature_part =
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(signature.clone());

        let compact_jws = format!("{}.{}.{}", header_part, claims_part, signature_part);

        Ok(Self {
            kid: verification_method_id,
            parts: vec![header_part, claims_part, signature_part],
            header,
            claims: claims.clone(),
            signature,
            compact_jws,
        })
    }

    pub fn from_compact_jws(compact_jws: &str, verify: bool) -> Result<Self> {
        let parts = compact_jws
            .split(".")
            .map(String::from)
            .collect::<Vec<String>>();
        if parts.len() != 3 {
            return Err(Web5Error::Parameter(
                "compact jws has wrong number of parts".to_string(),
            ));
        }

        let header_json_byte_array = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(&parts[0])
            .map_err(|e| {
                Web5Error::Parameter(format!("failed to base64 decode header part {}", e))
            })?;
        let header = JsonObject::from_json_byte_array(&header_json_byte_array)?;

        let claims_json_byte_array = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(&parts[1])
            .map_err(|e| {
                Web5Error::Parameter(format!("failed to base64 decode claims part {}", e))
            })?;
        let claims = JsonObject::from_json_byte_array(&claims_json_byte_array)?;

        let signature = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(&parts[2])
            .map_err(|e| {
                Web5Error::Parameter(format!("failed to base64 decode signature part {}", e))
            })?;

        let kid = header.get::<String>("kid")?.ok_or(Web5Error::Parameter(
            "jose header must include kid value".to_string(),
        ))?;

        if verify {
            let resolution_result = ResolutionResult::resolve(&kid);
            let document = match resolution_result.resolution_metadata.error {
                Some(e) => return Err(e.into()),
                None => match resolution_result.document {
                    Some(d) => d,
                    None => {
                        return Err(Web5Error::Parameter(format!(
                            "failed to resolve did document for {}",
                            kid
                        )))
                    }
                },
            };

            let public_jwk = document
                .find_verification_method(FindVerificationMethodOptions {
                    verification_method_id: Some(kid.clone()),
                })?
                .public_key_jwk;
            let verifier = match &public_jwk.alg {
                None => {
                    return Err(Web5Error::Parameter(format!(
                        "verification method jwk alg must be set {}",
                        kid
                    )))
                }
                Some(alg) => match Dsa::from_str(alg)? {
                    Dsa::Ed25519 => Ed25519Verifier::new(public_jwk),
                    Dsa::Secp256k1 => {
                        return Err(Web5Error::Unknown(
                            "secp256k1 verification not currently supported".to_string(),
                        ))
                    }
                },
            };

            let payload = format!("{}.{}", parts[0], parts[1]);

            verifier.verify(payload.as_bytes(), &signature)?;
        }

        Ok(Self {
            kid,
            parts,
            header,
            claims,
            signature,
            compact_jws: compact_jws.to_string(),
        })
    }
}
