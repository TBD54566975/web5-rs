use chrono::{DateTime, Utc};
use dids::{bearer::BearerDid, document::KeySelector};
use jwt::{sign_jwt, Claims, JwtError, JwtString};
use keys::key::KeyError;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use uuid::Uuid;

const BASE_CONTEXT: &str = "https://www.w3.org/2018/credentials/v1";
const BASE_TYPE: &str = "VerifiableCredential";

#[derive(Serialize, Deserialize, Debug)]
pub struct VerifiableCredential<T: CredentialSubject + Serialize> {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct DefaultCredentialSubject {
    pub id: String,
}

impl CredentialSubject for DefaultCredentialSubject {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn set_id(&mut self, id: String) {
        self.id = id
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
    #[error("vc jwt error {0}")]
    VcJwtError(String),
}

impl<T: CredentialSubject + Serialize + DeserializeOwned> VerifiableCredential<T> {
    pub fn create(
        credential_subject: T,
        issuer: &str,
        options: Option<CreateOptions>,
    ) -> Result<VerifiableCredential<T>, CredentialError> {
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

    pub async fn from_vcjwt(vcjwt: &str) -> Result<VerifiableCredential<T>, CredentialError> {
        let decoded_jwt = vcjwt.to_string().verify().await?;
        let vc_value = decoded_jwt.claims.vc.ok_or(CredentialError::VcJwtError(
            "vc claim missing from jwt".to_string(),
        ))?;

        let vc: VerifiableCredential<T> = serde_json::from_value(vc_value)
            .map_err(|_| CredentialError::VcJwtError("vc claim value error".to_string()))?;

        Ok(vc)
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
    fn test_sign_vcjwt() {
        let key_manager = Arc::new(LocalKeyManager::new_in_memory());
        let options = DidJwkCreateOptions {
            curve: Curve::Ed25519,
        };
        let bearer_did = DidJwk::create(key_manager, options).unwrap();

        let credential_subject = DefaultCredentialSubject {
            id: Uuid::new_v4().to_string(),
        };

        let mut vc = VerifiableCredential::create(
            credential_subject,
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

    #[tokio::test]
    async fn test_from_vcjwt() {
        let jwt = "eyJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkVSVFFTSXNJbU55ZGlJNklrVmtNalUxTVRraUxDSnJkSGtpT2lKUFMxQWlMQ0o0SWpvaWRUUXhiRU0wY1ZGMFJqazFOazR0VEROb1RsbG9NazFxTlZOWVVscFFORmt5TnpKSU9VeEpUVFJLYXlKOSNYSHNxQkxyOEVzY0RwYWdpOS1uTWhZM0g5MV9fWjJmcHBqZDFlYjV3LVZvIiwidHlwIjoiSldUIn0.eyJpc3MiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpFUlRRU0lzSW1OeWRpSTZJa1ZrTWpVMU1Ua2lMQ0pyZEhraU9pSlBTMUFpTENKNElqb2lkVFF4YkVNMGNWRjBSamsxTms0dFRETm9UbGxvTWsxcU5WTllVbHBRTkZreU56SklPVXhKVFRSS2F5SjkiLCJzdWIiOiJmMDg5MmE0ZS1lMmIwLTQ2MzUtOThkZC04YTEwZjk5MzVkODQiLCJleHAiOjE3MTMxNTI2ODksIm5iZiI6MTcxMzE1MDg4OSwianRpIjoidXJuOnZjOnV1aWQ6MjY4NmVmMWUtNmQ2NC00N2RkLWJkYWYtYTQwMzZiODJjZTdmIiwidmMiOnsiQGNvbnRleHQiOlsiaHR0cHM6Ly93d3cudzMub3JnLzIwMTgvY3JlZGVudGlhbHMvdjEiXSwiY3JlZGVudGlhbF9zdWJqZWN0Ijp7ImlkIjoiZjA4OTJhNGUtZTJiMC00NjM1LTk4ZGQtOGExMGY5OTM1ZDg0In0sImV4cGlyYXRpb25EYXRlIjoiMjAyNC0wNC0xNVQwMzo0NDo0OS45ODMxOTBaIiwiaWQiOiJ1cm46dmM6dXVpZDoyNjg2ZWYxZS02ZDY0LTQ3ZGQtYmRhZi1hNDAzNmI4MmNlN2YiLCJpc3N1YW5jZURhdGUiOiIyMDI0LTA0LTE1VDAzOjE0OjQ5Ljk4MzIyOFoiLCJpc3N1ZXIiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpFUlRRU0lzSW1OeWRpSTZJa1ZrTWpVMU1Ua2lMQ0pyZEhraU9pSlBTMUFpTENKNElqb2lkVFF4YkVNMGNWRjBSamsxTms0dFRETm9UbGxvTWsxcU5WTllVbHBRTkZreU56SklPVXhKVFRSS2F5SjkiLCJ0eXBlIjpbIlZlcmlmaWFibGVDcmVkZW50aWFsIl19fQ.VlCqPOJQuXojteVjWX2itsWg0XE6WjPn3Cp5cp7aIQ-tBPogzSJK-2i9FCBh3KhfBdCB_sTIKy3obMXGI7rtBw";
        let vc: VerifiableCredential<DefaultCredentialSubject> =
            VerifiableCredential::from_vcjwt(jwt).await.unwrap();
        println!("VC {:?}", vc)
    }
}
