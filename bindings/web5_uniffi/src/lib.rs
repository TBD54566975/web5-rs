use std::sync::Arc;

use web5_uniffi_wrapper::{
    credentials::{
        presentation_definition::PresentationDefinition,
        verifiable_credential_1_1::{
            data::VerifiableCredential as VerifiableCredentialData, VerifiableCredential,
        },
    },
    crypto::{
        dsa::{
            ed25519::{ed25519_generator_generate, Ed25519Signer, Ed25519Verifier},
            Signer, Verifier,
        },
        in_memory_key_manager::InMemoryKeyManager,
        key_manager::KeyManager,
    },
    dids::{
        bearer_did::{BearerDid, BearerDidData},
        data_model::document::Document,
        did::Did,
        methods::{
            did_dht::{did_dht_resolve, DidDht},
            did_jwk::{did_jwk_resolve, DidJwk},
            did_web::{did_web_resolve, DidWeb},
        },
        resolution::resolution_result::ResolutionResult,
    },
    errors::RustCoreError,
};

use web5::{
    crypto::{dsa::Dsa, jwk::Jwk as JwkData},
    dids::{
        data_model::{
            document::Document as DocumentData, service::Service as ServiceData,
            verification_method::VerificationMethod as VerificationMethodData,
        },
        did::Did as DidData,
        methods::{
            did_dht::DidDht as DidDhtData, did_jwk::DidJwk as DidJwkData,
            did_web::DidWeb as DidWebData,
        },
        resolution::{
            document_metadata::DocumentMetadata as DocumentMetadataData,
            resolution_metadata::{
                ResolutionMetadata as ResolutionMetadataData, ResolutionMetadataError,
            },
            resolution_result::ResolutionResult as ResolutionResultData,
        },
    },
};

#[derive(thiserror::Error, Debug)]
pub enum ExampleError {
    #[error("case a error")]
    CaseA,
}

#[derive(thiserror::Error, Debug)]
pub enum ExampleError2 {
    #[error("example error 2 with {a} and {b}")]
    CaseB { a: u64, b: u64 },
}

#[derive(Debug, thiserror::Error)]
pub enum RustCoreErrorV2 {
    #[error("{msg}")]
    Error {
        r#type: String,
        variant: String,
        msg: String,
    },
}

pub trait ExampleForeignTrait: Send + Sync {
    fn hello_world(&self) -> Result<(), ExampleError>;
    fn hello_world_2(&self) -> Result<(), ExampleError2>;
    fn hello_world_3(&self) -> Result<(), RustCoreErrorV2>;
}

pub fn example_foreign_trait(ex: Arc<dyn ExampleForeignTrait>) -> Result<(), RustCoreError> {
    ex.hello_world().unwrap();
    ex.hello_world_2().unwrap();
    ex.hello_world_3().unwrap();

    Err(RustCoreError::Error {
        r#type: "test-type".to_string(),
        variant: "test-varient".to_string(),
        msg: "test-msg".to_string(),
    })
}

uniffi::include_scaffolding!("web5");
