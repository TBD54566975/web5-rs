use crate::{
    credentials::VerificationError,
    crypto::dsa::{ed25519::Ed25519Verifier, secp256k1::Secp256k1Verifier, Dsa, Verifier},
    datetime::{deserialize_optional_unix_timestamp, serialize_optional_unix_timestamp},
    dids::{
        bearer_did::BearerDid, data_model::document::FindVerificationMethodOptions,
        resolution::resolution_result::ResolutionResult,
    },
    errors::{Result, Web5Error},
    json::{FromJson, JsonValue, ToJson},
};
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr, sync::Arc, time::SystemTime};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JoseHeader {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typ: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
}

impl FromJson for JoseHeader {}
impl ToJson for JoseHeader {}

pub struct Jws {
    pub kid: String,
    pub parts: Vec<String>,
    pub header: JoseHeader,
    pub payload: Vec<u8>,
    pub signature: Vec<u8>,
    pub compact_jws: String,
    pub detached_compact_jws: String,
}

impl Jws {
    pub fn from_payload(
        payload: &[u8],
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

        let header = JoseHeader {
            typ: Some("JWT".to_string()),
            alg: Some(alg),
            kid: Some(verification_method_id.clone()),
        };

        let header_part =
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(header.to_json_string()?);

        let claims_part = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(payload);

        let message = format!("{}.{}", header_part, claims_part);

        let signer = bearer_did.get_signer(&verification_method_id)?;
        let signature = signer.sign(message.as_bytes())?;

        let signature_part =
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(signature.clone());

        let compact_jws = format!("{}.{}.{}", header_part, claims_part, signature_part);
        let detached_compact_jws = format!("{}..{}", header_part, signature_part);

        Ok(Self {
            kid: verification_method_id,
            parts: vec![header_part, claims_part, signature_part],
            header,
            payload: payload.into(),
            signature,
            compact_jws,
            detached_compact_jws,
        })
    }

    pub async fn from_compact_jws(compact_jws: &str, verify: bool) -> Result<Self> {
        let parts = compact_jws
            .split('.')
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
        let header = JoseHeader::from_json_byte_array(&header_json_byte_array)?;

        let payload = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(&parts[1])
            .map_err(|e| {
                Web5Error::Parameter(format!("failed to base64 decode claims part {}", e))
            })?;

        let signature = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(&parts[2])
            .map_err(|e| {
                Web5Error::Parameter(format!("failed to base64 decode signature part {}", e))
            })?;

        let kid = header.kid.clone().ok_or(VerificationError::MissingKid)?;
        if kid.is_empty() {
            return Err(VerificationError::MissingKid.into());
        }

        if verify {
            let resolution_result = ResolutionResult::resolve(&kid).await;
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
            let verifier: Arc<dyn Verifier> = match Dsa::from_str(&public_jwk.crv)? {
                Dsa::Ed25519 => Arc::new(Ed25519Verifier::new(public_jwk)),
                Dsa::Secp256k1 => Arc::new(Secp256k1Verifier::new(public_jwk)),
            };

            let payload = format!("{}.{}", parts[0], parts[1]);

            verifier.verify(payload.as_bytes(), &signature)?;
        }

        Ok(Self {
            kid,
            parts: parts.clone(),
            header,
            payload,
            signature,
            compact_jws: compact_jws.to_string(),
            detached_compact_jws: format!("{}..{}", parts[0], parts[2]),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct JwtClaims {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iss: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jti: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub: Option<String>,
    #[serde(
        serialize_with = "serialize_optional_unix_timestamp",
        deserialize_with = "deserialize_optional_unix_timestamp",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub nbf: Option<SystemTime>,
    #[serde(
        serialize_with = "serialize_optional_unix_timestamp",
        deserialize_with = "deserialize_optional_unix_timestamp",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub iat: Option<SystemTime>,
    #[serde(
        serialize_with = "serialize_optional_unix_timestamp",
        deserialize_with = "deserialize_optional_unix_timestamp",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub exp: Option<SystemTime>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<HashMap<String, JsonValue>>,
}

impl FromJson for JwtClaims {}
impl ToJson for JwtClaims {}

pub struct Jwt {
    pub kid: String,
    pub parts: Vec<String>,
    pub header: JoseHeader,
    pub claims: JwtClaims,
    pub signature: Vec<u8>,
    pub compact_jws: String,
    pub detached_compact_jws: String,
}

impl Jwt {
    pub fn from_claims(
        claims: &JwtClaims,
        bearer_did: &BearerDid,
        verification_method_id: Option<String>,
    ) -> Result<Self> {
        let payload = claims.to_json_string()?;
        let jws = Jws::from_payload(payload.as_bytes(), bearer_did, verification_method_id)?;

        Ok(Self {
            kid: jws.kid,
            parts: jws.parts,
            header: jws.header,
            claims: claims.clone(),
            signature: jws.signature,
            compact_jws: jws.compact_jws,
            detached_compact_jws: jws.detached_compact_jws,
        })
    }

    pub async fn from_compact_jws(compact_jws: &str, verify: bool) -> Result<Self> {
        let jws = Jws::from_compact_jws(compact_jws, verify).await?;
        let claims = JwtClaims::from_json_byte_array(&jws.payload)?;

        Ok(Self {
            kid: jws.kid,
            parts: jws.parts,
            header: jws.header,
            claims: claims.clone(),
            signature: jws.signature,
            compact_jws: jws.compact_jws,
            detached_compact_jws: jws.detached_compact_jws,
        })
    }
}
