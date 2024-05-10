use dids::{bearer::BearerDid, document::KeySelector};
use jws::v2::JwsError;
use jwt::{
    jws::Jwt,
    lib_v2::{Claims, JwtError, RegisteredClaims},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use std::time::{SystemTime, UNIX_EPOCH};

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
    #[error("Invalid expiration date")]
    InvalidExpirationDate
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
                issuer: Some(self.issuer.clone()),
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
    pub async fn verify(jwt: &str) -> Result<Self, CredentialError> {
        let jwt_decoded = Jwt::verify::<VcJwtClaims>(jwt).await?;
        let mut vc = jwt_decoded.claims.vc;
        let reg_claims = jwt_decoded.claims.registered_claims;

        // Check each claim using the helper function
        check_claim(&mut vc.issuer, reg_claims.issuer, CredentialError::MissingIss, CredentialError::IssuerMismatch)?;
        check_claim(&mut vc.issuance_date, reg_claims.not_before, CredentialError::MissingNbf, CredentialError::IssuanceDateMismatch)?;
        check_claim(&mut vc.credential_subject.id, reg_claims.subject, CredentialError::MissingSub, CredentialError::SubjectMismatch)?;
        check_claim(&mut vc.id, reg_claims.jti, CredentialError::MissingJti, CredentialError::IdMismatch)?;

        // Additional checks for expiration dates
        if let (Some(exp), Some(vc_exp)) = (reg_claims.expiration, vc.expiration_date) {
            if exp != vc_exp {
                return Err(CredentialError::ExpirationDateMismatch);
            }

            // Check if the current time is after the expiration date
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;

            if now > vc_exp {
                return Err(CredentialError::CredentialExpired(
                    "The verifiable credential has expired".to_string(),
                ));
            }
        }
        if reg_claims.expiration.is_none() && vc.expiration_date.is_some() {
            return Err(CredentialError::MisconfiguredExpirationDate(
                "VC has expiration date but no exp in registered claims".to_string(),
            ));
        }

        // It is important to validate after the claims check because the vc data model may be updated from the claims
        validate_vc_data_model(&vc).map_err(CredentialError::ValidationError)?;

        Ok(vc)
    }

    pub fn decode(jwt: &str) -> Result<Self, CredentialError> {
        let jwt_decoded = Jwt::decode::<VcJwtClaims>(jwt)?;
        Ok(jwt_decoded.claims.vc)
    }
}

fn validate_vc_data_model(vc: &VerifiableCredential) -> Result<(), CredentialDataModelValidationError> {

    // Required fields
    if vc.id.is_empty() {
        return Err(CredentialDataModelValidationError::MissingId);
    }

    if vc.context.first().map_or(true, |v| v != BASE_CONTEXT) {
        return Err(CredentialDataModelValidationError::MissingContext);
    }

    if vc.r#type.first().map_or(true, |v| v != BASE_TYPE) {
        return Err(CredentialDataModelValidationError::MissingType);
    }

    if vc.issuer.is_empty() {
        return Err(CredentialDataModelValidationError::MissingIssuer);
    }

    if vc.issuance_date.is_negative() {
        return Err(CredentialDataModelValidationError::InvalidIssuanceDate);
    }

    // Validate expiration date if it exists
    if let Some(expiration_date) = vc.expiration_date {
        if expiration_date.is_negative() {
            return Err(CredentialDataModelValidationError::InvalidExpirationDate);
        }
    }

    // TODO: Add validations to credential_status, credential_schema, and evidence once they are added to the VcDataModel
    // https://github.com/TBD54566975/web5-rs/issues/112

    Ok(())
}

/// Validates or updates a property of a verifiable credential (VC) based on a JWT claim.
///
/// This function checks a JWT claim and compares it against a corresponding property in the VC.
/// If the VC property is either unset or set to its default value, the property is updated to the value of the JWT claim.
/// If the VC property is already set and does not match the JWT claim, an error is returned.
fn check_claim<T: PartialEq + Clone + Default>(
    vc_property: &mut T,
    jwt_claim: Option<T>,
    error_on_missing: CredentialError,
    error_on_mismatch: CredentialError
) -> Result<(), CredentialError> {
    match jwt_claim {
        Some(claim_value) => {
            if *vc_property == T::default() {
                // If the current value is default, update it without error.
                *vc_property = claim_value;
            } else if *vc_property != claim_value {
                // If the value is not default and does not match the JWT claim, return mismatch error.
                return Err(error_on_mismatch);
            }
            Ok(())
        },
        None => Err(error_on_missing),
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
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
            Method,
        },
    };
    use keys::key_manager::local_key_manager::LocalKeyManager;
    use std::time::{SystemTime, UNIX_EPOCH};
    use uuid::Uuid;

    fn create_bearer_did() -> BearerDid {
        let key_manager = Arc::new(LocalKeyManager::new_in_memory());
        let options = DidJwkCreateOptions {
            curve: Curve::Ed25519,
        };
        let bearer_did = DidJwk::create(key_manager, options).unwrap();
        bearer_did
    }

    fn create_vc(issuer: &str) -> VerifiableCredential {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        VerifiableCredential::new(
            vec![BASE_CONTEXT.to_string()],
            format!("urn:vc:uuid:{0}", Uuid::new_v4().to_string()),
            vec![BASE_TYPE.to_string()],
            issuer.to_string(),
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
        let vc = create_vc(&bearer_did.identifier.uri);
        assert_eq!(1, vc.context.len());
        assert_ne!("", vc.id);
        assert_eq!(1, vc.r#type.len());
        assert_eq!(vc.issuer, bearer_did.identifier.uri);
    }

    #[test]
    fn test_new() {
        let issuer = "did:jwk:something";
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let vc1 = VerifiableCredential::new(
            vec![BASE_CONTEXT.to_string()],
            format!("urn:vc:uuid:{0}", Uuid::new_v4().to_string()),
            vec![BASE_TYPE.to_string()],
            issuer.to_string(),
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
            vec!["some-other-context".to_string()],
            format!("urn:vc:uuid:{0}", Uuid::new_v4().to_string()),
            vec!["some-other-type".to_string()],
            issuer.to_string(),
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
    }

    #[tokio::test]
    async fn test_sign_and_verify() {
        let bearer_did = create_bearer_did();
        let vc = create_vc(&bearer_did.identifier.uri);
        let key_selector = KeySelector::MethodType {
            verification_method_type: VerificationMethodType::VerificationMethod,
        };
        let vcjwt = vc.sign(&bearer_did, &key_selector).unwrap();
        assert!(!vcjwt.is_empty());

        let verified_vc = VerifiableCredential::verify(&vcjwt).await.unwrap();
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

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let vc = VerifiableCredential::new(
            vec![BASE_CONTEXT.to_string()],
            format!("urn:vc:uuid:{0}", Uuid::new_v4().to_string()),
            vec![BASE_TYPE.to_string()],
            bearer_did.identifier.uri.to_string(),
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
        assert!(matches!(result, Err(CredentialError::CredentialExpired(_))), "Expected expiration error, but found different or no error");
    }

    #[tokio::test]
    async fn test_verify_mismatched_iss() {
        let mismatched_issuer_vc_jwt = "eyJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkVSVFFTSXNJbU55ZGlJNklrVmtNalUxTVRraUxDSnJkSGtpT2lKUFMxQWlMQ0o0SWpvaUxUaHpjVVYyYzBkZk5TMXNVRGxaWVd0aWIyNVRNRzAxUkZsVmFrbDVObTg0UWw5VmQzUnphbXhWT0NKOSMwIiwidHlwIjoiSldUIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp2Yzp1dWlkOjQwNmYxNjhlLTg4Y2QtNGVhMS05ZTBmLWFkZTUyMDFjODY4YyIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiXSwiaXNzdWVyIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaRVJUUVNJc0ltTnlkaUk2SWtWa01qVTFNVGtpTENKcmRIa2lPaUpQUzFBaUxDSjRJam9pTFRoemNVVjJjMGRmTlMxc1VEbFpZV3RpYjI1VE1HMDFSRmxWYWtsNU5tODRRbDlWZDNSemFteFZPQ0o5IiwiaXNzdWFuY2VEYXRlIjoxNzE1MzU4NjQ2LCJleHBpcmF0aW9uRGF0ZSI6MTcxNTMyODY0NiwiY3JlZGVudGlhbF9zdWJqZWN0Ijp7ImlkIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaRVJUUVNJc0ltTnlkaUk2SWtWa01qVTFNVGtpTENKcmRIa2lPaUpQUzFBaUxDSjRJam9pTFRoemNVVjJjMGRmTlMxc1VEbFpZV3RpYjI1VE1HMDFSRmxWYWtsNU5tODRRbDlWZDNSemFteFZPQ0o5In19LCJpc3MiOiJ3cm9uZ2lzc3VlciIsInN1YiI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkVSVFFTSXNJbU55ZGlJNklrVmtNalUxTVRraUxDSnJkSGtpT2lKUFMxQWlMQ0o0SWpvaUxUaHpjVVYyYzBkZk5TMXNVRGxaWVd0aWIyNVRNRzAxUkZsVmFrbDVObTg0UWw5VmQzUnphbXhWT0NKOSIsImV4cCI6MTcxNTMyODY0NiwibmJmIjoxNzE1MzU4NjQ2LCJqdGkiOiJ1cm46dmM6dXVpZDo0MDZmMTY4ZS04OGNkLTRlYTEtOWUwZi1hZGU1MjAxYzg2OGMifQ.gX3trvOMBzRX3vC2t1d3FEDj4RFNVrmotvIFgrLPoJVP2co4arz8jRT_VQ9-g7CRqWQ65uyhgAMQjZ_HWwk2DA";

        let result = VerifiableCredential::verify(&mismatched_issuer_vc_jwt).await;
        assert!(matches!(result, Err(CredentialError::IssuerMismatch)), "Expected mismatch issuer error, but found different or no error");
    }
}
