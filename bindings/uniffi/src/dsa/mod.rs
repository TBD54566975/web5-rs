pub mod ed25519;

use crate::errors::Result;
use std::sync::Arc;
use web5::apid::dsa::{Signer as InnerSigner, Verifier as InnerVerifier};

pub trait Signer: Send + Sync {
    fn sign(&self, payload: &[u8]) -> Result<Vec<u8>>;
    fn to_inner(&self) -> Arc<dyn InnerSigner>;
}

pub struct OuterSigner(pub Arc<dyn InnerSigner>);

impl Signer for OuterSigner {
    fn sign(&self, payload: &[u8]) -> Result<Vec<u8>> {
        self.0.sign(payload).map_err(|e| Arc::new(e.into()))
    }

    fn to_inner(&self) -> Arc<dyn InnerSigner> {
        self.0.clone()
    }
}

pub trait Verifier: Send + Sync {
    fn verify(&self, payload: &[u8], signature: &[u8]) -> Result<bool>;
    fn to_inner(&self) -> Arc<dyn InnerVerifier>;
}
