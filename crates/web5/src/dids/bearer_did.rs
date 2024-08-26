use super::{
    data_model::document::{Document, FindVerificationMethodOptions},
    did::Did,
    portable_did::PortableDid,
};
use crate::{
    crypto::{
        dsa::Signer,
        key_managers::{in_memory_key_manager::InMemoryKeyManager, KeyExporter, KeyManager},
    },
    errors::{Result, Web5Error},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct BearerDid {
    pub did: Did,
    pub document: Document,
    pub key_manager: Arc<dyn KeyManager>,
}

#[derive(Default)]
pub struct BearerDidGetSignerOptions {
    pub verification_method_id: Option<String>,
}

impl BearerDid {
    pub fn from_portable_did(portable_did: PortableDid) -> Result<Self> {
        let did = Did::parse(&portable_did.did_uri)?;

        let key_manager = Arc::new(InMemoryKeyManager::new());
        for private_jwk in portable_did.private_jwks {
            key_manager.import_private_jwk(private_jwk)?;
        }

        Ok(Self {
            did,
            document: portable_did.document,
            key_manager,
        })
    }

    pub fn get_signer(&self, options: BearerDidGetSignerOptions) -> Result<Arc<dyn Signer>> {
        let verification_method_id = options.verification_method_id.unwrap_or_default();
        if verification_method_id.is_empty() {
            return Err(Web5Error::Parameter(
                "no option satisfies query requirements".to_string(),
            ));
        }

        let public_jwk = self
            .document
            .find_verification_method(FindVerificationMethodOptions {
                verification_method_id: Some(verification_method_id),
            })?
            .public_key_jwk;
        self.key_manager.get_signer(public_jwk)
    }

    pub fn to_portable_did(&self, key_exporter: Arc<dyn KeyExporter>) -> Result<PortableDid> {
        let private_jwks = key_exporter.export_private_jwks()?;
        Ok(PortableDid {
            did_uri: self.did.uri.clone(),
            document: self.document.clone(),
            private_jwks,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dids::methods::did_jwk::{DidJwk, DidJwkCreateOptions};
    use crate::{test_helpers::UnitTestSuite, test_name};
    use lazy_static::lazy_static;

    mod from_portable_did {
        use super::*;

        lazy_static! {
            static ref TEST_SUITE: UnitTestSuite =
                UnitTestSuite::new("bearer_did_from_portable_did");
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
        fn test_can_instantiate_successfully() {
            TEST_SUITE.include(test_name!());

            let key_manager = Arc::new(InMemoryKeyManager::new());
            let did_jwk_bearer_did = DidJwk::create(Some(DidJwkCreateOptions {
                key_manager: Some(key_manager.clone()),
                ..Default::default()
            }))
            .unwrap();

            let portable_did = PortableDid {
                did_uri: did_jwk_bearer_did.did.uri,
                document: did_jwk_bearer_did.document,
                private_jwks: key_manager.export_private_jwks().unwrap(),
            };

            let result = BearerDid::from_portable_did(portable_did);
            assert!(result.is_ok());
        }
    }

    mod get_signer {
        use super::*;

        lazy_static! {
            static ref TEST_SUITE: UnitTestSuite = UnitTestSuite::new("bearer_did_get_signer");
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
        fn test_query_requirements_not_satisfied() {
            TEST_SUITE.include(test_name!());

            let bearer_did = DidJwk::create(None).unwrap();

            let result = bearer_did.get_signer(BearerDidGetSignerOptions {
                ..Default::default()
            });
            assert!(result.is_err());
            if let Err(Web5Error::Parameter(msg)) = result {
                assert_eq!(msg, "no option satisfies query requirements".to_string());
            } else {
                panic!("Expected Web5Error::Parameter, got something else");
            }
        }

        #[test]
        fn test_not_found_by_verification_method_id() {
            TEST_SUITE.include(test_name!());

            let bearer_did = DidJwk::create(None).unwrap();

            let verification_method_id = "something not valid".to_string();
            let result = bearer_did.get_signer(BearerDidGetSignerOptions {
                verification_method_id: Some(verification_method_id),
            });
            assert!(result.is_err());
            if let Err(Web5Error::NotFound(msg)) = result {
                assert_eq!(msg, "verification method not found".to_string());
            } else {
                panic!("Expected Web5Error::NotFound, got something else");
            }
        }

        #[test]
        fn test_found_by_verification_method_id() {
            TEST_SUITE.include(test_name!());

            let bearer_did = DidJwk::create(None).unwrap();

            let result = bearer_did.get_signer(BearerDidGetSignerOptions {
                verification_method_id: Some(bearer_did.document.verification_method[0].id.clone()),
            });
            assert!(result.is_ok());
        }
    }

    mod to_portable_did {
        use super::*;

        lazy_static! {
            static ref TEST_SUITE: UnitTestSuite = UnitTestSuite::new("bearer_did_to_portable_did");
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
        fn test_can_export() {
            TEST_SUITE.include(test_name!());

            let key_manager = Arc::new(InMemoryKeyManager::new());
            let bearer_did = DidJwk::create(Some(DidJwkCreateOptions {
                key_manager: Some(key_manager.clone()),
                ..Default::default()
            }))
            .unwrap();

            let result = bearer_did.to_portable_did(key_manager);
            assert!(result.is_ok());
            let portable_did = result.unwrap();
            assert_eq!(bearer_did.did.uri, portable_did.did_uri);
        }
    }
}
