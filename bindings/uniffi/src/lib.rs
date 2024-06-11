mod dids;
mod dsa;
mod inner;
mod keys;

use dids::{
    Did, Document, DocumentMetadata, ResolutionMetadata, ResolutionResult, Service,
    VerificationMethod,
};
use dsa::{Ed25519Signer, Ed25519Verifier};
use inner::{
    dids::ResolutionMetadataError,
    dsa::{Dsa, Signer, Verifier},
};
use keys::{InMemoryKeyManager, Jwk};

// 🚧 static methods
// 🚧 rather than this from_inner() and to_inner() can I implement a standard trait?

pub fn hello_world() {
    println!("Hello web5 :)")
}

uniffi::include_scaffolding!("web5");
