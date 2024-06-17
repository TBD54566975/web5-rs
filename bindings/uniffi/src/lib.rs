mod credentials;
mod dids;
mod dsa;

mod errors;
mod in_memory_key_manager;

use crate::{
    credentials::{
        presentation_definition::RcbPresentationDefinition,
        verifiable_credential_11::RcbVerifiableCredential,
    },
    dids::{
        did::RcbDid,
        methods::{
            did_dht::{rcb_did_dht_resolve, RcbDidDht},
            did_jwk::{rcb_did_jwk_resolve, RcbDidJwk},
            did_web::{rcb_did_web_resolve, RcbDidWeb},
        },
        resolution_result::RcbResolutionResult,
    },
    dsa::{
        ed25519::{rcb_ed25519_generator_generate, RcbEd25519Signer, RcbEd25519Verifier},
        RcbSigner, RcbVerifier,
    },
    errors::RcbError,
    in_memory_key_manager::RcbInMemoryKeyManager,
};

use web5::apid::{
    credentials::{
        presentation_definition::{
            Constraints as RcbConstraintsData, Field as RcbFieldData, Filter as RcbFilterData,
            InputDescriptor as RcbInputDescriptorData, Optionality as RcbOptionality,
            PresentationDefinition as RcbPresentationDefinitionData,
        },
        verifiable_credential_11::{
            CredentialSubject as RcbCredentialSubjectData,
            VerifiableCredential as RcbVerifiableCredentialData,
        },
    },
    dids::{
        did::Did as RcbDidData,
        document::{
            Document as RcbDocumentData, Service as RcbServiceData,
            VerificationMethod as RcbVerificationMethodData,
        },
        methods::{
            did_dht::DidDht as RcbDidDhtData, did_jwk::DidJwk as RcbDidJwkData,
            did_web::DidWeb as RcbDidWebData,
        },
        resolution_result::{
            DocumentMetadata as RcbDocumentMetadataData,
            ResolutionMetadata as RcbResolutionMetadataData,
            ResolutionMetadataError as RcbResolutionMetadataError,
            ResolutionResult as RcbResolutionResultData,
        },
    },
    dsa::Dsa as RcbDsa,
    jwk::Jwk as RcbJwkData,
};

uniffi::include_scaffolding!("web5");
