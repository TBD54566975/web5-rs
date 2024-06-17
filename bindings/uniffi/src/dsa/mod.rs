pub mod ed25519;

use crate::errors::RcbResult;
use std::sync::Arc;
use web5::apid::dsa::{Signer, Verifier};

pub trait RcbSigner: Send + Sync {
    fn sign(&self, payload: &[u8]) -> RcbResult<Vec<u8>>;
    fn to_signer(&self) -> Arc<dyn Signer>;
}

pub trait RcbVerifier: Send + Sync {
    fn verify(&self, payload: &[u8], signature: &[u8]) -> RcbResult<bool>;
    fn to_verifier(&self) -> Arc<dyn Verifier>;
}
