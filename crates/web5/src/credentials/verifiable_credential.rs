use crate::dids::{bearer::BearerDid, document::KeySelector};
use crate::jwt::{
    jws::Jwt,
    {Claims, JwtError, RegisteredClaims},
};
use chrono::Utc;
use core::fmt;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
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
    issuance_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "expirationDate")]
    expiration_date: Option<i64>,
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
            vc_payload: self.clone().into(),
        };

        let jwt = Jwt::sign(bearer_did, key_selector, None, &claims)?;

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

        if let Some(issuance_date) = &vc_payload.issuance_date {
            if issuance_date != &nbf {
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
                    if vc_payload_expiration_date != &exp {
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

        let vc = VerifiableCredential {
            context: vc_payload.context,
            id: jti,
            r#type: vc_payload.r#type,
            issuer: vc_issuer,
            issuance_date: nbf,
            expiration_date: exp,
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
    // Required fields
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

    let now = Utc::now().timestamp();

    if vc.issuance_date.is_negative() {
        return Err(CredentialError::VcDataModelValidationError(
            "invalid issuance date".to_string(),
        ));
    }

    if vc.issuance_date > now {
        return Err(CredentialError::VcDataModelValidationError(
            "issuance date in future".to_string(),
        ));
    }

    // Validate expiration date if it exists
    if let Some(expiration_date) = vc.expiration_date {
        if expiration_date.is_negative() {
            return Err(CredentialError::VcDataModelValidationError(
                "invalid expiration date".to_string(),
            ));
        }

        if expiration_date < now {
            return Err(CredentialError::VcDataModelValidationError(
                "credential expired".to_string(),
            ));
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
            now,
            Some(now + 631152000), // now + 20 years
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
            matches!(result, Err(CredentialError::CredentialExpired)),
            "Expected expiration error, but found different or no error"
        );
    }

    #[tokio::test]
    async fn test_verify_mismatched_iss() {
        let mismatched_issuer_vc_jwt = "eyJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkVSVFFTSXNJbU55ZGlJNklrVmtNalUxTVRraUxDSnJkSGtpT2lKUFMxQWlMQ0o0SWpvaUxUaHpjVVYyYzBkZk5TMXNVRGxaWVd0aWIyNVRNRzAxUkZsVmFrbDVObTg0UWw5VmQzUnphbXhWT0NKOSMwIiwidHlwIjoiSldUIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp2Yzp1dWlkOjQwNmYxNjhlLTg4Y2QtNGVhMS05ZTBmLWFkZTUyMDFjODY4YyIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaRVJUUVNJc0ltTnlkaUk2SWtWa01qVTFNVGtpTENKcmRIa2lPaUpQUzFBaUxDSjRJam9pTFRoemNVVjJjMGRmTlMxc1VEbFpZV3RpYjI1VE1HMDFSRmxWYWtsNU5tODRRbDlWZDNSemFteFZPQ0o5IiwiaXNzdWFuY2VEYXRlIjoxNzE1MzU4NjQ2LCJleHBpcmF0aW9uRGF0ZSI6MTcxNTMyODY0NiwiY3JlZGVudGlhbF9zdWJqZWN0Ijp7ImlkIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaRVJUUVNJc0ltTnlkaUk2SWtWa01qVTFNVGtpTENKcmRIa2lPaUpQUzFBaUxDSjRJam9pTFRoemNVVjJjMGRmTlMxc1VEbFpZV3RpYjI1VE1HMDFSRmxWYWtsNU5tODRRbDlWZDNSemFteFZPQ0o5In19LCJpc3MiOiJ3cm9uZ2lzc3VlciIsInN1YiI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkVSVFFTSXNJbU55ZGlJNklrVmtNalUxTVRraUxDSnJkSGtpT2lKUFMxQWlMQ0o0SWpvaUxUaHpjVVYyYzBkZk5TMXNVRGxaWVd0aWIyNVRNRzAxUkZsVmFrbDVObTg0UWw5VmQzUnphbXhWT0NKOSIsImV4cCI6MTcxNTMyODY0NiwibmJmIjoxNzE1MzU4NjQ2LCJqdGkiOiJ1cm46dmM6dXVpZDo0MDZmMTY4ZS04OGNkLTRlYTEtOWUwZi1hZGU1MjAxYzg2OGMifQ.gX3trvOMBzRX3vC2t1d3FEDj4RFNVrmotvIFgrLPoJVP2co4arz8jRT_VQ9-g7CRqWQ65uyhgAMQjZ_HWwk2DA";
        let result = VerifiableCredential::verify(&mismatched_issuer_vc_jwt).await;

        assert!(
            matches!(result, Err(CredentialError::ClaimMismatch(ref s)) if s == "issuer"),
            "Expected mismatch issuer error, but found different or no error"
        );
    }

    #[tokio::test]
    async fn test_full_featured_vc_jwt() {
        let full_featured_vc_jwt = "eyJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkVSVFFTSXNJbU55ZGlJNklrVmtNalUxTVRraUxDSnJkSGtpT2lKUFMxQWlMQ0o0SWpvaU5XOUNaRmhNTjNSRFdDMWlXbXd3Tm5VNVdXUlNXakJhYWxKTExVcHhWV1poWmtWM1owMHRUR0ptYXlKOSMwIiwidHlwIjoiSldUIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp2Yzp1dWlkOmUzMDc0OWVhLTg4YjctNDkwMi05ZTRlLWYwYjk1MTRjZmU1OSIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaRVJUUVNJc0ltTnlkaUk2SWtWa01qVTFNVGtpTENKcmRIa2lPaUpQUzFBaUxDSjRJam9pTlc5Q1pGaE1OM1JEV0MxaVdtd3dOblU1V1dSU1dqQmFhbEpMTFVweFZXWmhaa1YzWjAwdFRHSm1heUo5IiwiaXNzdWFuY2VEYXRlIjoxNzE2MzEyNDU3LCJleHBpcmF0aW9uRGF0ZSI6MjM0NzQ2NDQ1NywiY3JlZGVudGlhbFN1YmplY3QiOnsiaWQiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpFUlRRU0lzSW1OeWRpSTZJa1ZrTWpVMU1Ua2lMQ0pyZEhraU9pSlBTMUFpTENKNElqb2lOVzlDWkZoTU4zUkRXQzFpV213d05uVTVXV1JTV2pCYWFsSkxMVXB4VldaaFprVjNaMDB0VEdKbWF5SjkifX0sImlzcyI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkVSVFFTSXNJbU55ZGlJNklrVmtNalUxTVRraUxDSnJkSGtpT2lKUFMxQWlMQ0o0SWpvaU5XOUNaRmhNTjNSRFdDMWlXbXd3Tm5VNVdXUlNXakJhYWxKTExVcHhWV1poWmtWM1owMHRUR0ptYXlKOSIsInN1YiI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkVSVFFTSXNJbU55ZGlJNklrVmtNalUxTVRraUxDSnJkSGtpT2lKUFMxQWlMQ0o0SWpvaU5XOUNaRmhNTjNSRFdDMWlXbXd3Tm5VNVdXUlNXakJhYWxKTExVcHhWV1poWmtWM1owMHRUR0ptYXlKOSIsImV4cCI6MjM0NzQ2NDQ1NywibmJmIjoxNzE2MzEyNDU3LCJqdGkiOiJ1cm46dmM6dXVpZDplMzA3NDllYS04OGI3LTQ5MDItOWU0ZS1mMGI5NTE0Y2ZlNTkifQ.a8ciqXyNgqttWPKl76CFwDTRvEoJEq5nndfM1UMkClvzhPOUWSUtE0wNHOxQFwUBBSbwozScBNe-dc-mWQFqAQ";

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
            verify_vc.issuance_date
        );
        assert_eq!(
            registered_claims.expiration.unwrap(),
            verify_vc.expiration_date.unwrap()
        );
    }
}
