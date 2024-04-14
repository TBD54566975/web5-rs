use chrono::{DateTime, Utc};
use dids::{bearer::BearerDid, document::KeySelector};
use jwt::{sign_jwt, Claims, JwtError};
use keys::key::KeyError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

const BASE_CONTEXT: &str = "https://www.w3.org/2018/credentials/v1";
const BASE_TYPE: &str = "VerifiableCredential";

#[derive(Serialize, Deserialize)]
pub struct DataModel<T: CredentialSubject + Serialize> {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: Vec<String>,
    pub issuer: String,
    #[serde(rename = "issuanceDate")]
    pub issuance_date: DateTime<Utc>,
    #[serde(rename = "expirationDate")]
    pub expiration_date: Option<DateTime<Utc>>,
    pub credential_subject: T,
}

pub trait CredentialSubject {
    fn get_id(&self) -> String;
    fn set_id(&mut self, id: String);
}

pub type CredentialSubjectClaims = HashMap<String, serde_json::Value>;

impl CredentialSubject for CredentialSubjectClaims {
    fn get_id(&self) -> String {
        self.get("id")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string()
    }

    fn set_id(&mut self, id: String) {
        self.insert("id".to_string(), serde_json::Value::String(id));
    }
}

#[derive(Default)]
pub struct CreateOptions {
    pub id: Option<String>,
    pub contexts: Option<Vec<String>>,
    pub types: Option<Vec<String>>,
    pub issuance_date: Option<DateTime<Utc>>,
    pub expiration_date: Option<DateTime<Utc>>,
}

#[derive(thiserror::Error, Debug)]
pub enum CredentialError {
    #[error("issuer cannot be empty")]
    EmptyIssuer,
    #[error("signing failed")]
    SigningFailed,
    #[error(transparent)]
    KeyError(#[from] KeyError),
    #[error(transparent)]
    JwtError(#[from] JwtError),
}

impl<T: CredentialSubject + Serialize> DataModel<T> {
    pub fn create(
        credential_subject: T,
        issuer: &str,
        options: Option<CreateOptions>,
    ) -> Result<DataModel<T>, CredentialError> {
        if issuer.is_empty() {
            return Err(CredentialError::EmptyIssuer);
        }

        Ok(Self {
            id: options
                .as_ref()
                .and_then(|opts| opts.id.clone())
                .unwrap_or_else(|| format!("urn:vc:uuid:{0}", Uuid::new_v4().to_string())),
            context: options
                .as_ref()
                .and_then(|opts| opts.contexts.clone())
                .unwrap_or_default()
                .into_iter()
                .fold(vec![BASE_CONTEXT.to_string()], |mut acc, item| {
                    acc.push(item);
                    acc
                }),
            r#type: options
                .as_ref()
                .and_then(|opts| opts.types.clone())
                .unwrap_or_default()
                .into_iter()
                .fold(vec![BASE_TYPE.to_string()], |mut acc, t| {
                    acc.push(t);
                    acc
                }),
            issuance_date: options
                .as_ref()
                .and_then(|opts| opts.issuance_date)
                .unwrap_or_else(Utc::now),
            issuer: issuer.to_string(),
            expiration_date: options.as_ref().and_then(|opts| opts.expiration_date),
            credential_subject,
        })
    }

    pub fn sign_vcjwt(
        &mut self,
        bearer_did: BearerDid,
        key_selector: &KeySelector,
    ) -> Result<String, CredentialError> {
        self.issuer = bearer_did.identifier.uri.clone();

        let issuer = &bearer_did.identifier.uri;
        let claims = Claims {
            issuer: Some(issuer.clone()),
            jti: Some(self.id.clone()),
            subject: Some(self.credential_subject.get_id()),
            not_before: Some(self.issuance_date.timestamp()),
            expiration: match self.expiration_date {
                Some(exp) => Some(exp.timestamp()),
                None => None,
            },
            vc: Some(serde_json::to_value(self).map_err(|_| CredentialError::SigningFailed)?),
            ..Default::default()
        };

        let jwt = sign_jwt(&bearer_did, key_selector, &claims, None)?;
        Ok(jwt)
    }

    pub fn from_vcjwt(_vcjwt: &str) -> Result<Self, CredentialError> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    use crypto::Curve;
    use dids::document::VerificationMethodType;
    use dids::method::jwk::{DidJwk, DidJwkCreateOptions};
    use dids::method::Method;
    use keys::key_manager::local_key_manager::LocalKeyManager;
    use std::sync::Arc;

    #[test]
    fn test_everythang() {
        let key_manager = Arc::new(LocalKeyManager::new_in_memory());
        let options = DidJwkCreateOptions {
            curve: Curve::Ed25519,
        };
        let bearer_did = DidJwk::create(key_manager, options).unwrap();

        let mut claims = CredentialSubjectClaims::new();
        claims.set_id("subject_id-something-something-testing123".to_string());

        let mut vc = DataModel::create(
            claims,
            &bearer_did.identifier.uri,
            Some(CreateOptions {
                expiration_date: Some(Utc::now() + Duration::minutes(30)),
                ..Default::default()
            }),
        )
        .expect("Failed to create DataModel");

        let signed_vcjwt = vc
            .sign_vcjwt(
                bearer_did,
                &KeySelector::MethodType(VerificationMethodType::VerificationMethod),
            )
            .expect("Failed to sign VC");

        println!("Signed JWT: {:?}", signed_vcjwt);
    }
}
