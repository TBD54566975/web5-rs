use std::{collections::HashMap, sync::Arc};

#[derive(Default)]
pub struct Jwk {
    pub alg: String,
    pub kty: String,
    pub crv: String,
    pub d: Option<String>,
    pub x: String,
    pub y: Option<String>,
}

pub struct InMemoryKeyManager {}

impl InMemoryKeyManager {
    // 🚧
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate_key_material(&self) -> Jwk {
        println!("generate_key_material");
        Jwk {
            ..Default::default()
        }
    }

    pub fn get_signer(&self, _public_key: Jwk) -> Arc<Ed25519Signer> {
        println!("get_signer");
        Arc::new(Ed25519Signer {})
    }

    pub fn import_key(&self, _private_key: Jwk) -> Jwk {
        unimplemented!()
    }
}

pub enum Dsa {
    Ed25519,
}

pub trait Signer: Send + Sync {
    fn sign(&self, _payload: &[u8]) -> Vec<u8> {
        unimplemented!()
    }
}

pub trait Verifier: Send + Sync {
    fn verify(&self, _message: &[u8], _signature: &[u8]) -> bool {
        unimplemented!()
    }
}

pub struct Ed25519Generator {}

impl Ed25519Generator {
    pub fn generate() -> Jwk {
        unimplemented!()
    }
}

pub struct Ed25519Signer {}

impl Ed25519Signer {
    pub fn new(_private_key: Jwk) -> Self {
        unimplemented!()
    }
}

impl Signer for Ed25519Signer {
    fn sign(&self, _payload: &[u8]) -> Vec<u8> {
        unimplemented!()
    }
}

pub struct Ed25519Verifier {}

impl Ed25519Verifier {
    pub fn new(_public_key: Jwk) -> Self {
        unimplemented!()
    }
}

impl Verifier for Ed25519Verifier {
    fn verify(&self, _message: &[u8], _signature: &[u8]) -> bool {
        unimplemented!()
    }
}

#[derive(Default)]
pub struct Did {
    pub uri: String,
    pub url: String,
    pub method: String,
    pub id: String,
    pub params: Option<HashMap<String, String>>,
    pub path: Option<String>,
    pub query: Option<String>,
    pub fragment: Option<String>,
}

impl Did {
    pub fn new(_uri: &str) -> Self {
        unimplemented!()
    }
}

#[derive(Default)]
pub struct Document {
    pub id: String,
    pub context: Option<Vec<String>>,
    pub controller: Option<Vec<String>>,
    pub also_known_as: Option<Vec<String>>,
    pub verification_method: Vec<VerificationMethod>,
    pub authentication: Option<Vec<String>>,
    pub assertion_method: Option<Vec<String>>,
    pub key_agreement: Option<Vec<String>>,
    pub capability_invocation: Option<Vec<String>>,
    pub capability_delegation: Option<Vec<String>>,
    pub service: Option<Vec<Service>>,
}

pub struct VerificationMethod {
    pub id: String,
    pub r#type: String,
    pub controller: String,
    pub public_key_jwk: Jwk,
}

pub struct Service {
    pub id: String,
    pub r#type: String,
    pub service_endpoint: Vec<String>,
}

#[derive(Debug)]
pub enum ResolutionMetadataError {
    InvalidDid,
    NotFound,
    RepresentationNotSupported,
    MethodNotSupported,
    InvalidDidDocument,
    InvalidDidDocumentLength,
    InternalError,
}

// 🚧
impl Default for ResolutionMetadataError {
    fn default() -> Self {
        ResolutionMetadataError::InvalidDid
    }
}

#[derive(Default)]
pub struct ResolutionMetadata {
    pub error: ResolutionMetadataError,
}

#[derive(Default)]
pub struct DocumentMetadata {
    pub created: Option<String>,
    pub updated: Option<String>,
    pub deactivated: Option<bool>,
    pub next_update: Option<String>,
    pub version_id: Option<String>,
    pub next_version_id: Option<String>,
    pub equivalent_id: Option<Vec<String>>,
    pub canonical_id: Option<String>,
}

#[derive(Default)]
pub struct ResolutionResult {
    pub document: Document,
    pub document_metadata: DocumentMetadata,
    pub resolution_metadata: ResolutionMetadata,
}

impl ResolutionResult {
    pub fn resolve(_uri: &str) -> Self {
        unimplemented!()
    }
}

pub struct DidJwk {
    pub did: Did,
    pub document: Document,
}

impl DidJwk {
    pub fn from_public_key(_public_key: Jwk) -> Self {
        unimplemented!()
    }

    pub fn from_uri(_uri: &str) -> Self {
        unimplemented!()
    }

    pub fn resolve(_uri: &str) -> ResolutionResult {
        unimplemented!()
    }
}

pub struct DidWeb {
    pub did: Did,
    pub document: Document,
}

impl DidWeb {
    pub fn from_uri(_uri: &str) -> Self {
        unimplemented!()
    }

    pub fn resolve(_uri: &str) -> ResolutionResult {
        unimplemented!()
    }
}

#[derive(Default)]
pub struct DidDht {
    pub did: Did,
    pub document: Document,
}

impl DidDht {
    pub fn from_identity_key(_identity_key: Jwk) -> Self {
        println!("from_identity_key");
        Self {
            ..Default::default()
        }
    }

    pub fn from_uri(_uri: &str) -> Self {
        println!("from_uri");
        Self {
            ..Default::default()
        }
    }

    pub fn publish(&self, _signer: Arc<dyn Signer>) {
        println!("publish");
    }

    pub fn deactivate(&self, _signer: Arc<dyn Signer>) {
        println!("deactivate");
    }

    pub fn resolve(_uri: &str) -> ResolutionResult {
        ResolutionResult {
            ..Default::default()
        }
    }
}

pub struct VerifiableCredential {
    pub context: Vec<String>,
    pub id: String,
    pub r#type: Vec<String>,
    pub issuer: String, // 🚧
    pub issuance_date: String,
    pub expiration_date: Option<String>,
    pub credential_subject: String, // 🚧
}

impl VerifiableCredential {
    pub fn sign(&self, _signer: Arc<dyn Signer>) -> String {
        unimplemented!()
    }

    pub fn verify(_vcjwt: String) -> Self {
        unimplemented!()
    }

    pub fn verify_with_verifier(_vcjwt: String, _verifier: Arc<dyn Verifier>) -> Self {
        unimplemented!()
    }
}
