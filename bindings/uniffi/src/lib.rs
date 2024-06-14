mod dids;
mod keys;
mod vc;

use crate::{
    dids::{Did, DidDht, DidJwk, DidWeb, ResolutionResult},
    keys::InMemoryKeyManager,
    vc::VerifiableCredential,
};

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
    jwk::Jwk as JwkData,
};

uniffi::include_scaffolding!("web5");
