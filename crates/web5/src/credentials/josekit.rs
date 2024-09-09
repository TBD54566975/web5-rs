use std::{fmt::Formatter, sync::Arc};

use crate::crypto::dsa::{Dsa, Signer, Verifier};
use josekit::{
    jws::{
        alg::{ecdsa::EcdsaJwsAlgorithm, eddsa::EddsaJwsAlgorithm},
        JwsAlgorithm, JwsSigner, JwsVerifier,
    },
    JoseError,
};

#[derive(Clone)]
pub struct JoseSigner {
    pub kid: String,
    pub signer: Arc<dyn Signer>,
}

impl JwsSigner for JoseSigner {
    fn algorithm(&self) -> &dyn JwsAlgorithm {
        &EddsaJwsAlgorithm::Eddsa
    }

    fn key_id(&self) -> Option<&str> {
        Some(&self.kid)
    }

    fn signature_len(&self) -> usize {
        64
    }

    fn sign(&self, message: &[u8]) -> core::result::Result<Vec<u8>, JoseError> {
        self.signer
            .sign(message)
            // 🚧 improve error message semantics
            .map_err(|err| JoseError::InvalidSignature(err.into()))
    }

    fn box_clone(&self) -> Box<dyn JwsSigner> {
        Box::new(self.clone())
    }
}

impl core::fmt::Debug for JoseSigner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Signer").field("kid", &self.kid).finish()
    }
}

#[derive(Clone)]
pub struct JoseVerifier {
    pub kid: String,
    pub dsa: Dsa,
    pub verifier: Arc<dyn Verifier>,
}

impl JwsVerifier for JoseVerifier {
    fn algorithm(&self) -> &dyn JwsAlgorithm {
        match self.dsa {
            Dsa::Ed25519 => &EddsaJwsAlgorithm::Eddsa,
            Dsa::Secp256k1 => &EcdsaJwsAlgorithm::Es256k,
        }
    }

    fn key_id(&self) -> Option<&str> {
        Some(self.kid.as_str())
    }

    fn verify(&self, message: &[u8], signature: &[u8]) -> core::result::Result<(), JoseError> {
        self.verifier
            .verify(message, signature)
            .map_err(|e| JoseError::InvalidSignature(e.into()))
    }

    fn box_clone(&self) -> Box<dyn JwsVerifier> {
        Box::new(self.clone())
    }
}

impl core::fmt::Debug for JoseVerifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Verifier").field("kid", &self.kid).finish()
    }
}

#[derive(Clone)]
pub struct JoseVerifierAlwaysTrue {
    pub kid: String,
}
impl JwsVerifier for JoseVerifierAlwaysTrue {
    fn algorithm(&self) -> &dyn JwsAlgorithm {
        &EddsaJwsAlgorithm::Eddsa
    }

    fn key_id(&self) -> Option<&str> {
        Some(self.kid.as_str())
    }

    fn verify(&self, _message: &[u8], _signature: &[u8]) -> core::result::Result<(), JoseError> {
        Ok(())
    }

    fn box_clone(&self) -> Box<dyn JwsVerifier> {
        Box::new(self.clone())
    }
}
impl core::fmt::Debug for JoseVerifierAlwaysTrue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Verifier").field("kid", &"").finish()
    }
}
