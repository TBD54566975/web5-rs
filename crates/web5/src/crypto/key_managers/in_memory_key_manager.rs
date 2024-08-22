use super::KeyManager;
use crate::{
    crypto::{
        dsa::{ed25519::Ed25519Signer, Signer},
        jwk::Jwk,
    },
    errors::{Result, Web5Error},
};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[derive(Default)]
pub struct InMemoryKeyManager {
    map: RwLock<HashMap<String, Jwk>>,
}

impl Clone for InMemoryKeyManager {
    fn clone(&self) -> Self {
        let cloned_map = self.map.read().unwrap().clone();
        InMemoryKeyManager {
            map: RwLock::new(cloned_map),
        }
    }
}

impl InMemoryKeyManager {
    pub fn new() -> Self {
        Self {
            map: RwLock::new(HashMap::new()),
        }
    }
}

impl KeyManager for InMemoryKeyManager {
    fn import_private_jwk(&self, private_jwk: Jwk) -> Result<Jwk> {
        if private_jwk.is_public_key() {
            return Err(Web5Error::Parameter(
                "private_jwk must be a private key".to_string(),
            ));
        }

        let mut public_jwk = private_jwk.clone();
        public_jwk.d = None;

        let mut map_lock = self.map.write()?;
        map_lock.insert(public_jwk.compute_thumbprint()?, private_jwk);
        Ok(public_jwk)
    }

    fn get_signer(&self, public_jwk: Jwk) -> Result<Arc<dyn Signer>> {
        if !public_jwk.is_public_key() {
            return Err(Web5Error::Parameter(
                "public_jwk must be a public key".to_string(),
            ));
        }

        let map_lock = self.map.read()?;
        let thumbprint = public_jwk.compute_thumbprint()?;
        let private_jwk = map_lock
            .get(&thumbprint)
            .ok_or(Web5Error::NotFound(format!(
                "signer not found for public_jwk with thumbprint {}",
                thumbprint
            )))?;
        Ok(Arc::new(Ed25519Signer::new(private_jwk.clone())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::dsa::ed25519::Ed25519Generator;
    use crate::{test_helpers::UnitTestSuite, test_name};
    use std::sync::LazyLock;

    mod import_private_jwk {
        use super::*;

        static TEST_SUITE: LazyLock<UnitTestSuite> =
            LazyLock::new(|| UnitTestSuite::new("in_memory_key_manager_import_private_jwk"));

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
        fn test_must_be_private_jwk() {
            TEST_SUITE.include(test_name!());

            let key_manager = InMemoryKeyManager::new();
            let private_jwk = Ed25519Generator::generate();
            let mut public_jwk = private_jwk.clone();
            public_jwk.d = None;

            let result = key_manager.import_private_jwk(public_jwk);
            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err(),
                Web5Error::Parameter("private_jwk must be a private key".to_string())
            );
        }

        #[test]
        fn test_successfully_imports_and_returns_public_jwk() {
            TEST_SUITE.include(test_name!());

            let key_manager = InMemoryKeyManager::new();
            let private_jwk = Ed25519Generator::generate();

            let result = key_manager.import_private_jwk(private_jwk);
            assert!(result.is_ok());

            let public_jwk = result.unwrap();
            assert!(public_jwk.is_public_key());
        }
    }

    mod get_signer {
        use super::*;

        static TEST_SUITE: LazyLock<UnitTestSuite> =
            LazyLock::new(|| UnitTestSuite::new("in_memory_key_manager_get_signer"));

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
        fn test_must_be_public_key() {
            TEST_SUITE.include(test_name!());

            let key_manager = InMemoryKeyManager::new();
            let private_jwk = Ed25519Generator::generate();
            let result = key_manager.get_signer(private_jwk);
            assert!(result.is_err());
            if let Err(Web5Error::Parameter(err_msg)) = result {
                assert_eq!(err_msg, "public_jwk must be a public key".to_string());
            } else {
                panic!("Expected a Parameter error");
            }
        }

        #[test]
        fn test_not_found() {
            TEST_SUITE.include(test_name!());

            let key_manager = InMemoryKeyManager::new();
            let mut public_jwk = Ed25519Generator::generate();
            public_jwk.d = None;
            let result = key_manager.get_signer(public_jwk.clone());
            assert!(result.is_err());
            if let Err(Web5Error::NotFound(err_msg)) = result {
                assert_eq!(
                    err_msg,
                    format!(
                        "signer not found for public_jwk with thumbprint {}",
                        public_jwk.compute_thumbprint().unwrap()
                    )
                );
            } else {
                panic!("Expected a Parameter error");
            }
        }

        #[test]
        fn test_found() {
            TEST_SUITE.include(test_name!());

            let key_manager = InMemoryKeyManager::new();
            let private_jwk = Ed25519Generator::generate();
            let mut public_jwk = private_jwk.clone();
            public_jwk.d = None;

            key_manager.import_private_jwk(private_jwk).unwrap();

            let result = key_manager.get_signer(public_jwk);
            assert!(result.is_ok());
            result.unwrap();
        }
    }
}
