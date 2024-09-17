use crate::{
    crypto::{
        dsa::{ed25519::Ed25519Generator, secp256k1::Secp256k1Generator, Dsa},
        jwk::Jwk,
        key_managers::{in_memory_key_manager::InMemoryKeyManager, KeyManager},
    },
    dids::{
        bearer_did::BearerDid,
        data_model::{document::Document, verification_method::VerificationMethod},
        did::Did,
        resolution::{
            resolution_metadata::ResolutionMetadataError, resolution_result::ResolutionResult,
        },
    },
    errors::Result,
};
use base64::{engine::general_purpose, Engine as _};
use std::sync::Arc;

#[derive(Default)]
pub struct DidJwkCreateOptions {
    pub key_manager: Option<Arc<dyn KeyManager>>,
    pub dsa: Option<Dsa>,
}

pub struct DidJwk;

/// Provides functionality for creating and resolving "did:jwk" method Decentralized Identifiers (DIDs).
///
/// A "did:jwk" DID is a type of DID derived directly from a public key, allowing it to be self-verifiable
/// by third parties without relying on a separate blockchain or ledger. This is particularly useful for scenarios
/// involving verifiable credentials or capabilities.
impl DidJwk {

    /// Creates a new "did:jwk" DID, derived from a public key.
    ///
    /// This method generates a "did:jwk" DID by creating a key pair, using the provided key manager, and
    /// constructing the DID document. The method-specific identifier is a base64url encoded JSON Web Key (JWK).
    ///
    /// # Arguments
    ///
    /// * `options` - Optional. Contains a `KeyManager` to store the key and a `Dsa` specifying the key type (e.g., Ed25519 or Secp256k1).
    ///
    /// # Returns
    ///
    /// * `Result<BearerDid>` - The newly created "did:jwk" DID, encapsulated in a `BearerDid` object.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let did_jwk = DidJwk::create(None)?;
    /// println!("Created DID JWK: {:?}", did_jwk);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if key generation fails or if the specified curve is not supported.
    pub fn create(options: Option<DidJwkCreateOptions>) -> Result<BearerDid> {
        let options = options.unwrap_or_default();

        let key_manager = options
            .key_manager
            .unwrap_or_else(|| Arc::new(InMemoryKeyManager::new()));

        let private_jwk = match options.dsa.unwrap_or(Dsa::Ed25519) {
            Dsa::Ed25519 => Ed25519Generator::generate(),
            Dsa::Secp256k1 => Secp256k1Generator::generate(),
        };
        let mut public_jwk = key_manager.import_private_jwk(private_jwk)?;
        public_jwk.d = None;

        let jwk_string = serde_json::to_string(&public_jwk)?;
        let method_specific_id = general_purpose::URL_SAFE_NO_PAD.encode(jwk_string);

        let did_uri = format!("did:jwk:{}", method_specific_id);

        let did = Did::parse(&did_uri)?;

        let verification_method_id = format!("{}#0", did_uri);

        let document = Document {
            context: Some(vec!["https://www.w3.org/ns/did/v1".to_string()]),
            id: did_uri.clone(),
            verification_method: vec![VerificationMethod {
                id: verification_method_id.clone(),
                r#type: "JsonWebKey".to_string(),
                controller: did_uri.clone(),
                public_key_jwk: public_jwk.clone(),
            }],
            authentication: Some(vec![verification_method_id.clone()]),
            assertion_method: Some(vec![verification_method_id.clone()]),
            capability_invocation: Some(vec![verification_method_id.clone()]),
            capability_delegation: Some(vec![verification_method_id.clone()]),
            ..Default::default()
        };

        Ok(BearerDid {
            did,
            document,
            key_manager,
        })
    }

    /// Resolves a "did:jwk" DID into a `ResolutionResult`.
    ///
    /// This method constructs a DID document from the method-specific identifier (public key) encoded in the DID.
    /// It parses the DID URI, decodes the public key, and builds a document with verification methods, authentication,
    /// and other DID properties.
    ///
    /// # Arguments
    ///
    /// * `uri` - The DID URI to resolve.
    ///
    /// # Returns
    ///
    /// * `ResolutionResult` - The result of the resolution, containing the DID document and related metadata.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let result = DidJwk::resolve("did:jwk:example123");
    /// println!("Resolved DID Document: {:?}", result.document);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `ResolutionMetadataError` if the DID is invalid or cannot be resolved.
    pub fn resolve(uri: &str) -> ResolutionResult {
        let did = match Did::parse(uri) {
            Ok(d) => d,
            Err(_) => return ResolutionResult::from(ResolutionMetadataError::InvalidDid),
        };

        let decoded_jwk = match general_purpose::URL_SAFE_NO_PAD.decode(did.id) {
            Ok(dj) => dj,
            Err(_) => return ResolutionResult::from(ResolutionMetadataError::InvalidDid),
        };

        let public_jwk = match serde_json::from_slice::<Jwk>(&decoded_jwk) {
            Ok(pj) => pj,
            Err(_) => return ResolutionResult::from(ResolutionMetadataError::InvalidDid),
        };

        let kid = format!("{}#0", did.uri);
        let document = Document {
            context: Some(vec!["https://www.w3.org/ns/did/v1".to_string()]),
            id: did.uri.clone(),
            verification_method: vec![VerificationMethod {
                id: kid.clone(),
                r#type: "JsonWebKey".to_string(),
                controller: did.uri.clone(),
                public_key_jwk: public_jwk,
            }],
            assertion_method: Some(vec![kid.clone()]),
            authentication: Some(vec![kid.clone()]),
            capability_invocation: Some(vec![kid.clone()]),
            capability_delegation: Some(vec![kid.clone()]),

            // TODO: https://github.com/TBD54566975/web5-rs/issues/257 - If the JWK contains a `use` property with the value "sig" then the `keyAgreement` property
            // is not included in the DID Document. If the `use` value is "enc" then only the `keyAgreement`
            // property is included in the DID Document.
            // key_agreement: if public_jwk.use_.as_deref() != Some("sig") { Some(vec![kid.clone()]) } else { None },
            ..Default::default()
        };

        ResolutionResult {
            document: Some(document),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{test_helpers::UnitTestSuite, test_name};
    use lazy_static::lazy_static;

    mod create {
        use super::*;

        lazy_static! {
            static ref TEST_SUITE: UnitTestSuite = UnitTestSuite::new("did_jwk_create");
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
        fn test_can_specify_key_manager() {
            TEST_SUITE.include(test_name!());

            let key_manager = Arc::new(InMemoryKeyManager::new());
            let result = DidJwk::create(Some(DidJwkCreateOptions {
                key_manager: Some(key_manager.clone()),
                ..Default::default()
            }));

            assert!(result.is_ok());

            let bearer_did = result.unwrap();
            let public_jwk = bearer_did.document.verification_method[0]
                .public_key_jwk
                .clone();
            let result = key_manager.get_signer(public_jwk);
            assert!(result.is_ok())
        }

        #[test]
        fn test_can_specify_secp256k1() {
            TEST_SUITE.include(test_name!());

            let result = DidJwk::create(Some(DidJwkCreateOptions {
                dsa: Some(Dsa::Secp256k1),
                ..Default::default()
            }));

            assert!(result.is_ok());

            let bearer_did = result.unwrap();
            let public_jwk = bearer_did.document.verification_method[0]
                .public_key_jwk
                .clone();
            assert_eq!(public_jwk.alg, Some("ES256K".to_string()));
            assert_eq!(public_jwk.kty, "EC".to_string());
            assert_eq!(public_jwk.crv, "secp256k1".to_string());
        }

        #[test]
        fn test_defaults_to_ed25519() {
            TEST_SUITE.include(test_name!());

            let result = DidJwk::create(None);
            assert!(result.is_ok());

            let bearer_did = result.unwrap();
            let public_jwk = bearer_did.document.verification_method[0]
                .public_key_jwk
                .clone();
            assert_eq!(public_jwk.alg, Some("Ed25519".to_string()));
            assert_eq!(public_jwk.kty, "OKP".to_string());
            assert_eq!(public_jwk.crv, "Ed25519".to_string());
        }
    }

    mod resolve {
        use super::*;

        lazy_static! {
            static ref TEST_SUITE: UnitTestSuite = UnitTestSuite::new("did_jwk_resolve");
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
        fn test_invalid_did() {
            TEST_SUITE.include(test_name!());

            let resolution_result = DidJwk::resolve("something invalid");
            assert_eq!(
                resolution_result.resolution_metadata.error,
                Some(ResolutionMetadataError::InvalidDid)
            )
        }

        #[test]
        fn test_create_then_resolve() {
            TEST_SUITE.include(test_name!());

            let result = DidJwk::create(None);
            assert!(result.is_ok());
            let bearer_did = result.unwrap();

            let resolution_result = DidJwk::resolve(&bearer_did.did.uri);
            assert_eq!(resolution_result.document, Some(bearer_did.document));
        }
    }
}
