// 🚧 Custom DSL's STATIC METHOD's not supported with UDL
// 🚧 *Data struct are used for UDL dictionary's
// 🚧 UDL doesn't support multi-types, for VC's `issuer` member
//
// 🚧 try to prefer defaults over `unimplemented!()`
// 🚧 Using PresentationDefinition from existing source code
//      🚧 dictionary's don't support nested selfs and Filter has one
// 🚧 unwrap()'s in various places

mod dids;
mod keys;
mod pex;
mod vc;

use crate::{
    dids::{Did, DidDht, DidJwk, DidWeb, ResolutionResult},
    keys::InMemoryKeyManager,
    pex::PresentationDefinition,
    vc::VerifiableCredential,
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
    dsa::{
        ed25519::{Ed25519Signer, Ed25519Verifier},
        Dsa, Signer, Verifier,
    },
    jwk::Jwk as JwkData,
};

pub fn hello_world() {
    println!("Hello web5 :)")
}

uniffi::include_scaffolding!("web5");
