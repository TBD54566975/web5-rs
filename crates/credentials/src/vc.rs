use base64::{engine::general_purpose, Engine as _};
use dids::{bearer::BearerDid, document::KeySelector};
use jws::{splice_parts, JwsError, JwsHeader};
use jwt::{sign_jwt, verify_jwt, Claims, JwtError};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

#[derive(thiserror::Error, Debug)]
pub enum CredentialError {
    #[error(transparent)]
    JwtError(#[from] JwtError),
    #[error(transparent)]
    JwsError(#[from] JwsError),
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct VerifiableCredential {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: Vec<String>,
    pub issuer: String,
    #[serde(rename = "issuanceDate")]
    pub issuance_date: i64,
    #[serde(rename = "expirationDate")]
    pub expiration_date: Option<i64>,
    pub credential_subject: CredentialSubject,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CredentialSubject {
    pub id: String,
    #[serde(flatten)]
    pub params: Option<HashMap<String, String>>,
}

impl VerifiableCredential {
    pub fn new(
        context: Vec<String>,
        id: String,
        r#type: Vec<String>,
        issuer: String,
        issuance_date: i64,
        expiration_date: Option<i64>,
        credential_subject: CredentialSubject,
    ) -> Self {
        Self {
            context,
            id,
            r#type,
            issuer,
            issuance_date,
            expiration_date,
            credential_subject,
        }
    }

    pub fn sign(
        &self,
        bearer_did: &BearerDid,
        key_selector: &KeySelector,
    ) -> Result<String, CredentialError> {
        let header = JwsHeader::from_bearer_did(bearer_did, key_selector, "JWT")?;
        let claims = VcJwtClaims {
            base_claims: Claims {
                issuer: Some(self.issuer.clone()),
                jti: Some(self.id.clone()),
                subject: Some(self.credential_subject.id.clone()),
                not_before: Some(self.issuance_date),
                expiration: match self.expiration_date {
                    Some(exp) => Some(exp),
                    None => None,
                },
                ..Default::default()
            },
            vc: self.clone(),
        };

        let encoded_header = header.encode()?;
        let encoded_claims = claims.encode()?;

        let vcjwt = sign_jwt(bearer_did, key_selector, &encoded_header, &encoded_claims)?;
        Ok(vcjwt)
    }
}

pub async fn verify_vcjwt(jwt: &str) -> Result<Arc<VerifiableCredential>, CredentialError> {
    let _ = verify_jwt(jwt).await?;
    let claims = VcJwtClaims::new_from_compact_jws(jwt)?;
    Ok(Arc::new(claims.vc))
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct VcJwtClaims {
    vc: VerifiableCredential,
    #[serde(flatten)]
    base_claims: Claims,
}

impl VcJwtClaims {
    pub fn new_from_compact_jws(compact_jws: &str) -> Result<Self, CredentialError> {
        let parts = splice_parts(compact_jws)?;
        let decoded_bytes = general_purpose::URL_SAFE_NO_PAD
            .decode(&parts[1])
            .map_err(|e| JwsError::DecodingError(e.to_string()))?;
        let claims: Self = serde_json::from_slice(&decoded_bytes)
            .map_err(|e| JwsError::DeserializationError(e.to_string()))?;
        Ok(claims)
    }

    pub fn encode(&self) -> Result<String, CredentialError> {
        let json_str = serde_json::to_string(&self)
            .map_err(|e| JwsError::SerializationError(e.to_string()))?;
        let encoded_str = general_purpose::URL_SAFE_NO_PAD.encode(json_str.as_bytes());
        Ok(encoded_str)
    }
}
