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
    sync::Arc,
};

const BASE_CONTEXT: &str = "https://www.w3.org/2018/credentials/v1";
const BASE_TYPE: &str = "VerifiableCredential";

#[derive(thiserror::Error, Debug)]
pub enum CredentialError {
    #[error(transparent)]
    JwtError(#[from] JwtError),
    #[error(transparent)]
    JwsError(#[from] JwsError),
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
        context: Vec<String>,
        id: String,
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

    pub async fn verify(jwt: &str) -> Result<Self, CredentialError> {
        let jwt_decoded = Jwt::verify::<VcJwtClaims>(jwt).await?;

        // TODO Implement semantic VC verification rules https://github.com/TBD54566975/web5-rs/issues/151

        Ok(jwt_decoded.claims.vc)
    }

    pub fn decode(jwt: &str) -> Result<Self, CredentialError> {
        let jwt_decoded = Jwt::decode::<VcJwtClaims>(jwt)?;

        Ok(jwt_decoded.claims.vc)
    }
}

// todo we should remove this altogether in the follow-up PR, but it would break bindings so leaving it for now
pub async fn verify_vcjwt(jwt: &str) -> Result<Arc<VerifiableCredential>, CredentialError> {
    let jwt_decoded = Jwt::verify::<VcJwtClaims>(jwt).await?;
    Ok(Arc::new(jwt_decoded.claims.vc))
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
        methods::{
            jwk::{DidJwk, DidJwkCreateOptions},
            Create,
        },
    };
    use keys::key_manager::local_key_manager::LocalKeyManager;
    use std::time::{SystemTime, UNIX_EPOCH};
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
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        VerifiableCredential::new(
            vec![BASE_CONTEXT.to_string()],
            format!("urn:vc:uuid:{0}", Uuid::new_v4().to_string()),
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
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let vc1 = VerifiableCredential::new(
            vec![BASE_CONTEXT.to_string()],
            format!("urn:vc:uuid:{0}", Uuid::new_v4().to_string()),
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
            vec!["some-other-context".to_string()],
            format!("urn:vc:uuid:{0}", Uuid::new_v4().to_string()),
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
            vec![BASE_CONTEXT.to_string()],
            format!("urn:vc:uuid:{0}", Uuid::new_v4().to_string()),
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
        let vcjwt = vc.sign(&bearer_did, &key_selector).unwrap();
        assert!(!vcjwt.is_empty());

        let verified_vc = VerifiableCredential::verify(&vcjwt).await.unwrap();
        assert_eq!(vc.id, verified_vc.id);
        assert_eq!(vc.issuer, verified_vc.issuer);
        assert_eq!(vc.credential_subject.id, verified_vc.credential_subject.id);
    }
}
