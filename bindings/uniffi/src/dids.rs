use std::sync::Arc;
use web5::apid::{
    dids::{
        did::Did as InnerDid,
        methods::{
            did_dht::DidDht as InnerDidDht, did_jwk::DidJwk as InnerDidJwk,
            did_web::DidWeb as InnerDidWeb,
        },
        resolution_result::ResolutionResult as InnerResolutionResult,
    },
    dsa::Signer,
    jwk::Jwk,
};

pub struct Did(InnerDid);

impl Did {
    pub fn new(uri: &str) -> Self {
        Self {
            0: InnerDid::new(uri),
        }
    }

    pub fn get_data(&self) -> InnerDid {
        self.0.clone()
    }
}

pub struct ResolutionResult(InnerResolutionResult);

impl ResolutionResult {
    pub fn new(uri: &str) -> Self {
        Self {
            0: InnerResolutionResult::new(uri),
        }
    }

    pub fn get_data(&self) -> InnerResolutionResult {
        self.0.clone()
    }
}

pub struct DidJwk(InnerDidJwk);

impl DidJwk {
    pub fn from_public_jwk(public_key: Jwk) -> Self {
        Self {
            0: InnerDidJwk::from_public_jwk(public_key),
        }
    }

    pub fn from_uri(uri: &str) -> Self {
        Self {
            0: InnerDidJwk::from_uri(uri),
        }
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
        Self {
            0: InnerDidWeb::from_uri(uri),
        }
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
        Self {
            0: InnerDidDht::from_identity_key(public_key),
        }
    }

    pub fn from_uri(uri: &str) -> Self {
        Self {
            0: InnerDidDht::from_uri(uri),
        }
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
