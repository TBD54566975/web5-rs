use std::{collections::HashMap, sync::Arc};

use super::{dsa::Signer, keys::Jwk};

#[derive(Default, Clone)]
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
        println!("Invoked Did::new()");
        Self {
            ..Default::default()
        }
    }
}

#[derive(Default, Clone)]
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

#[derive(Clone)]
pub struct VerificationMethod {
    pub id: String,
    pub r#type: String,
    pub controller: String,
    pub public_key_jwk: Jwk,
}

#[derive(Clone)]
pub struct Service {
    pub id: String,
    pub r#type: String,
    pub service_endpoint: Vec<String>,
}

#[derive(Debug, Clone)]
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

#[derive(Default, Clone)]
pub struct ResolutionMetadata {
    pub error: ResolutionMetadataError,
}

#[derive(Default, Clone)]
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

#[derive(Default, Clone)]
pub struct ResolutionResult {
    pub document: Document,
    pub document_metadata: DocumentMetadata,
    pub resolution_metadata: ResolutionMetadata,
}

impl ResolutionResult {
    pub fn resolve(_uri: &str) -> Self {
        println!("Invoked ResolutionResult::resolve()");
        Self {
            ..Default::default()
        }
    }
}

#[derive(Default, Clone)]
pub struct DidJwk {
    pub did: Did,
    pub document: Document,
}

impl DidJwk {
    pub fn from_public_key(_public_key: Jwk) -> Self {
        println!("Invoked DidJwk::from_public_key()");
        Self {
            ..Default::default()
        }
    }

    pub fn from_uri(_uri: &str) -> Self {
        println!("Invoked DidJwk::from_uri()");
        Self {
            ..Default::default()
        }
    }

    // 🚧
    // pub fn resolve(_uri: &str) -> ResolutionResult {
    //     println!("Invoked DidJwk::resolve()");
    //     ResolutionResult {
    //         ..Default::default()
    //     }
    // }
}

#[derive(Default, Clone)]
pub struct DidWeb {
    pub did: Did,
    pub document: Document,
}

impl DidWeb {
    pub fn from_uri(_uri: &str) -> Self {
        println!("Invoked DidWeb::from_uri()");
        Self {
            ..Default::default()
        }
    }

    // 🚧
    // pub fn resolve(_uri: &str) -> ResolutionResult {
    //     println!("Invoked DidWeb::resolve()");
    //     ResolutionResult {
    //         ..Default::default()
    //     }
    // }
}

#[derive(Default, Clone)]
pub struct DidDht {
    pub did: Did,
    pub document: Document,
}

impl DidDht {
    pub fn from_identity_key(_identity_key: Jwk) -> Self {
        println!("Invoked DidDht::from_identity_key()");
        Self {
            ..Default::default()
        }
    }

    pub fn from_uri(_uri: &str) -> Self {
        println!("Invoked DidDht::from_uri()");
        Self {
            ..Default::default()
        }
    }

    pub fn publish(&self, _signer: Arc<dyn Signer>) {
        println!("Invoked DidDht.publish()");
    }

    pub fn deactivate(&self, _signer: Arc<dyn Signer>) {
        println!("Invoked DidDht.deactivate()");
    }

    // 🚧
    // pub fn resolve(_uri: &str) -> ResolutionResult {
    //     println!("Invoked DidDht::resolve()");
    //     ResolutionResult {
    //         ..Default::default()
    //     }
    // }
}
