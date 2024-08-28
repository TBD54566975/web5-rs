use web5_uniffi_wrapper::{
    credentials::{
        presentation_definition::PresentationDefinition,
        verifiable_credential_1_1::{VerifiableCredential, VerifiableCredentialData},
    },
    crypto::{
        dsa::{
            ed25519::{ed25519_generator_generate, Ed25519Signer, Ed25519Verifier},
            Signer, Verifier,
        },
        in_memory_key_manager::InMemoryKeyManager,
        jwk::Jwk,
        key_exporter::KeyExporter,
        key_manager::KeyManager,
    },
    dids::{
        bearer_did::{BearerDid, BearerDidData},
        data_model::document::Document,
        did::Did,
        methods::{
            did_dht::{did_dht_create, did_dht_publish, did_dht_resolve, DidDhtCreateOptions},
            did_jwk::{did_jwk_create, did_jwk_resolve, DidJwkCreateOptions},
            did_web::{did_web_create, did_web_resolve, DidWebCreateOptions},
        },
        portable_did::PortableDid,
        resolution::resolution_result::ResolutionResult,
    },
    errors::Web5Error,
};

use web5::{
    credentials::verifiable_credential_1_1::VerifiableCredentialCreateOptions as VerifiableCredentialCreateOptionsData,
    crypto::{dsa::Dsa, jwk::Jwk as JwkData},
    dids::{
        bearer_did::BearerDidGetSignerOptions as BearerDidGetSignerOptionsData,
        data_model::{
            document::Document as DocumentData, service::Service as ServiceData,
            verification_method::VerificationMethod as VerificationMethodData,
        },
        did::Did as DidData,
        portable_did::PortableDid as PortableDidData,
        resolution::{
            document_metadata::DocumentMetadata as DocumentMetadataData,
            resolution_metadata::{
                ResolutionMetadata as ResolutionMetadataData, ResolutionMetadataError,
            },
            resolution_result::ResolutionResult as ResolutionResultData,
        },
    },
};

uniffi::include_scaffolding!("web5");
