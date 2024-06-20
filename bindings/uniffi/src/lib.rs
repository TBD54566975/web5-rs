mod credentials;
mod crypto;
mod dids;
mod dsa;

mod errors;

use crate::{
    credentials::{
        presentation_definition::PresentationDefinition,
        verifiable_credential_1_1::VerifiableCredential,
    },
    crypto::{in_memory_key_manager::InMemoryKeyManager, key_manager::KeyManager},
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
    dsa::{
        ed25519::{ed25519_generator_generate, Ed25519Signer, Ed25519Verifier},
        Signer, Verifier,
    },
    errors::RustCoreError,
};

use web5::apid::{
    credentials::{
        presentation_definition::{
            Constraints as ConstraintsData, Field as FieldData, Filter as FilterData,
            InputDescriptor as InputDescriptorData, Optionality,
            PresentationDefinition as PresentationDefinitionData,
        },
        verifiable_credential_1_1::{
            CredentialSubject as CredentialSubjectData,
            VerifiableCredential as VerifiableCredentialData,
        },
    },
    crypto::jwk::Jwk as JwkData,
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
    dsa::Dsa,
};

uniffi::include_scaffolding!("web5");
