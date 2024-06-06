use chrono::{DateTime, TimeZone, Utc};
use core::fmt;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::{
    collections::HashMap,
    fmt::Debug,
    fmt::{Display, Formatter},
    sync::Arc,
};

use josekit::{
    jwk::alg::ed::EdCurve,
    jws::{alg::eddsa::EddsaJwsAlgorithm, JwsAlgorithm, JwsHeader, JwsSigner},
    jwt::{encode_with_signer, JwtPayload},
    JoseError,
};

use crate::{
    dids::{
        bearer::BearerDid,
        document::{KeyIdFragment, KeySelector, VerificationMethod},
    },
    jwt::{
        jws::Jwt,
        {Claims, JwtError, RegisteredClaims},
    },
    keys::key_manager::KeyManager,
};

const BASE_CONTEXT: &str = "https://www.w3.org/2018/credentials/v1";
const BASE_TYPE: &str = "VerifiableCredential";

#[derive(thiserror::Error, Debug)]
pub enum CredentialError {
    #[error(transparent)]
    JwtError(#[from] JwtError),
    #[error("missing claim: {0}")]
    MissingClaim(String),
    #[error("claim mismatch: {0}")]
    ClaimMismatch(String),
    #[error("misconfigured expiration date: {0}")]
    MisconfiguredExpirationDate(String),
    #[error("Credential expired")]
    CredentialExpired,
    #[error("VC data model validation error: {0}")]
    VcDataModelValidationError(String),
    #[error("invalid timestamp: {0}")]
    InvalidTimestamp(String),
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct NamedIssuer {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Issuer {
    String(String),
    Object(NamedIssuer),
}

impl<I> From<I> for Issuer
where
    I: Into<String>,
{
    fn from(s: I) -> Self {
        Issuer::String(s.into())
    }
}

impl Display for Issuer {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Issuer::String(s) => write!(f, "{}", s),
            Issuer::Object(ni) => write!(f, "{}", ni.id),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerifiableCredential {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: Vec<String>,
    pub issuer: Issuer,
    #[serde(rename = "issuanceDate")]
    pub issuance_date: String,
    #[serde(rename = "expirationDate")]
    pub expiration_date: Option<String>,
    pub credential_subject: CredentialSubject,
}

impl From<VerifiableCredential> for JwtPayloadVerifiableCredential {
    fn from(credential: VerifiableCredential) -> Self {
        JwtPayloadVerifiableCredential {
            context: credential.context,
            id: Some(credential.id),
            r#type: credential.r#type,
            issuer: Some(credential.issuer),
            issuance_date: Some(credential.issuance_date),
            expiration_date: credential.expiration_date,
            credential_subject: Some(credential.credential_subject),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JwtPayloadVerifiableCredential {
    #[serde(rename = "@context")]
    context: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename = "type")]
    r#type: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer: Option<Issuer>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "issuanceDate")]
    issuance_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "expirationDate")]
    expiration_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "credentialSubject")]
    credential_subject: Option<CredentialSubject>,
}

impl TryFrom<JwtPayloadVerifiableCredential> for VerifiableCredential {
    type Error = CredentialError;
    fn try_from(payload: JwtPayloadVerifiableCredential) -> Result<Self, Self::Error> {
        Ok(VerifiableCredential {
            context: payload.context,
            id: payload
                .id
                .ok_or(CredentialError::VcDataModelValidationError(
                    "invalid or missing id".to_string(),
                ))?,
            r#type: payload.r#type,
            issuer: payload
                .issuer
                .ok_or(CredentialError::VcDataModelValidationError(
                    "invalid or missing issuer".to_string(),
                ))?,
            issuance_date: payload.issuance_date.ok_or(
                CredentialError::VcDataModelValidationError(
                    "invalid or missing issuance date".to_string(),
                ),
            )?,
            expiration_date: payload.expiration_date,
            credential_subject: payload.credential_subject.ok_or(
                CredentialError::VcDataModelValidationError(
                    "invalid or missing credential subject".to_string(),
                ),
            )?,
        })
    }
}

pub struct CustomSigner {
    key_manager: Arc<dyn KeyManager>,
    key_alias: String,
    algorithm: EddsaJwsAlgorithm,
    curve: EdCurve,
    key_id: Option<String>,
}

impl CustomSigner {
    pub fn new(
        key_manager: Arc<dyn KeyManager>,
        key_alias: String,
        algorithm: EddsaJwsAlgorithm,
        curve: EdCurve,
        key_id: Option<String>,
    ) -> Self {
        CustomSigner {
            key_manager,
            key_alias,
            algorithm,
            curve,
            key_id,
        }
    }
}

impl Debug for CustomSigner {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("CustomSigner")
            .field("key_manager", &"<Arc<dyn KeyManager>>")
            .field("key_alias", &self.key_alias)
            .field("algorithm", &self.algorithm)
            .field("curve", &self.curve)
            .field("key_id", &self.key_id)
            .finish()
    }
}

impl JwsSigner for CustomSigner {
    fn algorithm(&self) -> &dyn JwsAlgorithm {
        &self.algorithm
    }

    fn key_id(&self) -> Option<&str> {
        self.key_id.as_deref()
    }

    fn signature_len(&self) -> usize {
        match self.curve {
            EdCurve::Ed25519 => 64,
            EdCurve::Ed448 => 114,
        }
    }

    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, JoseError> {
        self.key_manager
            .sign(&self.key_alias, message)
            .map_err(|err| JoseError::InvalidSignature(err.into()))
    }

    fn box_clone(&self) -> Box<dyn JwsSigner> {
        Box::new(self.clone())
    }
}

impl Clone for CustomSigner {
    fn clone(&self) -> Self {
        Self {
            key_manager: Arc::clone(&self.key_manager),
            key_alias: self.key_alias.clone(),
            algorithm: self.algorithm,
            curve: self.curve,
            key_id: self.key_id.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CredentialSubject {
    pub id: String,
    #[serde(flatten)]
    pub params: Option<HashMap<String, String>>,
}

impl VerifiableCredential {
    pub fn new(
        id: String,
        context: Vec<String>,
        r#type: Vec<String>,
        issuer: Issuer,
        issuance_date: String,
        expiration_date: Option<String>,
        credential_subject: CredentialSubject,
    ) -> Self {
        let context_with_base = std::iter::once(BASE_CONTEXT.to_string())
            .chain(context.into_iter().filter(|c| c != BASE_CONTEXT))
            .collect::<Vec<_>>();

        let type_with_base = std::iter::once(BASE_TYPE.to_string())
            .chain(r#type.into_iter().filter(|t| t != BASE_TYPE))
            .collect::<Vec<_>>();

        Self {
            context: context_with_base,
            id,
            r#type: type_with_base,
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
        let issuance_timestamp = rfc3339_to_timestamp(&self.issuance_date)?;
        let expiration_timestamp = match &self.expiration_date {
            Some(date) => Some(rfc3339_to_timestamp(date)?),
            None => None,
        };

        let claims = VcJwtClaims {
            registered_claims: RegisteredClaims {
                issuer: Some(self.issuer.to_string()),
                jti: Some(self.id.clone()),
                subject: Some(self.credential_subject.id.clone()),
                not_before: Some(issuance_timestamp),
                expiration: expiration_timestamp,
                ..Default::default()
            },
            vc_payload: self.clone().into(),
        };

        // Serialize the claims to JSON
        let claims_json: String = serde_json::to_string(&claims).unwrap();

        // Create a JwtPayload from the claims map
        let claims_map: Map<String, Value> = serde_json::from_str(&claims_json).unwrap();
        let jwt_payload: JwtPayload = JwtPayload::from_map(claims_map).unwrap();

        let verification_method: VerificationMethod = bearer_did
            .document
            .get_verification_method(key_selector)
            .unwrap();

        let mut header: JwsHeader = JwsHeader::new();
        header.set_algorithm(verification_method.public_key_jwk.alg.clone());
        header.set_key_id(verification_method.id.clone());
        header.set_token_type("JWT".to_string());

        let key_alias: String = KeyIdFragment(verification_method.id.clone()).splice_key_alias();

        let signer: CustomSigner = CustomSigner::new(
            Arc::clone(&bearer_did.key_manager),
            key_alias,
            EddsaJwsAlgorithm::Eddsa,
            EdCurve::Ed25519,
            Some(verification_method.id.clone()),
        );

        // Sign the JWT
        let jwt: String = encode_with_signer(&jwt_payload, &header, &signer).unwrap();

        Ok(jwt)
    }

    pub async fn verify(jwt: &str) -> Result<VerifiableCredential, CredentialError> {
        let jwt_decoded = Jwt::verify::<VcJwtClaims>(jwt).await?;
        let vc_payload = jwt_decoded.claims.vc_payload;
        let registered_claims = jwt_decoded.claims.registered_claims;

        // registered claims checks
        let jti = registered_claims
            .jti
            .ok_or(CredentialError::MissingClaim("jti".to_string()))?;
        let iss = registered_claims
            .issuer
            .ok_or(CredentialError::MissingClaim("issuer".to_string()))?;
        let sub = registered_claims
            .subject
            .ok_or(CredentialError::MissingClaim("subject".to_string()))?;
        let nbf = registered_claims
            .not_before
            .ok_or(CredentialError::MissingClaim("not_before".to_string()))?;
        let exp = registered_claims.expiration;

        if let Some(id) = &vc_payload.id {
            if id != &jti {
                return Err(CredentialError::ClaimMismatch("id".to_string()));
            }
        }

        if let Some(issuer) = &vc_payload.issuer {
            let vc_issuer = issuer.to_string();
            if iss != vc_issuer {
                return Err(CredentialError::ClaimMismatch("issuer".to_string()));
            }
        }

        if let Some(credential_subject) = &vc_payload.credential_subject {
            if sub != credential_subject.id {
                return Err(CredentialError::ClaimMismatch("subject".to_string()));
            }
        }

        if let Some(vc_payload_issuance_date) = &vc_payload.issuance_date {
            let vc_payload_timestamp = rfc3339_to_timestamp(vc_payload_issuance_date)?;
            if vc_payload_timestamp != nbf {
                return Err(CredentialError::ClaimMismatch("issuance_date".to_string()));
            }
        }

        let now = Utc::now().timestamp();
        match vc_payload.expiration_date {
            Some(ref vc_payload_expiration_date) => match exp {
                None => {
                    return Err(CredentialError::MisconfiguredExpirationDate(
                        "VC has expiration date but no exp in registered claims".to_string(),
                    ));
                }
                Some(exp) => {
                    let vc_payload_timestamp = rfc3339_to_timestamp(vc_payload_expiration_date)?;
                    if vc_payload_timestamp != exp {
                        return Err(CredentialError::ClaimMismatch(
                            "expiration_date".to_string(),
                        ));
                    }

                    if now > exp {
                        return Err(CredentialError::CredentialExpired);
                    }
                }
            },
            None => {
                if let Some(exp) = exp {
                    if now > exp {
                        return Err(CredentialError::CredentialExpired);
                    }
                }
            }
        }

        let vc_issuer = vc_payload.issuer.unwrap_or(Issuer::String(iss));

        let vc_credential_subject = vc_payload.credential_subject.unwrap_or(CredentialSubject {
            id: sub,
            params: None,
        });

        let nbf_issuance_date = timestamp_to_rfc3339(nbf)?;
        let exp_expiration_date = match exp {
            Some(exp_timestamp) => Some(timestamp_to_rfc3339(exp_timestamp)?),
            None => None,
        };

        let vc = VerifiableCredential {
            context: vc_payload.context,
            id: jti,
            r#type: vc_payload.r#type,
            issuer: vc_issuer,
            issuance_date: nbf_issuance_date,
            expiration_date: exp_expiration_date,
            credential_subject: vc_credential_subject,
        };

        validate_vc_data_model(&vc)?;

        Ok(vc)
    }

    pub fn decode(jwt: &str) -> Result<Self, CredentialError> {
        let jwt_decoded = Jwt::decode::<VcJwtClaims>(jwt)?;
        let vc_payload: JwtPayloadVerifiableCredential = jwt_decoded.claims.vc_payload;
        let vc: VerifiableCredential = vc_payload.try_into()?;
        Ok(vc)
    }
}

fn validate_vc_data_model(vc: &VerifiableCredential) -> Result<(), CredentialError> {
    // Required fields ["@context", "id", "type", "issuer", "issuanceDate", "credentialSubject"]
    if vc.id.is_empty() {
        return Err(CredentialError::VcDataModelValidationError(
            "missing id".to_string(),
        ));
    }

    if vc.context.is_empty() || vc.context[0] != BASE_CONTEXT {
        return Err(CredentialError::VcDataModelValidationError(
            "missing context".to_string(),
        ));
    }

    if vc.r#type.is_empty() || vc.r#type[0] != BASE_TYPE {
        return Err(CredentialError::VcDataModelValidationError(
            "missing type".to_string(),
        ));
    }

    if vc.issuer.to_string().is_empty() {
        return Err(CredentialError::VcDataModelValidationError(
            "missing issuer".to_string(),
        ));
    }

    if vc.issuance_date.is_empty() {
        return Err(CredentialError::VcDataModelValidationError(
            "missing issuance date".to_string(),
        ));
    }

    if vc.credential_subject.id.is_empty() {
        return Err(CredentialError::VcDataModelValidationError(
            "missing credential subject".to_string(),
        ));
    }

    let now = Utc::now().timestamp();
    let issuance_timestamp = rfc3339_to_timestamp(&vc.issuance_date)?;

    if issuance_timestamp > now {
        return Err(CredentialError::VcDataModelValidationError(
            "issuance date in future".to_string(),
        ));
    }

    // Validate expiration date if it exists
    if let Some(ref expiration_date) = vc.expiration_date {
        let expiration_timestamp = rfc3339_to_timestamp(expiration_date)?;

        if expiration_timestamp < now {
            return Err(CredentialError::VcDataModelValidationError(
                "credential expired".to_string(),
            ));
        }
    }

    // TODO: Add validations to credential_status, credential_schema, and evidence once they are added to the VcDataModel
    // https://github.com/TBD54566975/web5-rs/issues/112

    Ok(())
}

/// Convert an i64 timestamp to an RFC 3339 formatted date-time string
pub fn timestamp_to_rfc3339(timestamp: i64) -> Result<String, CredentialError> {
    let datetime = Utc
        .timestamp_opt(timestamp, 0)
        .single()
        .ok_or_else(|| CredentialError::InvalidTimestamp("Invalid timestamp".to_string()))?;
    Ok(datetime.to_rfc3339())
}

/// Convert an RFC 3339 formatted date-time string to an i64 timestamp
pub fn rfc3339_to_timestamp(rfc3339: &str) -> Result<i64, CredentialError> {
    let datetime: DateTime<Utc> = rfc3339
        .parse()
        .map_err(|_| CredentialError::InvalidTimestamp("Invalid timestamp".to_string()))?;
    Ok(datetime.timestamp())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VcJwtClaims {
    #[serde(rename = "vc")]
    vc_payload: JwtPayloadVerifiableCredential,
    #[serde(flatten)]
    registered_claims: RegisteredClaims,
}

impl Claims for VcJwtClaims {}

#[cfg(test)]
mod test {
    use super::*;
    use crate::crypto::Curve;
    use crate::dids::{
        document::VerificationMethodType,
        methods::{
            jwk::{DidJwk, DidJwkCreateOptions},
            Create,
        },
    };
    use crate::keys::key_manager::local_key_manager::LocalKeyManager;
    use std::sync::Arc;
    use uuid::Uuid;

    fn create_bearer_did() -> BearerDid {
        let key_manager = Arc::new(LocalKeyManager::new());
        let options = DidJwkCreateOptions {
            curve: Curve::Ed25519,
        };
        let bearer_did = DidJwk::create(key_manager, options).unwrap();
        bearer_did
    }

    fn create_vc(issuer: Issuer) -> VerifiableCredential {
        let now = Utc::now().timestamp();

        VerifiableCredential::new(
            format!("urn:vc:uuid:{0}", Uuid::new_v4().to_string()),
            vec![BASE_CONTEXT.to_string()],
            vec![BASE_TYPE.to_string()],
            issuer.clone(),
            timestamp_to_rfc3339(now).unwrap(),
            Some(timestamp_to_rfc3339(now + 631152000).unwrap()), // now + 20 years
            CredentialSubject {
                id: issuer.to_string(),
                ..Default::default()
            },
        )
    }

    #[test]
    fn test_create() {
        let bearer_did = create_bearer_did();
        let vc = create_vc((&bearer_did.identifier.uri).into());
        assert_eq!(1, vc.context.len());
        assert_ne!("", vc.id);
        assert_eq!(1, vc.r#type.len());
        assert_eq!(vc.issuer, Issuer::String(bearer_did.identifier.uri.clone()));

        let vc2 = create_vc(Issuer::Object(NamedIssuer {
            id: bearer_did.identifier.uri.clone(),
            name: bearer_did.identifier.id.clone(),
        }));
        assert_eq!(1, vc2.context.len());
        assert_ne!("", vc2.id);
        assert_eq!(1, vc2.r#type.len());
        assert_eq!(
            vc2.issuer,
            Issuer::Object(NamedIssuer {
                id: bearer_did.identifier.uri.clone(),
                name: bearer_did.identifier.id,
            })
        );
    }

    #[test]
    fn test_new() {
        let issuer = "did:jwk:something";
        let issuer_name = "marvin-paranoid-robot";
        let now = Utc::now().timestamp();

        let vc1 = VerifiableCredential::new(
            format!("urn:vc:uuid:{0}", Uuid::new_v4().to_string()),
            vec![BASE_CONTEXT.to_string()],
            vec![BASE_TYPE.to_string()],
            Issuer::String(issuer.to_string()),
            timestamp_to_rfc3339(now).unwrap(),
            Some(timestamp_to_rfc3339(now + 30 * 60).unwrap()),
            CredentialSubject {
                id: issuer.to_string(),
                ..Default::default()
            },
        );

        assert_eq!(1, vc1.context.len());
        assert_eq!(1, vc1.r#type.len());
        assert_eq!(BASE_CONTEXT, vc1.context[0]);
        assert_eq!(BASE_TYPE, vc1.r#type[0]);
        assert_eq!(1, vc1.context.iter().filter(|&c| c == BASE_CONTEXT).count());
        assert_eq!(1, vc1.r#type.iter().filter(|&t| t == BASE_TYPE).count());

        let vc2 = VerifiableCredential::new(
            format!("urn:vc:uuid:{0}", Uuid::new_v4().to_string()),
            vec!["some-other-context".to_string()],
            vec!["some-other-type".to_string()],
            Issuer::String(issuer.to_string()),
            timestamp_to_rfc3339(now).unwrap(),
            Some(timestamp_to_rfc3339(now + 30 * 60).unwrap()),
            CredentialSubject {
                id: issuer.to_string(),
                ..Default::default()
            },
        );

        assert_eq!(2, vc2.context.len());
        assert_eq!(2, vc2.r#type.len());
        assert_eq!(BASE_CONTEXT, vc2.context[0]);
        assert_eq!(BASE_TYPE, vc2.r#type[0]);
        assert_eq!(1, vc2.context.iter().filter(|&c| c == BASE_CONTEXT).count());
        assert_eq!(1, vc2.r#type.iter().filter(|&t| t == BASE_TYPE).count());

        let vc3 = VerifiableCredential::new(
            format!("urn:vc:uuid:{0}", Uuid::new_v4().to_string()),
            vec![BASE_CONTEXT.to_string()],
            vec![BASE_TYPE.to_string()],
            Issuer::Object(NamedIssuer {
                id: issuer.to_string(),
                name: issuer_name.to_string(),
            }),
            timestamp_to_rfc3339(now).unwrap(),
            Some(timestamp_to_rfc3339(now + 30 * 60).unwrap()),
            CredentialSubject {
                id: issuer.to_string(),
                ..Default::default()
            },
        );

        assert_eq!(1, vc3.context.len());
        assert_eq!(1, vc3.r#type.len());
        assert_eq!(BASE_CONTEXT, vc3.context[0]);
        assert_eq!(BASE_TYPE, vc3.r#type[0]);
        assert_eq!(1, vc3.context.iter().filter(|&c| c == BASE_CONTEXT).count());
        assert_eq!(1, vc3.r#type.iter().filter(|&t| t == BASE_TYPE).count());
        assert_eq!(
            Issuer::Object(NamedIssuer {
                id: issuer.to_string(),
                name: issuer_name.to_string(),
            }),
            vc3.issuer,
        );
    }

    #[tokio::test]
    async fn test_sign_and_verify() {
        let bearer_did = create_bearer_did();
        let vc = create_vc((&bearer_did.identifier.uri).into());
        let key_selector = KeySelector::MethodType {
            verification_method_type: VerificationMethodType::VerificationMethod,
        };
        let vc_jwt = vc.sign(&bearer_did, &key_selector).unwrap();
        assert!(!vc_jwt.is_empty());

        let verified_vc = VerifiableCredential::verify(&vc_jwt).await.unwrap();
        assert_eq!(vc.id, verified_vc.id);
        assert_eq!(vc.issuer, verified_vc.issuer);
        assert_eq!(vc.credential_subject.id, verified_vc.credential_subject.id);
    }

    #[tokio::test]
    async fn test_verify_with_expired_exp() {
        let bearer_did = create_bearer_did();
        let key_selector = KeySelector::MethodType {
            verification_method_type: VerificationMethodType::VerificationMethod,
        };

        let now = Utc::now().timestamp();

        let issuer = Issuer::Object(NamedIssuer {
            id: bearer_did.identifier.uri.clone(),
            name: bearer_did.identifier.id.clone(),
        });

        let vc = VerifiableCredential::new(
            format!("urn:vc:uuid:{0}", Uuid::new_v4().to_string()),
            vec![BASE_CONTEXT.to_string()],
            vec![BASE_TYPE.to_string()],
            issuer.clone(),
            timestamp_to_rfc3339(now).unwrap(),
            Some(timestamp_to_rfc3339(now.clone() - 300000).unwrap()),
            CredentialSubject {
                id: bearer_did.identifier.uri.to_string(),
                ..Default::default()
            },
        );

        let vc_jwt = vc.sign(&bearer_did, &key_selector).unwrap();
        assert!(!vc_jwt.is_empty());

        let result = VerifiableCredential::verify(&vc_jwt).await;
        assert!(
            matches!(result, Err(CredentialError::CredentialExpired)),
            "Expected expiration error, but found different or no error"
        );
    }

    #[tokio::test]
    async fn test_verify_mismatched_iss() {
        let mismatched_issuer_vc_jwt = "eyJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkVSVFFTSXNJbU55ZGlJNklrVmtNalUxTVRraUxDSnJkSGtpT2lKUFMxQWlMQ0o0SWpvaU5sVTRSV1JRY210b2JFdGhOWFJvYW05SWEyMDVaV0pFVFhCaVFWWm5iVEIwWm14MU1sZDRkalkwTkNKOSMwIiwidHlwIjoiSldUIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp2Yzp1dWlkOjkyYzMzNmFmLWIxY2ItNDYzMi05YjI1LTgzYmY3NTY1MjBiYiIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaRVJUUVNJc0ltTnlkaUk2SWtWa01qVTFNVGtpTENKcmRIa2lPaUpQUzFBaUxDSjRJam9pTmxVNFJXUlFjbXRvYkV0aE5YUm9hbTlJYTIwNVpXSkVUWEJpUVZabmJUQjBabXgxTWxkNGRqWTBOQ0o5IiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wNS0yOVQxOToxNzo0NCswMDowMCIsImV4cGlyYXRpb25EYXRlIjoiMjA0NC0wNS0yOVQxOToxNzo0NCswMDowMCIsImNyZWRlbnRpYWxTdWJqZWN0Ijp7ImlkIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaRVJUUVNJc0ltTnlkaUk2SWtWa01qVTFNVGtpTENKcmRIa2lPaUpQUzFBaUxDSjRJam9pTmxVNFJXUlFjbXRvYkV0aE5YUm9hbTlJYTIwNVpXSkVUWEJpUVZabmJUQjBabXgxTWxkNGRqWTBOQ0o5In19LCJpc3MiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpFUlRRU0lzSW1OeWRpSTZJa1ZrTWpVMU1Ua2lMQ0pyZEhraU9pSlBTMUFpTENKNElqb2lObFU0UldSUWNtdG9iRXRoTlhSb2FtOUlhMjA1WldKRVRYQmlRVlpuYlRCMFpteDFNbGQ0ZGpZME5DSjlyYW5kb20iLCJzdWIiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpFUlRRU0lzSW1OeWRpSTZJa1ZrTWpVMU1Ua2lMQ0pyZEhraU9pSlBTMUFpTENKNElqb2lObFU0UldSUWNtdG9iRXRoTlhSb2FtOUlhMjA1WldKRVRYQmlRVlpuYlRCMFpteDFNbGQ0ZGpZME5DSjkiLCJleHAiOjIzNDgxNjIyNjQsIm5iZiI6MTcxNzAxMDI2NCwianRpIjoidXJuOnZjOnV1aWQ6OTJjMzM2YWYtYjFjYi00NjMyLTliMjUtODNiZjc1NjUyMGJiIn0.Xwkdx5ZcTqYBSW2NPFQqpzSzi2TiWrZYeDlGJIYIF9clSx2iB04K-jexDcMd4K3wyKofa_lo1_B00hxFXCasDA";
        let result = VerifiableCredential::verify(&mismatched_issuer_vc_jwt).await;

        assert!(
            matches!(result, Err(CredentialError::ClaimMismatch(ref s)) if s == "issuer"),
            "Expected mismatch issuer error, but found different or no error"
        );
    }

    #[tokio::test]
    async fn test_full_featured_vc_jwt() {
        let full_featured_vc_jwt = "eyJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkVSVFFTSXNJbU55ZGlJNklrVmtNalUxTVRraUxDSnJkSGtpT2lKUFMxQWlMQ0o0SWpvaVZVTnlNRkpRUTFCWllYVTRZalpIZGpkU1pIcGtWV052V0VoUlRGbFRlV2xIUldjMVdDMUJibEJzTkNKOSMwIiwidHlwIjoiSldUIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp2Yzp1dWlkOjQ0NzA2MjYwLTUzYzctNGRkMC04MmEyLTQ4NzdiMjU3MzAwNSIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaRVJUUVNJc0ltTnlkaUk2SWtWa01qVTFNVGtpTENKcmRIa2lPaUpQUzFBaUxDSjRJam9pVlVOeU1GSlFRMUJaWVhVNFlqWkhkamRTWkhwa1ZXTnZXRWhSVEZsVGVXbEhSV2MxV0MxQmJsQnNOQ0o5IiwiaXNzdWFuY2VEYXRlIjoiMjAyNC0wNS0yOVQxOToxMjoxMyswMDowMCIsImV4cGlyYXRpb25EYXRlIjoiMjA0NC0wNS0yOVQxOToxMjoxMyswMDowMCIsImNyZWRlbnRpYWxTdWJqZWN0Ijp7ImlkIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaRVJUUVNJc0ltTnlkaUk2SWtWa01qVTFNVGtpTENKcmRIa2lPaUpQUzFBaUxDSjRJam9pVlVOeU1GSlFRMUJaWVhVNFlqWkhkamRTWkhwa1ZXTnZXRWhSVEZsVGVXbEhSV2MxV0MxQmJsQnNOQ0o5In19LCJpc3MiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpFUlRRU0lzSW1OeWRpSTZJa1ZrTWpVMU1Ua2lMQ0pyZEhraU9pSlBTMUFpTENKNElqb2lWVU55TUZKUVExQlpZWFU0WWpaSGRqZFNaSHBrVldOdldFaFJURmxUZVdsSFJXYzFXQzFCYmxCc05DSjkiLCJzdWIiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpFUlRRU0lzSW1OeWRpSTZJa1ZrTWpVMU1Ua2lMQ0pyZEhraU9pSlBTMUFpTENKNElqb2lWVU55TUZKUVExQlpZWFU0WWpaSGRqZFNaSHBrVldOdldFaFJURmxUZVdsSFJXYzFXQzFCYmxCc05DSjkiLCJleHAiOjIzNDgxNjE5MzMsIm5iZiI6MTcxNzAwOTkzMywianRpIjoidXJuOnZjOnV1aWQ6NDQ3MDYyNjAtNTNjNy00ZGQwLTgyYTItNDg3N2IyNTczMDA1In0.WXnRpNsawB_-_LpMpzlT3GBqj1WmpxFAabInEhUGqja_s3S7c9CKUPFMBFRtpz3mVf2g0Gkc4mfdG8yR2j2DDw";
        let jwt_decoded = Jwt::verify::<VcJwtClaims>(&full_featured_vc_jwt)
            .await
            .unwrap();
        let registered_claims = jwt_decoded.claims.registered_claims;

        let verify_result = VerifiableCredential::verify(full_featured_vc_jwt).await;
        let verify_vc = verify_result.unwrap();

        assert_eq!(
            vec!["https://www.w3.org/2018/credentials/v1".to_string()],
            verify_vc.context
        );
        assert_eq!(vec!["VerifiableCredential".to_string()], verify_vc.r#type);

        assert_eq!(registered_claims.jti.unwrap(), verify_vc.id);
        assert_eq!(
            registered_claims.issuer.unwrap(),
            verify_vc.issuer.to_string()
        );
        assert_eq!(
            registered_claims.subject.unwrap(),
            verify_vc.credential_subject.id
        );
        assert_eq!(
            registered_claims.not_before.unwrap(),
            rfc3339_to_timestamp(&verify_vc.issuance_date).unwrap()
        );
        assert_eq!(
            registered_claims.expiration.unwrap(),
            rfc3339_to_timestamp(&verify_vc.expiration_date.unwrap()).unwrap()
        );
    }

    #[tokio::test]
    async fn test_minimum_viable_vc_jwt() {
        let minified_vc_jwt = "eyJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkVSVFFTSXNJbU55ZGlJNklrVmtNalUxTVRraUxDSnJkSGtpT2lKUFMxQWlMQ0o0SWpvaVMyaDZNbFJFVWxScE1XeExiMHMzTkhCMlJHRk1iWE5MWmxaNFlrazVlalp3UjJKTmVXeE1iRWd6Y3lKOSMwIiwidHlwIjoiSldUIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJ0eXBlIjpbIlZlcmlmaWFibGVDcmVkZW50aWFsIl19LCJpc3MiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpFUlRRU0lzSW1OeWRpSTZJa1ZrTWpVMU1Ua2lMQ0pyZEhraU9pSlBTMUFpTENKNElqb2lTMmg2TWxSRVVsUnBNV3hMYjBzM05IQjJSR0ZNYlhOTFpsWjRZa2s1ZWpad1IySk5lV3hNYkVnemN5SjkiLCJzdWIiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpFUlRRU0lzSW1OeWRpSTZJa1ZrTWpVMU1Ua2lMQ0pyZEhraU9pSlBTMUFpTENKNElqb2lTMmg2TWxSRVVsUnBNV3hMYjBzM05IQjJSR0ZNYlhOTFpsWjRZa2s1ZWpad1IySk5lV3hNYkVnemN5SjkiLCJleHAiOjIzNDc0NjQ3MTQsIm5iZiI6MTcxNjMxMjcxNCwianRpIjoidXJuOnZjOnV1aWQ6NjE5NWRhOTEtY2RiYi00NzJkLWFlNjktYjAwNGU0OWE5ZjUxIn0.uhFoQ-coZ1sfzaNzFfWOmEDWWJwuCs9hDw0yw1pq2HgMinvvCdcarvQ9sbVN9At0oqQhhSEYwaUT42Tlyi7FBw";

        let jwt_decoded = Jwt::verify::<VcJwtClaims>(&minified_vc_jwt).await.unwrap();
        let registered_claims = jwt_decoded.claims.registered_claims;

        let verify_result = VerifiableCredential::verify(minified_vc_jwt).await;
        let verify_vc = verify_result.unwrap();

        assert_eq!(
            vec!["https://www.w3.org/2018/credentials/v1".to_string()],
            verify_vc.context
        );
        assert_eq!(vec!["VerifiableCredential".to_string()], verify_vc.r#type);

        assert_eq!(registered_claims.jti.unwrap(), verify_vc.id);
        assert_eq!(
            registered_claims.issuer.unwrap(),
            verify_vc.issuer.to_string()
        );
        assert_eq!(
            registered_claims.subject.unwrap(),
            verify_vc.credential_subject.id
        );
        assert_eq!(
            registered_claims.not_before.unwrap(),
            rfc3339_to_timestamp(&verify_vc.issuance_date).unwrap()
        );
        assert_eq!(
            registered_claims.expiration.unwrap(),
            rfc3339_to_timestamp(&verify_vc.expiration_date.unwrap()).unwrap()
        );
    }
}
