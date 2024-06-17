mod credentials;
mod dids;
mod dsa;

mod errors;
mod in_memory_key_manager;

use crate::{
    credentials::{
        presentation_definition::PresentationDefinition,
        verifiable_credential_11::VerifiableCredential,
    },
    dids::{
        did::Did,
        methods::{
            did_dht::{did_dht_resolve, DidDht},
            did_jwk::{did_jwk_resolve, DidJwk},
            did_web::{did_web_resolve, DidWeb},
        },
        resolution_result::ResolutionResult,
    },
    dsa::{
        ed25519::{ed25519_generator_generate, Ed25519Signer, Ed25519Verifier},
        Signer, Verifier,
    },
    errors::Error,
    in_memory_key_manager::InMemoryKeyManager,
};

use web5::apid::{
    credentials::{
        presentation_definition::{
            Constraints as ConstraintsData, Field as FieldData, Filter as FilterData,
            InputDescriptor as InputDescriptorData, Optionality,
            PresentationDefinition as PresentationDefinitionData,
        },
        verifiable_credential_11::{
            CredentialSubject as CredentialSubjectData,
            VerifiableCredential as VerifiableCredentialData,
        },
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
    dsa::Dsa,
    jwk::Jwk as JwkData,
};

uniffi::include_scaffolding!("web5");
