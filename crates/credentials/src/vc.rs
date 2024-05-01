use base64::{engine::general_purpose, Engine as _};
use dids::{bearer::BearerDid, document::KeySelector};
use jws::{splice_parts, JwsError, JwsHeader};
use jwt::{sign_jwt, verify_jwt, Claims, JwtError};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

const BASE_CONTEXT: &str = "https://www.w3.org/2018/credentials/v1";
const BASE_TYPE: &str = "VerifiableCredential";

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
        println!("KW DBG sign bearer did vm {:?}", bearer_did.document.verification_method);
        let header = JwsHeader::from_bearer_did(bearer_did, key_selector, "JWT")?;
        let claims = VcJwtClaims {
            base_claims: Claims {
                issuer: Some(self.issuer.clone()),
                jti: Some(self.id.clone()),
                subject: Some(self.credential_subject.id.clone()),
                not_before: Some(self.issuance_date),
                expiration: self.expiration_date,
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
    verify_jwt(jwt).await?;
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

        let verified_vc = verify_vcjwt(&vcjwt).await.unwrap();
        assert_eq!(vc.id, verified_vc.id);
        assert_eq!(vc.issuer, verified_vc.issuer);
        assert_eq!(vc.credential_subject.id, verified_vc.credential_subject.id);
    }

    #[tokio::test]
    async fn test_create_and_verify() {
        let bearer_did = create_bearer_did();
        let issuer = bearer_did.identifier.uri.clone();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let vc = VerifiableCredential::new(
            vec![BASE_CONTEXT.to_string()],
            format!("urn:vc:uuid:{0}", Uuid::new_v4().to_string()),
            vec![BASE_TYPE.to_string(), "StreetCred".to_string()],
            issuer.to_string(),
            now,
            None,
            CredentialSubject {
                id: issuer.to_string(),
                ..Default::default()
            },
        );
        let signed_vcjwt = vc
            .sign(
                &bearer_did,
                &KeySelector::MethodType {
                    verification_method_type: VerificationMethodType::VerificationMethod,
                },
            )
            .unwrap();
        println!("{:?}", signed_vcjwt);
        verify_vcjwt(&signed_vcjwt).await.unwrap();
    }

    #[tokio::test]
    async fn test_verify() {
        let vcjwt = "eyJhbGciOiJFZERTQSIsImtpZCI6ImRpZDpqd2s6ZXlKcmRIa2lPaUpQUzFBaUxDSmpjbllpT2lKRlpESTFOVEU1SWl3aVlXeG5Jam9pUldSRVUwRWlMQ0o0SWpvaVpHRkliWG93UWxKaU9XbEZXVUpTYWxkTE1VZHJaa2h6V1hKT1ZGbDJRVjlVVDI5SE5tUkRVM05sVFNKOSMwIiwidHlwIjoiSldUIn0.eyJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSJdLCJpZCI6InVybjp1dWlkOmRjOGU3NTg2LTk5YjEtNDRjMi1iZWM4LWM3ZDMzMGVhODIxYiIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiLCJTdHJlZXRDcmVkIl0sImlzc3VlciI6ImRpZDpqd2s6ZXlKcmRIa2lPaUpQUzFBaUxDSmpjbllpT2lKRlpESTFOVEU1SWl3aVlXeG5Jam9pUldSRVUwRWlMQ0o0SWpvaVpHRkliWG93UWxKaU9XbEZXVUpTYWxkTE1VZHJaa2h6V1hKT1ZGbDJRVjlVVDI5SE5tUkRVM05sVFNKOSIsImlzc3VhbmNlRGF0ZSI6MTcxNDU5MTA5NjAwMCwiZXhwaXJhdGlvbkRhdGUiOm51bGwsImNyZWRlbnRpYWxfc3ViamVjdCI6eyJpZCI6ImRpZDpqd2s6ZXlKcmRIa2lPaUpQUzFBaUxDSmpjbllpT2lKRlpESTFOVEU1SWl3aVlXeG5Jam9pUldSRVUwRWlMQ0o0SWpvaVpGRlFVMFZNWTBwclpGUXRObDlJZUdkMVQySm1ablpHY1VKbk9HVm1hRTVmUkVKMldYcDBja3hhWnlKOSJ9fSwiaXNzIjoiZGlkOmp3azpleUpyZEhraU9pSlBTMUFpTENKamNuWWlPaUpGWkRJMU5URTVJaXdpWVd4bklqb2lSV1JFVTBFaUxDSjRJam9pWkdGSWJYb3dRbEppT1dsRldVSlNhbGRMTVVkclpraHpXWEpPVkZsMlFWOVVUMjlITm1SRFUzTmxUU0o5Iiwic3ViIjoiZGlkOmp3azpleUpyZEhraU9pSlBTMUFpTENKamNuWWlPaUpGWkRJMU5URTVJaXdpWVd4bklqb2lSV1JFVTBFaUxDSjRJam9pWkZGUVUwVk1ZMHByWkZRdE5sOUllR2QxVDJKbVpuWkdjVUpuT0dWbWFFNWZSRUoyV1hwMGNreGFaeUo5IiwibmJmIjoxNzE0NTkxMDk2MDAwLCJqdGkiOiJ1cm46dXVpZDpkYzhlNzU4Ni05OWIxLTQ0YzItYmVjOC1jN2QzMzBlYTgyMWIifQ.-_hJeOOfudfx8w_h83r2lD2HLF1fVUHAgJpDAlR2eLIPQmzzOJEPA15xLNpFWe6OoOoFSWy28FtYsgrM1r-nAg";
        let verified_vc = verify_vcjwt(vcjwt).await.unwrap();
        println!("{:?}", verified_vc)
    }
}
