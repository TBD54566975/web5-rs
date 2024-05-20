use chrono::Utc;
use core::fmt;
use dids::{bearer::BearerDid, document::KeySelector};
use jws::JwsError;
use jwt::{
    jws::Jwt,
    {Claims, JwtError, RegisteredClaims},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

const BASE_CONTEXT: &str = "https://www.w3.org/2018/credentials/v1";
const BASE_TYPE: &str = "VerifiableCredential";

#[derive(thiserror::Error, Debug)]
pub enum CredentialError {
    // JWT specific errors
    #[error(transparent)]
    JwtError(#[from] JwtError),
    #[error(transparent)]
    JwsError(#[from] JwsError),

    // Credential-specific validation errors
    #[error("Expiration date mismatch")]
    ExpirationDateMismatch,
    #[error("Credential expired: {0}")]
    CredentialExpired(String),
    #[error("Issuer mismatch")]
    IssuerMismatch,
    #[error("Issuance date is in the future")]
    IssuanceDateInFuture,
    #[error("Issuance date mismatch")]
    IssuanceDateMismatch,
    #[error("Credential ID mismatch")]
    IdMismatch,
    #[error("Subject mismatch")]
    SubjectMismatch,
    #[error("Missing issuer")]
    MissingIssuer,
    #[error("Missing issuance date")]
    MissingIssuanceDate,
    #[error("Missing subject")]
    MissingSubject,
    #[error("Missing credential ID")]
    MissingId,
    #[error("Misconfigured expiration date: {0}")]
    MisconfiguredExpirationDate(String),

    // Data model validation error
    #[error("Missing Iss")]
    MissingIss,
    #[error("Missing Nbf")]
    MissingNbf,
    #[error("Missing Sub")]
    MissingSub,
    #[error("Missing Jti")]
    MissingJti,
    #[error("Validation error: {0}")]
    ValidationError(#[from] CredentialDataModelValidationError),
}

#[derive(thiserror::Error, Debug)]
pub enum CredentialDataModelValidationError {
    #[error("Missing context")]
    MissingContext,
    #[error("Missing ID")]
    MissingId,
    #[error("Missing type")]
    MissingType,
    #[error("Missing issuer")]
    MissingIssuer,
    #[error("Missing issuance date")]
    MissingIssuanceDate,
    #[error("Invalid issuance date")]
    InvalidIssuanceDate,
    #[error("Issuance date in future")]
    IssuanceDateInFuture,
    #[error("Invalid expiration date")]
    InvalidExpirationDate,
    #[error("Credential expired")]
    CredentialExpired,
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
    pub issuance_date: i64,
    #[serde(rename = "expirationDate")]
    pub expiration_date: Option<i64>,
    pub credential_subject: CredentialSubject,
}

impl VerifiableCredential {
    fn to_jwt_payload_verifiable_credential(self) -> JwtPayloadVerifiableCredential {
        JwtPayloadVerifiableCredential {
            context: self.context,
            id: Some(self.id),
            r#type: self.r#type,
            issuer: Some(self.issuer),
            issuance_date: Some(self.issuance_date),
            expiration_date: self.expiration_date,
            credential_subject: Some(self.credential_subject),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct JwtPayloadVerifiableCredential {
    #[serde(rename = "@context")]
    context: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename = "type")]
    r#type: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer: Option<Issuer>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "issuanceDate")]
    issuance_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "expirationDate")]
    expiration_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "credentialSubject")]
    credential_subject: Option<CredentialSubject>,
}

impl JwtPayloadVerifiableCredential {
    fn to_verifiable_credential(self) -> VerifiableCredential {
        VerifiableCredential {
            context: self.context,
            id: self.id.unwrap_or_default(),
            r#type: self.r#type,
            issuer: self.issuer.unwrap(),
            issuance_date: self.issuance_date.unwrap_or_default(),
            expiration_date: self.expiration_date,
            credential_subject: self.credential_subject.unwrap(),
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
        issuance_date: i64,
        expiration_date: Option<i64>,
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
        let claims = VcJwtClaims {
            registered_claims: RegisteredClaims {
                issuer: Some(self.issuer.to_string()),
                jti: Some(self.id.clone()),
                subject: Some(self.credential_subject.id.clone()),
                not_before: Some(self.issuance_date),
                expiration: self.expiration_date,
                ..Default::default()
            },
            vc_payload: self.clone().to_jwt_payload_verifiable_credential(),
        };

        let jwt = Jwt::sign(bearer_did, key_selector, None, &claims)?;

        Ok(jwt)
    }

    pub async fn verify(jwt: &str) -> Result<VerifiableCredential, CredentialError> {
        let jwt_decoded = Jwt::verify::<VcJwtClaims>(jwt).await?;
        let vc_payload = jwt_decoded.claims.vc_payload;
        let registered_claims = jwt_decoded.claims.registered_claims;

        // registered claims checks
        let jti = registered_claims.jti.ok_or(CredentialError::MissingJti)?;
        let iss = registered_claims
            .issuer
            .ok_or(CredentialError::MissingIss)?;
        let sub = registered_claims
            .subject
            .ok_or(CredentialError::MissingSub)?;
        let nbf = registered_claims
            .not_before
            .ok_or(CredentialError::MissingNbf)?;
        let exp = registered_claims.expiration;

        if let Some(id) = &vc_payload.id {
            if id != &jti {
                return Err(CredentialError::IdMismatch);
            }
        }

        if let Some(issuer) = &vc_payload.issuer {
            let vc_issuer = issuer.to_string();
            if iss != vc_issuer {
                return Err(CredentialError::IssuerMismatch);
            }
        }

        if let Some(credential_subject) = &vc_payload.credential_subject {
            if sub != credential_subject.id {
                return Err(CredentialError::SubjectMismatch);
            }
        }

        if let Some(issuance_date) = &vc_payload.issuance_date {
            if issuance_date != &nbf {
                return Err(CredentialError::IssuanceDateMismatch);
            }
        }

        if let Some(expiration_date) = &vc_payload.expiration_date {
            if exp.is_none() {
                return Err(CredentialError::MisconfiguredExpirationDate(
                    "VC has expiration date but no exp in registered claims".to_string(),
                ));
            }

            if expiration_date != &exp.unwrap() {
                return Err(CredentialError::IssuanceDateMismatch);
            }
        }

        if exp.is_some() {
            let now = Utc::now().timestamp();
            if now > exp.unwrap() {
                return Err(CredentialError::CredentialExpired(
                    "The verifiable credential has expired".to_string(),
                ));
            }
        }

        let mut vc_issuer = Issuer::String(iss.clone());

        if let Some(issuer) = vc_payload.issuer.as_ref() {
            if let Issuer::Object(_) = issuer {
                vc_issuer = vc_payload.issuer.clone().unwrap();
            }
        }

        let vc_credential_subject =
            vc_payload
                .credential_subject
                .clone()
                .unwrap_or_else(|| CredentialSubject {
                    id: sub,
                    params: None,
                });

        let vc = VerifiableCredential {
            context: vc_payload.context.clone(),
            id: jti.clone(),
            r#type: vc_payload.r#type.clone(),
            issuer: vc_issuer.clone(),
            issuance_date: nbf.clone(),
            expiration_date: exp.clone(),
            credential_subject: vc_credential_subject,
        };

        validate_vc_data_model(&vc).map_err(CredentialError::ValidationError)?;

        Ok(vc)
    }

    pub fn decode(jwt: &str) -> Result<Self, CredentialError> {
        let jwt_decoded = Jwt::decode::<VcJwtClaims>(jwt)?;

        let vc = jwt_decoded.claims.vc_payload.to_verifiable_credential();
        Ok(vc)
    }
}

fn validate_vc_data_model(
    vc: &VerifiableCredential,
) -> Result<(), CredentialDataModelValidationError> {
    // Required fields
    if vc.id.is_empty() {
        return Err(CredentialDataModelValidationError::MissingId);
    }

    if vc.context.is_empty() || vc.context[0] != BASE_CONTEXT {
        return Err(CredentialDataModelValidationError::MissingContext);
    }

    if vc.r#type.is_empty() || vc.r#type[0] != BASE_TYPE {
        return Err(CredentialDataModelValidationError::MissingType);
    }

    if vc.issuer.to_string().is_empty() {
        return Err(CredentialDataModelValidationError::MissingIssuer);
    }

    let now = Utc::now().timestamp();

    if vc.issuance_date.is_negative() {
        return Err(CredentialDataModelValidationError::InvalidIssuanceDate);
    }

    if vc.issuance_date > now {
        return Err(CredentialDataModelValidationError::IssuanceDateInFuture);
    }

    // Validate expiration date if it exists
    if let Some(expiration_date) = vc.expiration_date {
        if expiration_date.is_negative() {
            return Err(CredentialDataModelValidationError::InvalidExpirationDate);
        }

        if expiration_date < now {
            return Err(CredentialDataModelValidationError::CredentialExpired);
        }
    }

    // TODO: Add validations to credential_status, credential_schema, and evidence once they are added to the VcDataModel
    // https://github.com/TBD54566975/web5-rs/issues/112

    Ok(())
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
    use crypto::Curve;
    use dids::{
        document::VerificationMethodType,
        methods::{
            jwk::{DidJwk, DidJwkCreateOptions},
            Create,
        },
    };
    use keys::key_manager::local_key_manager::LocalKeyManager;
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
            now,
            Some(now + 30 * 60),
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
            now,
            Some(now + 30 * 60),
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
            now,
            Some(now + 30 * 60),
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
            now,
            Some(now + 30 * 60),
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
            now,
            Some(now.clone() - 300000),
            CredentialSubject {
                id: bearer_did.identifier.uri.to_string(),
                ..Default::default()
            },
        );

        let vc_jwt = vc.sign(&bearer_did, &key_selector).unwrap();
        assert!(!vc_jwt.is_empty());

        let result = VerifiableCredential::verify(&vc_jwt).await;
        assert!(
            matches!(result, Err(CredentialError::CredentialExpired(_))),
            "Expected expiration error, but found different or no error"
        );
    }

    #[tokio::test]
    async fn test_verify_mismatched_iss() {
        let mismatched_issuer_vc_jwt = "eyJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkVSVFFTSXNJbU55ZGlJNklrVmtNalUxTVRraUxDSnJkSGtpT2lKUFMxQWlMQ0o0SWpvaUxUaHpjVVYyYzBkZk5TMXNVRGxaWVd0aWIyNVRNRzAxUkZsVmFrbDVObTg0UWw5VmQzUnphbXhWT0NKOSMwIiwidHlwIjoiSldUIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp2Yzp1dWlkOjQwNmYxNjhlLTg4Y2QtNGVhMS05ZTBmLWFkZTUyMDFjODY4YyIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaRVJUUVNJc0ltTnlkaUk2SWtWa01qVTFNVGtpTENKcmRIa2lPaUpQUzFBaUxDSjRJam9pTFRoemNVVjJjMGRmTlMxc1VEbFpZV3RpYjI1VE1HMDFSRmxWYWtsNU5tODRRbDlWZDNSemFteFZPQ0o5IiwiaXNzdWFuY2VEYXRlIjoxNzE1MzU4NjQ2LCJleHBpcmF0aW9uRGF0ZSI6MTcxNTMyODY0NiwiY3JlZGVudGlhbF9zdWJqZWN0Ijp7ImlkIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaRVJUUVNJc0ltTnlkaUk2SWtWa01qVTFNVGtpTENKcmRIa2lPaUpQUzFBaUxDSjRJam9pTFRoemNVVjJjMGRmTlMxc1VEbFpZV3RpYjI1VE1HMDFSRmxWYWtsNU5tODRRbDlWZDNSemFteFZPQ0o5In19LCJpc3MiOiJ3cm9uZ2lzc3VlciIsInN1YiI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkVSVFFTSXNJbU55ZGlJNklrVmtNalUxTVRraUxDSnJkSGtpT2lKUFMxQWlMQ0o0SWpvaUxUaHpjVVYyYzBkZk5TMXNVRGxaWVd0aWIyNVRNRzAxUkZsVmFrbDVObTg0UWw5VmQzUnphbXhWT0NKOSIsImV4cCI6MTcxNTMyODY0NiwibmJmIjoxNzE1MzU4NjQ2LCJqdGkiOiJ1cm46dmM6dXVpZDo0MDZmMTY4ZS04OGNkLTRlYTEtOWUwZi1hZGU1MjAxYzg2OGMifQ.gX3trvOMBzRX3vC2t1d3FEDj4RFNVrmotvIFgrLPoJVP2co4arz8jRT_VQ9-g7CRqWQ65uyhgAMQjZ_HWwk2DA";
        let result = VerifiableCredential::verify(&mismatched_issuer_vc_jwt).await;
        assert!(
            matches!(result, Err(CredentialError::IssuerMismatch)),
            "Expected mismatch issuer error, but found different or no error"
        );
    }

    #[tokio::test]
    async fn test_full_featured_vc_jwt() {
        let full_featured_vc_jwt = "eyJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkVSVFFTSXNJbU55ZGlJNklrVmtNalUxTVRraUxDSnJkSGtpT2lKUFMxQWlMQ0o0SWpvaVZHUjVTWGRIUlRobFIyZElNM1J2WkZBMGFrMW1WMHhTYUUxa1FrZzRjVzU2YVc0eFpVNVJVbHB1TUNKOSMwIiwidHlwIjoiSldUIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp2Yzp1dWlkOmRkMDU2NjdkLThkODItNDI5ZS1iMzJiLWM1NTdkMjBhNDc5MSIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaRVJUUVNJc0ltTnlkaUk2SWtWa01qVTFNVGtpTENKcmRIa2lPaUpQUzFBaUxDSjRJam9pVkdSNVNYZEhSVGhsUjJkSU0zUnZaRkEwYWsxbVYweFNhRTFrUWtnNGNXNTZhVzR4WlU1UlVscHVNQ0o5IiwiaXNzdWFuY2VEYXRlIjoxNzE2MjM5MjU1LCJleHBpcmF0aW9uRGF0ZSI6MTcxNjI0MTA1NSwiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpFUlRRU0lzSW1OeWRpSTZJa1ZrTWpVMU1Ua2lMQ0pyZEhraU9pSlBTMUFpTENKNElqb2lWR1I1U1hkSFJUaGxSMmRJTTNSdlpGQTBhazFtVjB4U2FFMWtRa2c0Y1c1NmFXNHhaVTVSVWxwdU1DSjkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkVSVFFTSXNJbU55ZGlJNklrVmtNalUxTVRraUxDSnJkSGtpT2lKUFMxQWlMQ0o0SWpvaVZHUjVTWGRIUlRobFIyZElNM1J2WkZBMGFrMW1WMHhTYUUxa1FrZzRjVzU2YVc0eFpVNVJVbHB1TUNKOSIsInN1YiI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkVSVFFTSXNJbU55ZGlJNklrVmtNalUxTVRraUxDSnJkSGtpT2lKUFMxQWlMQ0o0SWpvaVZHUjVTWGRIUlRobFIyZElNM1J2WkZBMGFrMW1WMHhTYUUxa1FrZzRjVzU2YVc0eFpVNVJVbHB1TUNKOSIsImV4cCI6MTcxNjI0MTA1NSwibmJmIjoxNzE2MjM5MjU1LCJqdGkiOiJ1cm46dmM6dXVpZDpkZDA1NjY3ZC04ZDgyLTQyOWUtYjMyYi1jNTU3ZDIwYTQ3OTEifQ.Zw7YOuSWQNODPNwhRiRy5qAZg_yutxCSxFW_WJ6knkiu8jvtO921tsRjXBGukPbotUgDWBFt-OQMdbkWcRZhCw";

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
            verify_vc.issuance_date
        );
        assert_eq!(
            registered_claims.expiration.unwrap(),
            verify_vc.expiration_date.unwrap()
        );
    }

    #[tokio::test]
    async fn test_minimum_viable_vc_jwt() {
        let minified_vc_jwt = "eyJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkVSVFFTSXNJbU55ZGlJNklrVmtNalUxTVRraUxDSnJkSGtpT2lKUFMxQWlMQ0o0SWpvaVRETmpjbmd0UlVsQk1rVXRiRVJ5TjJFdFoyNTBVRGRFYlZWU01qSnlXRWc0ZUVSUmNXbFVXSG96TUNKOSMwIiwidHlwIjoiSldUIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJ0eXBlIjpbIlZlcmlmaWFibGVDcmVkZW50aWFsIl19LCJpc3MiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpFUlRRU0lzSW1OeWRpSTZJa1ZrTWpVMU1Ua2lMQ0pyZEhraU9pSlBTMUFpTENKNElqb2lURE5qY25ndFJVbEJNa1V0YkVSeU4yRXRaMjUwVURkRWJWVlNNakp5V0VnNGVFUlJjV2xVV0hvek1DSjkiLCJzdWIiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpFUlRRU0lzSW1OeWRpSTZJa1ZrTWpVMU1Ua2lMQ0pyZEhraU9pSlBTMUFpTENKNElqb2lURE5qY25ndFJVbEJNa1V0YkVSeU4yRXRaMjUwVURkRWJWVlNNakp5V0VnNGVFUlJjV2xVV0hvek1DSjkiLCJleHAiOjE3MTYyMzk4ODksIm5iZiI6MTcxNjIzODA4OSwianRpIjoidXJuOnZjOnV1aWQ6MWFlNjY0YjktOTY3MC00YmUwLThkNjQtNDgxZGY1M2RjMDVhIn0.3i-oArdmC8h_2UPqo61Krx9K3lys9dtxsUgn-TEdTykH_UAA1pfOyEL9cD7LuNQhvo8NFbBXuLZLIlc0Yr0PAA";

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
            verify_vc.issuance_date
        );
        assert_eq!(
            registered_claims.expiration.unwrap(),
            verify_vc.expiration_date.unwrap()
        );
    }
}
