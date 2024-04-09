use chrono::{DateTime, Utc};
use crypto::{jose::jws_signer::KeyManagerJwsSigner, key_manager::KeyManager};
use dids::bearer::{BearerDid, VerificationMethodSelector};
use josekit::{
    jws::JwsHeader,
    jwt::{encode_with_signer, JwtPayload},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc, time::SystemTime};
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

    pub fn encode_vcjwt(
        &self,
        bearer_did: BearerDid,
        selector: VerificationMethodSelector,
    ) -> Result<String, CredentialError> {
        // todo claims, header,

        let mut claims = JwtPayload::new();
        claims.set_issuer(&bearer_did.identifier.uri);
        claims.set_jwt_id(&self.id);
        claims.set_subject(self.credential_subject.get_id());
        claims.set_not_before(&SystemTime::from(Utc::now()));
        match self.expiration_date {
            Some(exp) => claims.set_expires_at(&SystemTime::from(exp)),
            None => (),
        }
        claims
            .set_claim(
                "vc",
                Some(serde_json::to_value(self).map_err(|_| CredentialError::SigningFailed)?),
            )
            .map_err(|_| CredentialError::SigningFailed)?;

        let mut header = JwsHeader::new();
        header.set_token_type("JWT");

        // todo reconcile arc vs box
        let vm = bearer_did
            .select_verification_method(selector)
            .map_err(|_| CredentialError::SigningFailed)?;

        let parts: Vec<&str> = vm.id.split('#').collect();
        let fragment_str = parts.get(1).unwrap_or(&""); // `fragment_str` is of type `&&str`
        let fragment = fragment_str.to_string(); // Convert `&&str` to `String`

        let raw_key_manager = Box::into_raw(bearer_did.key_manager);
        let arc_key_manager: Arc<dyn KeyManager> = unsafe { Arc::from_raw(raw_key_manager) };
        let test = KeyManagerJwsSigner::new(arc_key_manager, fragment);
        let signer = test.unwrap();

        let jwt = encode_with_signer(&claims, &header, &signer)
            .map_err(|_| CredentialError::SigningFailed)?;

        Ok(jwt)
    }

    pub fn from_vcjwt(_vcjwt: &str) -> Result<Self, CredentialError> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crypto::{key::KeyType, key_manager::local_key_manager::LocalKeyManager};
    use dids::bearer::VerificationMethodType;
    use dids::method::jwk::{DidJwk, DidJwkCreateOptions};
    use dids::method::Method;

    use super::*;

    #[test]
    fn test_everythang() {
        let key_manager = Box::new(LocalKeyManager::new_in_memory());
        let options = DidJwkCreateOptions {
            key_type: KeyType::Ed25519,
        };
        let bearer_did = DidJwk::create(key_manager, options).unwrap();

        let mut claims = Claims::new();
        claims.set_id("subject_id".to_string());

        // Define issuance and expiration dates
        let issuance_date = Utc::now();
        let expiration_date = Some(Utc::now() + chrono::Duration::days(90));

        // Create options for the DataModel
        let options = CreateOptions {
            id: Some("vc_id".to_string()),
            contexts: Some(vec![BASE_CONTEXT.to_string()]),
            types: Some(vec![BASE_TYPE.to_string()]),
            issuance_date: Some(issuance_date),
            expiration_date: expiration_date,
        };

        // Create the DataModel instance
        let vc = DataModel::create(claims, "issuer_did", Some(options))
            .expect("Failed to create DataModel");

        // Sign the VC, converting it into a JWT
        let signed_jwt = vc
            .encode_vcjwt(
                bearer_did,
                VerificationMethodSelector::MethodType(VerificationMethodType::VerificationMethod),
            )
            .expect("Failed to sign VC");

        println!("Signed JWT: {:?}", signed_jwt);
    }
}
