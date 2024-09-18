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

/// Represents a Decentralized Identifier (DID) along with its DID document and key manager.
///
/// This struct provides functions to interact with the DID, such as signing data and exporting
/// the DID into a portable format.
#[derive(Clone)]
pub struct BearerDid {
    /// The Decentralized Identifier (DID).
    pub did: Did,
    /// The DID Document associated with the DID.
    pub document: Document,
    /// Manages cryptographic keys associated with the DID.
    pub key_manager: Arc<dyn KeyManager>,
}

impl BearerDid {
    /// Creates a `BearerDid` from a given `PortableDid`.
    ///
    /// This allows you to instantiate a `BearerDid` using an existing DID's key material,
    /// DID document, and metadata.
    ///
    /// # Arguments
    ///
    /// * `portable_did` - The `PortableDid` to import.
    ///
    /// # Returns
    ///
    /// * `Result<Self>` - A `BearerDid` instance or an error if parsing or key import fails.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let portable_did = PortableDid {
    ///             did_uri: self.did.uri.clone(),
    ///             document: self.document.clone(),
    ///             private_jwks,
    ///         }
    /// let bearer_did = BearerDid::from_portable_did(portable_did)?;
    /// ```
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

    /// Returns a signer for the specified verification method ID.
    ///
    /// This signer can be used to sign data using a key associated with the DID.
    ///
    /// # Arguments
    ///
    /// * `verification_method_id` - The ID of the verification method to use.
    ///
    /// # Returns
    ///
    /// * `Result<Arc<dyn Signer>>` - A signer instance or an error if the verification method is invalid.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let verification_method_id = "did:example:123#key-1";
    /// let signer = bearer_did.get_signer(verification_method_id)?;
    /// let data = b"Hello, world!";
    /// let signature = signer.sign(data)?;
    /// ```
    pub fn get_signer(&self, verification_method_id: &str) -> Result<Arc<dyn Signer>> {
        if verification_method_id.is_empty() {
            return Err(Web5Error::Parameter(
                "verification_method_id cannot be empty".to_string(),
            ));
        }

        let public_jwk = self
            .document
            .find_verification_method(FindVerificationMethodOptions {
                verification_method_id: Some(verification_method_id.to_string()),
            })?
            .public_key_jwk;
        self.key_manager.get_signer(public_jwk)
    }

    /// Exports the `BearerDid` into a `PortableDid`.
    ///
    /// This method serializes the DID, its document, and associated private keys for transport or storage.
    ///
    /// # Arguments
    ///
    /// * `key_exporter` - An instance of `KeyExporter` used to export private keys.
    ///
    /// # Returns
    ///
    /// * `Result<PortableDid>` - A `PortableDid` instance or an error if key export fails.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use crate::BearerDid;
    /// use crate::crypto::key_managers::KeyExporter;
    ///
    /// let key_exporter = /* ... */;
    /// let portable_did = bearer_did.to_portable_did(key_exporter)?;
    /// ```
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

    mod from_portable_did {
        use super::*;

        #[test]
        fn test_can_instantiate_successfully() {
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

        #[test]
        fn test_verification_method_id_empty() {
            let bearer_did = DidJwk::create(None).unwrap();

            let result = bearer_did.get_signer("");
            assert!(result.is_err());
            if let Err(Web5Error::Parameter(msg)) = result {
                assert_eq!(msg, "verification_method_id cannot be empty".to_string());
            } else {
                panic!("Expected Web5Error::Parameter, got something else");
            }
        }

        #[test]
        fn test_not_found_by_verification_method_id() {
            let bearer_did = DidJwk::create(None).unwrap();

            let result = bearer_did.get_signer("something invalid");
            assert!(result.is_err());
            if let Err(Web5Error::NotFound(msg)) = result {
                assert_eq!(msg, "verification method not found".to_string());
            } else {
                panic!("Expected Web5Error::NotFound, got something else");
            }
        }

        #[test]
        fn test_found_by_verification_method_id() {
            let bearer_did = DidJwk::create(None).unwrap();

            let result = bearer_did.get_signer(&bearer_did.document.verification_method[0].id);
            assert!(result.is_ok());
        }
    }

    mod to_portable_did {
        use super::*;

        #[test]
        fn test_can_export() {
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