use chrono::{DateTime, Utc};
use crypto::key::PrivateKeySigner;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

const BASE_CONTEXT: &str = "https://www.w3.org/2018/credentials/v1";
const BASE_TYPE: &str = "VerifiableCredential";

#[derive(Serialize, Deserialize)]
pub struct DataModel<T: CredentialSubject> {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: Vec<String>,
    pub issuer: String, // TODO also need to support it as an object
    #[serde(rename = "issuanceDate")]
    pub issuance_date: DateTime<Utc>,
    #[serde(rename = "expirationDate")]
    pub expiration_date: Option<DateTime<Utc>>,
    pub credential_subject: T,
    // todo credential_status, credential_schema, evidence
}

pub trait CredentialSubject {
    fn get_id(&self) -> String;
    fn set_id(&mut self, id: String);
}

pub type Claims = HashMap<String, serde_json::Value>;

impl CredentialSubject for Claims {
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
}

impl<T: CredentialSubject> DataModel<T> {
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
                .unwrap_or_else(|| Uuid::new_v4().to_string()),
            context: options
                .as_ref()
                .and_then(|opts| opts.contexts.clone())
                .unwrap_or_default()
                .into_iter()
                .fold(vec![BASE_CONTEXT.to_string()], |mut acc, ctx| {
                    acc.push(ctx);
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

    pub fn sign(&self, _signer: PrivateKeySigner) -> Result<String, CredentialError> {
      // todo claims, header, 

        unimplemented!()
    }

    pub fn from_vcjwt(_vcjwt: &str) -> Result<Self, CredentialError> {
        unimplemented!()
    }
}
