mod stubbed_in;

use std::sync::Arc;

use stubbed_in::{
    Did as DidData, DidDht as DidDhtData, DidJwk as DidJwkData, DidWeb as DidWebData,
    Document as DocumentData, DocumentMetadata as DocumentMetadataData, Dsa,
    Ed25519Generator as Ed25519GeneratorInner, Ed25519Signer, Ed25519Verifier, InMemoryKeyManager,
    Jwk as JwkData, ResolutionMetadata as ResolutionMetadataData, ResolutionMetadataError,
    ResolutionResult as ResolutionResultData, Service as ServiceData, Signer,
    VerifiableCredential as VerifiableCredentialData, VerificationMethod as VerificationMethodData,
    Verifier,
};
use web5::credentials::presentation_definition::{
    Constraints as ConstraintsData, Field as FieldData, Filter as FilterData,
    InputDescriptor as InputDescriptorData, Optionality,
    PresentationDefinition as PresentationDefinitionData,
};

// 🚧 Custom DSL's STATIC METHOD's not supported with UDL
// 🚧 *Data struct are used for UDL dictionary's
// 🚧 UDL doesn't support multi-types, for VC's `issuer` member
//
// 🚧 try to prefer defaults over `unimplemented!()`
// 🚧 Using PresentationDefinition from existing source code
//      🚧 dictionary's don't support nested selfs and Filter has one

pub struct Ed25519Generator {}

impl Ed25519Generator {
    pub fn generate(&self) -> JwkData {
        Ed25519GeneratorInner::generate()
    }
}

pub struct Did {}

impl Did {
    pub fn parse(&self, uri: &str) -> DidData {
        DidData::new(uri)
    }
}

pub struct ResolutionResult {}

impl ResolutionResult {
    pub fn resolve(&self, uri: &str) -> ResolutionResultData {
        ResolutionResultData::resolve(uri)
    }
}

pub struct DidJwk {}

impl DidJwk {
    pub fn from_public_key(&self, public_key: JwkData) -> DidJwkData {
        DidJwkData::from_public_key(public_key)
    }

    pub fn from_uri(&self, uri: &str) -> DidJwkData {
        DidJwkData::from_uri(uri)
    }

    pub fn resolve(&self, uri: &str) -> ResolutionResultData {
        DidJwkData::resolve(uri)
    }
}

pub struct DidWeb {}

impl DidWeb {
    pub fn from_uri(&self, uri: &str) -> DidWebData {
        DidWebData::from_uri(uri)
    }

    pub fn resolve(&self, uri: &str) -> ResolutionResultData {
        DidWebData::resolve(uri)
    }
}

pub struct DidDht {
    inner: DidDhtData,
}

impl DidDht {
    pub fn from_identity_key(identity_key: JwkData) -> Self {
        Self {
            inner: DidDhtData::from_identity_key(identity_key),
        }
    }

    pub fn from_uri(&self, uri: &str) -> DidDhtData {
        DidDhtData::from_uri(uri)
    }

    pub fn new(did_dht_data: DidDhtData) -> Self {
        Self {
            inner: did_dht_data,
        }
    }

    pub fn publish(&self, signer: Arc<dyn Signer>) {
        self.inner.publish(signer)
    }

    pub fn deactivate(&self, signer: Arc<dyn Signer>) {
        self.inner.deactivate(signer)
    }

    pub fn resolve(&self, uri: &str) -> ResolutionResultData {
        DidDhtData::resolve(uri)
    }
}

pub struct VerifiableCredential {
    verifiable_credential_data: VerifiableCredentialData,
}

impl VerifiableCredential {
    pub fn new(verifiable_credential_data: VerifiableCredentialData) -> Self {
        Self {
            verifiable_credential_data,
        }
    }

    pub fn sign(&self, signer: Arc<dyn Signer>) -> String {
        self.verifiable_credential_data.sign(signer)
    }

    pub fn verify(&self, vcjwt: String) -> VerifiableCredentialData {
        VerifiableCredentialData::verify(vcjwt)
    }

    pub fn verify_with_verifier(
        &self,
        vcjwt: String,
        verifier: Arc<dyn Verifier>,
    ) -> VerifiableCredentialData {
        VerifiableCredentialData::verify_with_verifier(vcjwt, verifier)
    }
}

pub struct PresentationDefinition {
    inner: PresentationDefinitionData,
}

impl PresentationDefinition {
    pub fn new(data: PresentationDefinitionData) -> Self {
        Self { inner: data }
    }

    pub fn select_credentials(&self, vc_jwts: Vec<String>) -> Vec<String> {
        self.inner.select_credentials(&vc_jwts).unwrap()
    }
}

pub fn hello_world() {
    println!("Hello web5 :)")
}

uniffi::include_scaffolding!("web5");
