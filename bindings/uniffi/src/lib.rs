// mod dids;
// mod keys;
mod errors;
// mod vc;

// use crate::{
//     dids::{Did, DidDht, DidJwk, DidWeb, ResolutionResult},
//     keys::InMemoryKeyManager,
//     vc::VerifiableCredential,
// };

use std::sync::Arc;

use web5::apid::{
    credentials::verifiable_credential_11::{
        CredentialSubject as CredentialSubjectData,
        VerifiableCredential as VerifiableCredentialData,
    },
    dids::{
        did::Did as DidData,
        document::{
            Document as DocumentData, Service as ServiceData,
            VerificationMethod as VerificationMethodData,
        },
        methods::{
            did_dht::DidDht as DidDhtData, did_jwk::DidJwk as DidJwkData,
            did_web::DidWeb as DidWebData,
        },
        resolution_result::{
            DocumentMetadata as DocumentMetadataData, ResolutionMetadata as ResolutionMetadataData,
            ResolutionMetadataError, ResolutionResult as ResolutionResultData,
        },
    },
    dsa::{
        ed25519::{Ed25519Signer, Ed25519Verifier},
        Dsa, Signer, Verifier,
    },
    jwk::{Jwk as JwkData, JwkError},
};

use errors::{test_jwk_err, test_key_manager_err, UniffiWeb5Error};

#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct UniFfiJwkError {
    message: String,
}

impl UniFfiJwkError {
    fn new(message: String) -> Self {
        Self { message }
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }
}

impl From<JwkError> for UniFfiJwkError {
    fn from(error: JwkError) -> Self {
        match error {
            JwkError::ThumbprintFailed(msg) => {
                UniFfiJwkError::new(format!("Thumbprint failed: {}", msg))
            }
        }
    }
}

pub fn test_err() -> Result<(), Arc<UniFfiJwkError>> {
    Err(Arc::new(
        JwkError::ThumbprintFailed("testing inner string".to_string()).into(),
    ))
}

uniffi::include_scaffolding!("web5");
