use crate::{
    crypto::{dsa::Signer, jwk::Jwk},
    errors::Result,
};
use std::sync::Arc;

pub mod in_memory_key_manager;

/// A trait for managing cryptographic keys.
///
/// The `KeyManager` trait provides methods for importing private JWKs and retrieving signers for public JWKs.
pub trait KeyManager: Send + Sync {
    /// Imports a private JWK and returns the corresponding public JWK.
    ///
    /// # Arguments
    /// * `private_jwk` - The private JWK to import.
    ///
    /// # Returns
    /// The public JWK.
    fn import_private_jwk(&self, private_jwk: Jwk) -> Result<Jwk>;

    /// Retrieves a signer for a given public JWK.
    ///
    /// # Arguments
    /// * `public_jwk` - The public JWK for which to retrieve the signer.
    ///
    /// # Returns
    /// A cryptographic signer associated with the public key.
    fn get_signer(&self, public_jwk: Jwk) -> Result<Arc<dyn Signer>>;
}

/// A trait for exporting private key material.
///
/// By default, key export is disabled as exporting private keys can be unsafe. Implementations must override this method if export is needed.
pub trait KeyExporter: Send + Sync {
    /// Exports all private JWKs.
    ///
    /// # Returns
    /// A list of private JWKs.
    fn export_private_jwks(&self) -> Result<Vec<Jwk>> {
        unimplemented!("exporting private key material is unsafe")
    }
}
