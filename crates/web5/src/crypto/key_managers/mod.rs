use crate::{
    crypto::{dsa::Signer, jwk::Jwk},
    errors::Result,
};
use std::sync::Arc;

pub mod in_memory_key_manager;

pub trait KeyManager: Send + Sync {
    fn import_private_jwk(&self, private_jwk: Jwk) -> Result<Jwk>;
    fn get_signer(&self, public_jwk: Jwk) -> Result<Arc<dyn Signer>>;
}
