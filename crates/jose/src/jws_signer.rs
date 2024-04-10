use anyhow::anyhow;
use josekit::{
    jws::{
        alg::{
            ecdsa::EcdsaJwsAlgorithm as JosekitEcdsaJwsAlgorithm,
            eddsa::EddsaJwsAlgorithm as JosekitEddsaJwsAlgorithm,
        },
        JwsAlgorithm as JosekitJwsAlgorithm, JwsSigner as JosekitJwsSigner,
    },
    JoseError as JosekitJoseError,
};
use std::sync::Arc;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum JwsSignerError {
    #[error("unknown error {0}")]
    UnknownError(String)
}

type Signer = Arc<dyn Fn(&str, &[u8]) -> Result<Vec<u8>, JwsSignerError> + Send + Sync>;

pub struct JwsSigner {
    algorithm: String,
    key_id: String,
    signer: Signer,
}

impl JwsSigner {
    pub fn new(algorithm: String, key_id: String, signer: Signer) -> Self {
        Self {
            algorithm,
            key_id,
            signer,
        }
    }
}

impl JosekitJwsSigner for JwsSigner {
    fn algorithm(&self) -> &dyn JosekitJwsAlgorithm {
        match self.algorithm.as_str() {
            "ES256K" => &JosekitEcdsaJwsAlgorithm::Es256k,
            "EdDSA" => &JosekitEddsaJwsAlgorithm::Eddsa,
            _ => panic!("alg not supported"),
        }
    }

    fn key_id(&self) -> Option<&str> {
        Some(&self.key_id)
    }

    fn signature_len(&self) -> usize {
        64 // secp256k1 and Ed25519 are both always length 64
    }

    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, JosekitJoseError> {
        // todo error mapping
        let signature = (self.signer)(&self.key_id, message).map_err(|_| {
            JosekitJoseError::UnsupportedSignatureAlgorithm(anyhow!("TODO failure case"))
        })?;
        Ok(signature)
    }

    fn box_clone(&self) -> Box<dyn JosekitJwsSigner> {
        Box::new(self.clone())
    }
}

impl std::fmt::Debug for JwsSigner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyManagerJwsSigner")
            .field("alias", &self.key_id)
            .finish()
    }
}

impl Clone for JwsSigner {
    fn clone(&self) -> Self {
        Self {
            algorithm: self.algorithm.clone(),
            key_id: self.key_id.clone(),
            signer: self.signer.clone(),
        }
    }
}

// todo tests for signing
