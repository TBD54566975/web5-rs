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
            vc: self.clone(),
        };

        let jwt = Jwt::sign(bearer_did, key_selector, None, &claims)?;

        Ok(jwt)
    }

    pub async fn verify(jwt: &str) -> Result<VerifiableCredential, CredentialError> {
        let jwt_decoded = Jwt::verify::<VcJwtClaims>(jwt).await?;
        let mut vc = jwt_decoded.claims.vc;
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

        // TODO: Change this to None after this issue is resolved - https://github.com/TBD54566975/web5-rs/issues/202
        // check claims match expected values
        if vc.id == String::default() {
            vc.id = jti;
        } else if vc.id != jti {
            return Err(CredentialError::IdMismatch);
        }

        let vc_issuer = vc.issuer.to_string();
        if vc_issuer.is_empty() {
            vc.issuer = Issuer::String(iss.clone());
        } else if iss != vc_issuer {
            return Err(CredentialError::IssuerMismatch);
        }

        if vc.credential_subject.id == String::default() {
            vc.credential_subject.id = sub;
        } else if vc.credential_subject.id != sub {
            return Err(CredentialError::SubjectMismatch);
        }

        if vc.issuance_date == i64::default() {
            vc.issuance_date = nbf;
        } else if vc.issuance_date != nbf {
            return Err(CredentialError::IssuanceDateMismatch);
        }

        // if exp exists, make sure there is not a mismatch and assign it to vc expiration date
        // if vc expiration date exists and exp does not, throw a misconfigured exp error
        if exp.is_some() {
            if vc.expiration_date.is_some() {
                if vc.expiration_date != exp {
                    return Err(CredentialError::ExpirationDateMismatch);
                }

                let now = Utc::now().timestamp();
                if now > exp.unwrap() {
                    return Err(CredentialError::CredentialExpired(
                        "The verifiable credential has expired".to_string(),
                    ));
                }

                vc.expiration_date = exp;
            }
        } else if vc.expiration_date.is_some() {
            return Err(CredentialError::MisconfiguredExpirationDate(
                "VC has expiration date but no exp in registered claims".to_string(),
            ));
        }

        validate_vc_data_model(&vc).map_err(CredentialError::ValidationError)?;

        Ok(vc)
    }

    pub fn decode(jwt: &str) -> Result<Self, CredentialError> {
        let jwt_decoded = Jwt::decode::<VcJwtClaims>(jwt)?;
        Ok(jwt_decoded.claims.vc)
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
    pub vc: VerifiableCredential,
    #[serde(flatten)]
    pub registered_claims: RegisteredClaims,
}

impl Claims for VcJwtClaims {}

#[cfg(test)]
mod test {
    use super::*;
    use crypto::Curve;
    use dids::{
        document::VerificationMethodType,
        method::{
            jwk::{DidJwk, DidJwkCreateOptions},
            Create,
        },
    };
    use keys::key_manager::local_key_manager::LocalKeyManager;
    use std::sync::Arc;
    use uuid::Uuid;

    fn create_bearer_did() -> BearerDid {
        let key_manager = Arc::new(LocalKeyManager::new_in_memory());
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

    // TODO: Add this test after doing this issuer - https://github.com/TBD54566975/web5-rs/issues/202
    // #[tokio::test]
    // async fn test_minified_jwt() {
    //     let minified_vc_jwt = "eyJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkVSVFFTSXNJbU55ZGlJNklrVmtNalUxTVRraUxDSnJkSGtpT2lKUFMxQWlMQ0o0SWpvaVJFNU5kRGhyWVVOUGFubFNWbms0UVdoSGJWVmxkbTUzUkZGTlJYTlVkemxQY2s1M05DMHlOWFJ5VlNKOSMwIiwidHlwIjoiSldUIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6IiIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiIiwiaXNzdWFuY2VEYXRlIjowLCJleHBpcmF0aW9uRGF0ZSI6MCwiY3JlZGVudGlhbF9zdWJqZWN0Ijp7ImlkIjoiIn19LCJpc3MiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpFUlRRU0lzSW1OeWRpSTZJa1ZrTWpVMU1Ua2lMQ0pyZEhraU9pSlBTMUFpTENKNElqb2lSRTVOZERocllVTlBhbmxTVm5rNFFXaEhiVlZsZG01M1JGRk5SWE5VZHpsUGNrNTNOQzB5TlhSeVZTSjkiLCJzdWIiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpFUlRRU0lzSW1OeWRpSTZJa1ZrTWpVMU1Ua2lMQ0pyZEhraU9pSlBTMUFpTENKNElqb2lSRTVOZERocllVTlBhbmxTVm5rNFFXaEhiVlZsZG01M1JGRk5SWE5VZHpsUGNrNTNOQzB5TlhSeVZTSjkiLCJleHAiOjE3MTUzNzA1OTIsIm5iZiI6MTcxNTM2ODc5MiwianRpIjoidXJuOnZjOnV1aWQ6MTMwMjMwOWMtMDcyOS00YTAwLThmNDAtOTNkZjc3ZDQxODg5In0.u5SCPyx6Una88BYmztZ3-fbWnfDHCXIU6vBHva0SZtZQ8CYUaSjMvWRRCYY7j99JgHZU7R5wPHR1f7sb10qEBw";
    //
    //     let jwt_decoded = Jwt::verify::<VcJwtClaims>(&minified_vc_jwt).await.unwrap();
    //     let registered_claims = jwt_decoded.claims.registered_claims;
    //
    //     let verify_result = VerifiableCredential::verify(minified_vc_jwt).await;
    //     let verify_vc = verify_result.unwrap();
    //
    //     assert_eq!(registered_claims.jti.unwrap(), verify_vc.id);
    //     assert_eq!(registered_claims.issuer.unwrap(), verify_vc.issuer.get_id());
    //     assert_eq!(registered_claims.subject.unwrap(), verify_vc.credential_subject.id);
    //     assert_eq!(registered_claims.not_before.unwrap(), verify_vc.issuance_date);
    //     assert_eq!(registered_claims.expiration.unwrap(), verify_vc.expiration_date.unwrap());
    // }
}
