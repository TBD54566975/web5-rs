use std::sync::Arc;

use crate::key_manager::{KeyManager, KeyManagerError};
use anyhow::anyhow;
use josekit::{
    jws::{
        alg::{ecdsa::EcdsaJwsAlgorithm, eddsa::EddsaJwsAlgorithm},
        JwsAlgorithm, JwsSigner,
    },
    JoseError,
};

pub struct KeyManagerJwsSigner {
    key_manager: Arc<dyn KeyManager>,
    alias: String,
}

impl KeyManagerJwsSigner {
    pub fn new(key_manager: Arc<dyn KeyManager>, alias: String) -> Result<Self, KeyManagerError> {
        let public_key = key_manager.get_public_key(&alias)?;
        if public_key.is_none() {
            return Err(KeyManagerError::SigningKeyNotFound);
        }

        Ok(Self { key_manager, alias })
    }
}

impl JwsSigner for KeyManagerJwsSigner {
    fn algorithm(&self) -> &dyn JwsAlgorithm {
        // todo lots of potential panics here hmmm
        // todo resolve by trying to handle gracefully in the new() method

        // let _test = self.key_manager.get_public_key(&self.alias);
        // let _test2 = _test.ok();
        // let _test3 = _test2.

        let public_key = self
            .key_manager
            .get_public_key(&self.alias)
            .unwrap()
            .unwrap();
        let alg = public_key.algorithm().unwrap();

        match alg.as_str() {
            "ES256K" => &EcdsaJwsAlgorithm::Es256k,
            "EdDSA" => &EddsaJwsAlgorithm::Eddsa,
            _ => panic!("alg not supported"),
        }
    }

    fn key_id(&self) -> Option<&str> {
        Some(self.alias.as_str())
    }

    fn signature_len(&self) -> usize {
        64 // secp256k1 and Ed25519 are both always length 64
    }

    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, josekit::JoseError> {
        // todo error mapping
        let signature = self
            .key_manager
            .sign(&self.alias, message)
            .map_err(|_| JoseError::UnsupportedSignatureAlgorithm(anyhow!("TODO failure case")))?;
        Ok(signature)
    }

    fn box_clone(&self) -> Box<dyn JwsSigner> {
        Box::new(self.clone())
    }
}

impl std::fmt::Debug for KeyManagerJwsSigner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyManagerJwsSigner")
            .field("alias", &self.alias)
            .finish()
    }
}

impl Clone for KeyManagerJwsSigner {
    fn clone(&self) -> Self {
        Self {
            key_manager: self.key_manager.clone(),
            alias: self.alias.clone(),
        }
    }
}
