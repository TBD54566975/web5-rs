use crate::{
    dsa::{RcbSigner, RcbVerifier},
    errors::RcbResult,
};
use std::sync::Arc;
use web5::apid::credentials::verifiable_credential_11::VerifiableCredential;

pub struct RcbVerifiableCredential(pub VerifiableCredential);

impl RcbVerifiableCredential {
    pub fn new(verifiable_credential: VerifiableCredential) -> Self {
        Self(verifiable_credential)
    }

    // 🚧 should be constructor in the APID
    pub fn verify(vcjwt: &str) -> RcbResult<Self> {
        let vc = VerifiableCredential::verify(vcjwt).map_err(|e| Arc::new(e.into()))?;
        Ok(Self(vc))
    }

    // 🚧 should be constructor in the APID
    pub fn verify_with_verifier(vcjwt: &str, verifier: Arc<dyn RcbVerifier>) -> RcbResult<Self> {
        let vc = VerifiableCredential::verify_with_verifier(vcjwt, verifier.to_verifier())
            .map_err(|e| Arc::new(e.into()))?;
        Ok(Self(vc))
    }

    pub fn sign(&self, signer: Arc<dyn RcbSigner>) -> RcbResult<String> {
        self.0
            .sign(signer.to_signer())
            .map_err(|e| Arc::new(e.into()))
    }

    pub fn get_data(&self) -> VerifiableCredential {
        self.0.clone()
    }
}
