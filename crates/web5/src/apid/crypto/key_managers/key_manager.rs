use super::Result;
use crate::apid::{crypto::jwk::Jwk, dsa::Signer};
use std::sync::Arc;

pub trait KeyManager {
    fn get_signer(&self, public_jwk: Jwk) -> Result<Arc<dyn Signer>>;
}
