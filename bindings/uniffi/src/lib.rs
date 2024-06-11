// 🚧 Custom DSL's STATIC METHOD's not supported with UDL
// 🚧 *Data struct are used for UDL dictionary's
// 🚧 UDL doesn't support multi-types, for VC's `issuer` member
//
// 🚧 try to prefer defaults over `unimplemented!()`
// 🚧 Using PresentationDefinition from existing source code
//      🚧 dictionary's don't support nested selfs and Filter has one

mod dids;
mod inner;
mod keys;
mod vc;

use crate::inner::{
    dids::{
        Did as DidData, DidDht as DidDhtData, DidJwk as DidJwkData, DidWeb as DidWebData,
        Document as DocumentData, DocumentMetadata as DocumentMetadataData,
        ResolutionMetadata as ResolutionMetadataData, ResolutionMetadataError,
        ResolutionResult as ResolutionResultData, Service as ServiceData,
        VerificationMethod as VerificationMethodData,
    },
    dsa::{Dsa, Ed25519Signer, Ed25519Verifier, Signer, Verifier},
    keys::Jwk as JwkData,
    vc::VerifiableCredential as VerifiableCredentialData,
};
use crate::{
    dids::{Did, DidDht, DidJwk, DidWeb, ResolutionResult},
    keys::InMemoryKeyManager,
    vc::VerifiableCredential,
};

pub fn hello_world() {
    println!("Hello web5 :)")
}

uniffi::include_scaffolding!("web5");
