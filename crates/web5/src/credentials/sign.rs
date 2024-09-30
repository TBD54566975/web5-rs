use super::{
    jwt_payload_vc::JwtPayloadVerifiableCredential, verifiable_credential_1_1::VerifiableCredential,
};
use crate::{
    dids::bearer_did::BearerDid,
    errors::{Result, Web5Error},
    jose::{Jwt, JwtClaims},
    json::{JsonValue, ToJsonValue},
};
use std::{collections::HashMap, time::SystemTime};

pub fn sign_with_did(
    vc: &VerifiableCredential,
    bearer_did: &BearerDid,
    verification_method_id: Option<String>,
) -> Result<String> {
    if !vc.issuer.to_string().starts_with(&bearer_did.did.uri) {
        return Err(Web5Error::Parameter(format!(
            "Bearer DID URI {} does not match issuer {}",
            bearer_did.did.uri, vc.issuer
        )));
    }

    let vc_claim = JwtPayloadVerifiableCredential {
        context: vc.context.clone(),
        id: Some(vc.id.clone()),
        r#type: vc.r#type.clone(),
        issuer: Some(vc.issuer.clone()),
        issuance_date: Some(vc.issuance_date),
        expiration_date: vc.expiration_date,
        credential_status: vc.credential_status.clone(),
        credential_subject: Some(vc.credential_subject.clone()),
        credential_schema: vc.credential_schema.clone(),
        evidence: vc.evidence.clone(),
    };

    let mut additional_properties: HashMap<String, JsonValue> = HashMap::new();
    additional_properties.insert("vc".to_string(), vc_claim.to_json_value()?);

    let claims = JwtClaims {
        aud: None,
        iss: Some(vc.issuer.to_string()),
        jti: Some(vc.id.clone()),
        sub: Some(vc.credential_subject.id.clone()),
        nbf: Some(vc.issuance_date),
        iat: Some(SystemTime::now()),
        exp: vc.expiration_date,
        additional_properties: Some(additional_properties),
    };

    let jwt = Jwt::from_claims(&claims, bearer_did, verification_method_id)?;
    Ok(jwt.compact_jws)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::credentials::credential_subject::CredentialSubject;

    const SUBJECT_DID_URI: &str = "did:dht:qgmmpyjw5hwnqfgzn7wmrm33ady8gb8z9ideib6m9gj4ys6wny8y";

    fn credential_subject() -> CredentialSubject {
        CredentialSubject::from(SUBJECT_DID_URI)
    }

    mod sign {
        use crate::{credentials::issuer::Issuer, dids::methods::did_jwk::DidJwk};

        use super::*;

        #[tokio::test]
        async fn test_can_sign_then_verify() {
            let bearer_did = DidJwk::create(None).unwrap();
            let vc = VerifiableCredential::create(
                Issuer::String(bearer_did.did.uri.clone()),
                credential_subject(),
                Default::default(),
            )
            .await
            .unwrap();

            let vc_jwt =
                sign_with_did(&vc, &bearer_did, None).expect("should be able to sign vc jwt");

            let vc_from_vc_jwt = VerifiableCredential::from_vc_jwt(&vc_jwt, true)
                .await
                .expect("should be able to verify the signed vc jwt");
            assert_eq!(vc.id, vc_from_vc_jwt.id)
        }

        #[tokio::test]
        async fn test_bearer_did_mismatch_issuer() {
            let bearer_did = DidJwk::create(None).unwrap();
            let vc = VerifiableCredential::create(
                Issuer::String(bearer_did.did.uri.clone()),
                credential_subject(),
                Default::default(),
            )
            .await
            .unwrap();

            let different_bearer_did = DidJwk::create(None).unwrap();
            let result = sign_with_did(&vc, &different_bearer_did, None);

            match result {
                Err(Web5Error::Parameter(err_msg)) => {
                    assert_eq!(
                        err_msg,
                        format!(
                            "Bearer DID URI {} does not match issuer {}",
                            different_bearer_did.did.uri, bearer_did.did.uri
                        )
                    )
                }
                _ => panic!("Expected Web5Error::Parameter, but got: {:?}", result),
            };
        }

        #[tokio::test]
        async fn test_defaults_to_first_vm() {
            let bearer_did = DidJwk::create(None).unwrap();
            let vc = VerifiableCredential::create(
                Issuer::String(bearer_did.did.uri.clone()),
                credential_subject(),
                Default::default(),
            )
            .await
            .unwrap();

            let vc_jwt =
                sign_with_did(&vc, &bearer_did, None).expect("should sign with default vm");

            let kid = Jwt::from_compact_jws(&vc_jwt, false).await.unwrap().kid;

            assert_eq!(bearer_did.document.verification_method[0].id, kid)
        }

        #[tokio::test]
        async fn test_vm_must_be_assertion_method() {
            let mut bearer_did = DidJwk::create(None).unwrap();
            let vc = VerifiableCredential::create(
                Issuer::String(bearer_did.did.uri.clone()),
                credential_subject(),
                Default::default(),
            )
            .await
            .unwrap();

            // remove the assertionMethod
            if let Some(assertion_method) = bearer_did.document.assertion_method.as_mut() {
                assertion_method.remove(0);
            }

            let vm_id = bearer_did.document.verification_method[0].id.clone();

            let result = sign_with_did(&vc, &bearer_did, Some(vm_id.clone()));

            match result {
                Err(Web5Error::Parameter(err_msg)) => {
                    assert_eq!(
                        err_msg,
                        format!(
                            "verification_method_id {} is not an assertion_method",
                            vm_id
                        )
                    )
                }
                _ => panic!("expected Web5Error::Parameter but got {:?}", result),
            }
        }
    }
}
