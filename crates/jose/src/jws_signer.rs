use std::sync::Arc;

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
use keys::key_manager::KeyManager;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum JwsSignerError {}

pub struct JwsSigner {
    key_manager: Arc<dyn KeyManager>,
    alias: String,
}

impl JwsSigner {
    pub fn new(key_manager: Arc<dyn KeyManager>, alias: String) -> Result<Self, JwsSignerError> {
        // todo
        // let public_key = key_manager.get_public_key(&alias)?;
        // if public_key.is_none() {
        //     return Err(BearerDidError::VerificationMethodNotFound);
        // }

        Ok(Self { key_manager, alias })
    }
}

impl JosekitJwsSigner for JwsSigner {
    fn algorithm(&self) -> &dyn JosekitJwsAlgorithm {
        // todo lots of potential panics here hmmm
        // todo resolve by trying to handle gracefully in the new() method
        let public_key = self
            .key_manager
            .get_public_key(&self.alias)
            .unwrap()
            .unwrap();
        let alg = public_key.algorithm().unwrap();

        match alg.as_str() {
            "ES256K" => &JosekitEcdsaJwsAlgorithm::Es256k,
            "EdDSA" => &JosekitEddsaJwsAlgorithm::Eddsa,
            _ => panic!("alg not supported"),
        }
    }

    fn key_id(&self) -> Option<&str> {
        Some(self.alias.as_str())
    }

    fn signature_len(&self) -> usize {
        64 // secp256k1 and Ed25519 are both always length 64
    }

    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, JosekitJoseError> {
        // todo error mapping
        let signature = self.key_manager.sign(&self.alias, message).map_err(|_| {
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
            .field("alias", &self.alias)
            .finish()
    }
}

impl Clone for JwsSigner {
    fn clone(&self) -> Self {
        Self {
            key_manager: self.key_manager.clone(),
            alias: self.alias.clone(),
        }
    }
}

// todo tests for signing
