use std::sync::Arc;

use crate::inner::{
    dids::{
        Did as InnerDid, DidDht as InnerDidDht, DidJwk as InnerDidJwk, DidWeb as InnerDidWeb,
        ResolutionResult as InnerResolutionResult,
    },
    dsa::Signer,
    keys::Jwk,
};

pub struct Did(InnerDid);

impl Did {
    pub fn new(uri: &str) -> Self {
        Self(InnerDid::new(uri))
    }

    pub fn get_data(&self) -> InnerDid {
        self.0.clone()
    }
}

pub struct ResolutionResult(InnerResolutionResult);

impl ResolutionResult {
    pub fn new(uri: &str) -> Self {
        Self(InnerResolutionResult::resolve(uri))
    }

    pub fn get_data(&self) -> InnerResolutionResult {
        self.0.clone()
    }
}

pub struct DidJwk(InnerDidJwk);

impl DidJwk {
    pub fn from_public_key(public_key: Jwk) -> Self {
        Self(InnerDidJwk::from_public_key(public_key))
    }

    pub fn from_uri(uri: &str) -> Self {
        Self(InnerDidJwk::from_uri(uri))
    }

    // 🚧
    // pub fn resolve(_uri: &str) -> ResolutionResult {
    //
    // }

    pub fn get_data(&self) -> InnerDidJwk {
        self.0.clone()
    }
}

pub struct DidWeb(InnerDidWeb);

impl DidWeb {
    pub fn from_uri(uri: &str) -> Self {
        Self(InnerDidWeb::from_uri(uri))
    }

    // 🚧
    // pub fn resolve(_uri: &str) -> ResolutionResult {
    //
    // }

    pub fn get_data(&self) -> InnerDidWeb {
        self.0.clone()
    }
}

pub struct DidDht(InnerDidDht);

impl DidDht {
    pub fn from_identity_key(public_key: Jwk) -> Self {
        Self(InnerDidDht::from_identity_key(public_key))
    }

    pub fn from_uri(uri: &str) -> Self {
        Self(InnerDidDht::from_uri(uri))
    }

    // 🚧
    // pub fn resolve(_uri: &str) -> ResolutionResult {
    //
    // }

    pub fn publish(&self, signer: Arc<dyn Signer>) {
        self.0.publish(signer)
    }

    pub fn deactivate(&self, signer: Arc<dyn Signer>) {
        self.0.deactivate(signer)
    }

    pub fn get_data(&self) -> InnerDidDht {
        self.0.clone()
    }
}
