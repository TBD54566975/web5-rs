use std::sync::Arc;

use crate::{document::Document, identifier::Identifier};
use anyhow::anyhow;
use crypto::key_manager::KeyManager;
use josekit::{
    jws::{
        alg::{ecdsa::EcdsaJwsAlgorithm, eddsa::EddsaJwsAlgorithm},
        JwsAlgorithm, JwsSigner,
    },
    JoseError,
};

pub struct BearerDid {
    pub identifier: Identifier,
    pub key_manager: Arc<dyn KeyManager>,
    pub document: Document,
}

// todo is this necessary?
// impl std::fmt::Debug for BearerDid {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("BearerDid").finish()
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationMethodType {
    VerificationMethod,
    AssertionMethod,
    Authentication,
    CapabilityDelegation,
    CapabilityInvocation,
}

// Define an enum to encapsulate the selection criteria
#[derive(Debug, Clone, PartialEq)]
pub enum SignerSelector {
    KeyId(String),
    MethodType(VerificationMethodType),
}

// todo more precise errors
#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum BearerDidError {
    #[error("verfication method not found")]
    VerificationMethodNotFound,
}

impl BearerDid {
    // todo support optional selector
    pub fn get_signer(&self, selector: SignerSelector) -> Result<BearerDidSigner, BearerDidError> {
        let key_id = match selector {
            SignerSelector::KeyId(key_id) => key_id,
            SignerSelector::MethodType(method_type) => match method_type {
                // todo a lot of duplication
                VerificationMethodType::AssertionMethod => self
                    .document
                    .assertion_method
                    .as_ref()
                    .ok_or(BearerDidError::VerificationMethodNotFound)?
                    .first()
                    .ok_or(BearerDidError::VerificationMethodNotFound)?
                    .to_string(),
                VerificationMethodType::Authentication => self
                    .document
                    .authentication
                    .as_ref()
                    .ok_or(BearerDidError::VerificationMethodNotFound)?
                    .first()
                    .ok_or(BearerDidError::VerificationMethodNotFound)?
                    .to_string(),
                VerificationMethodType::CapabilityDelegation => self
                    .document
                    .capability_delegation
                    .as_ref()
                    .ok_or(BearerDidError::VerificationMethodNotFound)?
                    .first()
                    .ok_or(BearerDidError::VerificationMethodNotFound)?
                    .to_string(),
                VerificationMethodType::CapabilityInvocation => self
                    .document
                    .capability_invocation
                    .as_ref()
                    .ok_or(BearerDidError::VerificationMethodNotFound)?
                    .first()
                    .ok_or(BearerDidError::VerificationMethodNotFound)?
                    .to_string(),
                VerificationMethodType::VerificationMethod => self
                    .document
                    .verification_method
                    .first()
                    .ok_or(BearerDidError::VerificationMethodNotFound)?
                    .id
                    .clone(),
            },
        };

        let identifier =
            Identifier::parse(&key_id).map_err(|_| BearerDidError::VerificationMethodNotFound)?;
        let key_alias = identifier
            .fragment
            .ok_or(BearerDidError::VerificationMethodNotFound)?;

        let signer = BearerDidSigner::new(self.key_manager.clone(), key_alias)?;

        Ok(signer)
    }
}

pub struct BearerDidSigner {
    key_manager: Arc<dyn KeyManager>,
    alias: String,
}

impl BearerDidSigner {
    pub fn new(key_manager: Arc<dyn KeyManager>, alias: String) -> Result<Self, BearerDidError> {
        // todo
        // let public_key = key_manager.get_public_key(&alias)?;
        // if public_key.is_none() {
        //     return Err(BearerDidError::VerificationMethodNotFound);
        // }

        Ok(Self { key_manager, alias })
    }
}

impl JwsSigner for BearerDidSigner {
    fn algorithm(&self) -> &dyn JwsAlgorithm {
        // todo lots of potential panics here hmmm
        // todo resolve by trying to handle gracefully in the new() method
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

// todo
impl std::fmt::Debug for BearerDidSigner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyManagerJwsSigner")
            .field("alias", &self.alias)
            .finish()
    }
}

impl Clone for BearerDidSigner {
    fn clone(&self) -> Self {
        Self {
            key_manager: self.key_manager.clone(),
            alias: self.alias.clone(),
        }
    }
}

// todo tests for signing
