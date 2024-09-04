use std::{sync::Arc, time::SystemTime};

use crate::{
    crypto::dsa::Signer,
    dids::bearer_did::BearerDid,
    errors::{Result, Web5Error},
};
use josekit::{jws::JwsHeader, jwt::JwtPayload};

use super::{
    josekit::JoseSigner, jwt_payload_vc::JwtPayloadVerifiableCredential,
    verifiable_credential_1_1::VerifiableCredential,
};

pub fn sign_with_signer(
    vc: &VerifiableCredential,
    key_id: &str,
    signer: Arc<dyn Signer>,
) -> Result<String> {
    let mut payload = JwtPayload::new();
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
    payload
        .set_claim("vc", Some(serde_json::to_value(vc_claim)?))
        .map_err(|e| Web5Error::Unknown(format!("failed to set claim {}", e)))?;
    payload.set_issuer(vc.issuer.to_string());
    payload.set_jwt_id(&vc.id);
    payload.set_subject(&vc.credential_subject.id);
    payload.set_not_before(&vc.issuance_date);
    payload.set_issued_at(&SystemTime::now());
    if let Some(exp) = &vc.expiration_date {
        payload.set_expires_at(exp);
    }

    let jose_signer = JoseSigner {
        kid: key_id.to_string(),
        signer,
    };

    let mut header = JwsHeader::new();
    header.set_token_type("JWT");
    let vc_jwt = josekit::jwt::encode_with_signer(&payload, &header, &jose_signer)
        .map_err(|e| Web5Error::Crypto(format!("failed to sign vc-jwt {}", e)))?;

    Ok(vc_jwt)
}

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

    let verification_method_id = verification_method_id
        .unwrap_or_else(|| bearer_did.document.verification_method[0].id.clone());

    let is_assertion_method = if let Some(assertion_methods) = &bearer_did.document.assertion_method
    {
        assertion_methods.contains(&verification_method_id)
    } else {
        false
    };

    if !is_assertion_method {
        return Err(Web5Error::Parameter(format!(
            "verification_method_id {} is not an assertion_method",
            verification_method_id
        )));
    }

    let signer = bearer_did.get_signer(&verification_method_id)?;
    sign_with_signer(vc, &verification_method_id, signer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::credentials::credential_subject::CredentialSubject;
    use crate::{test_helpers::UnitTestSuite, test_name};
    use lazy_static::lazy_static;

    const SUBJECT_DID_URI: &str = "did:dht:qgmmpyjw5hwnqfgzn7wmrm33ady8gb8z9ideib6m9gj4ys6wny8y";

    fn credential_subject() -> CredentialSubject {
        CredentialSubject::from(SUBJECT_DID_URI)
    }

    mod sign {
        use crate::{credentials::issuer::Issuer, dids::methods::did_jwk::DidJwk};

        use super::*;

        lazy_static! {
            static ref TEST_SUITE: UnitTestSuite =
                UnitTestSuite::new("verifiable_credential_1_1_sign");
        }

        #[test]
        fn z_assert_all_suite_cases_covered() {
            // fn name prefixed with `z_*` b/c rust test harness executes in alphabetical order,
            // unless intentionally executed with "shuffle" https://doc.rust-lang.org/rustc/tests/index.html#--shuffle
            // this may not work if shuffled or if test list grows to the extent of 100ms being insufficient wait time

            // wait 100ms to be last-in-queue of mutex lock
            std::thread::sleep(std::time::Duration::from_millis(100));

            TEST_SUITE.assert_coverage()
        }

        #[test]
        fn test_can_sign_then_verify() {
            TEST_SUITE.include(test_name!());

            let bearer_did = DidJwk::create(None).unwrap();
            let vc = VerifiableCredential::create(
                Issuer::String(bearer_did.did.uri.clone()),
                credential_subject(),
                Default::default(),
            )
            .unwrap();

            let vc_jwt =
                sign_with_did(&vc, &bearer_did, None).expect("should be able to sign vc jwt");

            let vc_from_vc_jwt = VerifiableCredential::from_vc_jwt(&vc_jwt, true)
                .expect("should be able to verify the signed vc jwt");
            assert_eq!(vc.id, vc_from_vc_jwt.id)
        }

        #[test]
        fn test_bearer_did_mismatch_issuer() {
            TEST_SUITE.include(test_name!());

            let bearer_did = DidJwk::create(None).unwrap();
            let vc = VerifiableCredential::create(
                Issuer::String(bearer_did.did.uri.clone()),
                credential_subject(),
                Default::default(),
            )
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

        #[test]
        fn test_defaults_to_first_vm() {
            TEST_SUITE.include(test_name!());

            let bearer_did = DidJwk::create(None).unwrap();
            let vc = VerifiableCredential::create(
                Issuer::String(bearer_did.did.uri.clone()),
                credential_subject(),
                Default::default(),
            )
            .unwrap();

            let vc_jwt =
                sign_with_did(&vc, &bearer_did, None).expect("should sign with default vm");

            let header = josekit::jwt::decode_header(vc_jwt).unwrap();

            let kid = header
                .claim("kid")
                .and_then(serde_json::Value::as_str)
                .unwrap();

            assert_eq!(bearer_did.document.verification_method[0].id, kid)
        }

        #[test]
        fn test_vm_must_be_assertion_method() {
            TEST_SUITE.include(test_name!());

            let mut bearer_did = DidJwk::create(None).unwrap();
            let vc = VerifiableCredential::create(
                Issuer::String(bearer_did.did.uri.clone()),
                credential_subject(),
                Default::default(),
            )
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
