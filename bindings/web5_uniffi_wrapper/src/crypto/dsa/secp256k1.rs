use super::{Signer, Verifier};
use crate::errors::Result;
use web5::crypto::{
    dsa::{
        secp256k1::{
            Secp256k1Generator as InnerSecp256k1Generator, Secp256k1Signer as InnerSecp256k1Signer,
            Secp256k1Verifier as InnerSecp256k1Verifier,
        },
        Signer as InnerSigner, Verifier as InnerVerifier,
    },
    jwk::Jwk,
};

pub fn secp256k1_generator_generate() -> Jwk {
    InnerSecp256k1Generator::generate()
}

pub struct Secp256k1Signer(pub InnerSecp256k1Signer);

impl Secp256k1Signer {
    pub fn new(private_jwk: Jwk) -> Self {
        Self(InnerSecp256k1Signer::new(private_jwk))
    }
}

impl Signer for Secp256k1Signer {
    fn sign(&self, payload: Vec<u8>) -> Result<Vec<u8>> {
        Ok(self.0.sign(&payload)?)
    }
}

pub struct Secp256k1Verifier(pub InnerSecp256k1Verifier);

impl Secp256k1Verifier {
    pub fn new(public_jwk: Jwk) -> Self {
        Self(InnerSecp256k1Verifier::new(public_jwk))
    }
}

impl Verifier for Secp256k1Verifier {
    fn verify(&self, payload: Vec<u8>, signature: Vec<u8>) -> Result<()> {
        Ok(self.0.verify(&payload, &signature)?)
    }
}
