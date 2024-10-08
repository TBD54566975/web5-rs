use super::{KeyExporter, KeyManager};
use crate::{
    crypto::{
        dsa::{ed25519::Ed25519Signer, secp256k1::Secp256k1Signer, Dsa, Signer},
        jwk::Jwk,
    },
    errors::{Result, Web5Error},
};
use std::{
    collections::HashMap,
    str::FromStr,
    sync::{Arc, RwLock},
};

/// An in-memory implementation of the `KeyManager` and `KeyExporter` traits.
///
/// The `InMemoryKeyManager` provides a simple, thread-safe key management solution
/// that stores private keys in memory. It allows for importing private JWKs, retrieving
/// signers for public JWKs, and exporting the stored private JWKs.
///
/// # Examples
///
/// ```ignore
/// let key_manager = InMemoryKeyManager::new();
/// let private_jwk = Ed25519Generator::generate();
/// let public_jwk = key_manager.import_private_jwk(private_jwk.clone()).unwrap();
/// let signer = key_manager.get_signer(public_jwk).unwrap();
/// ```
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
    /// Imports a private JWK into the key manager and returns the corresponding public JWK.
    ///
    /// This method adds the private JWK to the internal key map and returns the public JWK
    /// (with the private component removed).
    ///
    /// # Arguments
    ///
    /// * `private_jwk` - The private JWK to import.
    ///
    /// # Returns
    ///
    /// * `Result<Jwk>` - The public JWK.
    ///
    /// # Errors
    ///
    /// Returns an error if the provided JWK is not a private key.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let key_manager = InMemoryKeyManager::new();
    /// let private_jwk = Ed25519Generator::generate();
    /// let public_jwk = key_manager.import_private_jwk(private_jwk.clone()).unwrap();
    /// ```
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

    /// Retrieves a signer for a given public JWK.
    ///
    /// This method looks up the private JWK associated with the given public JWK
    /// and returns a signer for cryptographic operations.
    ///
    /// # Arguments
    ///
    /// * `public_jwk` - The public JWK for which to retrieve the signer.
    ///
    /// # Returns
    ///
    /// * `Result<Arc<dyn Signer>>` - The signer associated with the public JWK.
    ///
    /// # Errors
    ///
    /// Returns an error if the public JWK is not found or if the JWK is not a public key.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let key_manager = InMemoryKeyManager::new();
    /// let private_jwk = Ed25519Generator::generate();
    /// let public_jwk = key_manager.import_private_jwk(private_jwk.clone()).unwrap();
    /// let signer = key_manager.get_signer(public_jwk).unwrap();
    /// ```
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

        let signer: Arc<dyn Signer> = match Dsa::from_str(&public_jwk.crv)? {
            Dsa::Ed25519 => Arc::new(Ed25519Signer::new(private_jwk.clone())),
            Dsa::Secp256k1 => Arc::new(Secp256k1Signer::new(private_jwk.clone())),
        };
        Ok(signer)
    }
}

impl KeyExporter for InMemoryKeyManager {
    /// Exports all private JWKs stored in the key manager.
    ///
    /// This method returns all the private keys that have been stored in the key manager.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<Jwk>>` - A list of private JWKs.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let private_jwks = key_manager.export_private_jwks()?;
    /// ```
    fn export_private_jwks(&self) -> Result<Vec<Jwk>> {
        let map_lock = self.map.read()?;
        let jwks = map_lock.values().cloned().collect();
        Ok(jwks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::dsa::ed25519::Ed25519Generator;

    mod import_private_jwk {
        use super::*;

        #[test]
        fn test_must_be_private_jwk() {
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

        #[test]
        fn test_must_be_public_key() {
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

    mod export_private_jwks {
        use super::*;

        #[test]
        fn test_export_empty_list() {
            let key_manager = InMemoryKeyManager::new();
            let result = key_manager.export_private_jwks().unwrap();
            assert!(result.is_empty());
        }

        #[test]
        fn test_export_single_key() {
            let key_manager = InMemoryKeyManager::new();
            let private_jwk = Ed25519Generator::generate();
            key_manager.import_private_jwk(private_jwk.clone()).unwrap();

            let result = key_manager.export_private_jwks().unwrap();
            assert_eq!(result.len(), 1);
            assert_eq!(result[0], private_jwk);
        }

        #[test]
        fn test_export_multiple_keys() {
            let key_manager = InMemoryKeyManager::new();
            let private_jwk1 = Ed25519Generator::generate();
            let private_jwk2 = Ed25519Generator::generate();

            key_manager
                .import_private_jwk(private_jwk1.clone())
                .unwrap();
            key_manager
                .import_private_jwk(private_jwk2.clone())
                .unwrap();

            let result = key_manager.export_private_jwks().unwrap();
            assert_eq!(result.len(), 2);
            assert!(result.contains(&private_jwk1));
            assert!(result.contains(&private_jwk2));
        }
    }
}
