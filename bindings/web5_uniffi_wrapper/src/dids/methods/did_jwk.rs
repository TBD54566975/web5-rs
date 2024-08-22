use crate::{
    crypto::key_manager::{KeyManager, ToInnerKeyManager},
    dids::{bearer_did::BearerDid, resolution::resolution_result::ResolutionResult},
    errors::Result,
};
use std::sync::Arc;
use web5::{
    crypto::dsa::Dsa,
    dids::methods::did_jwk::{DidJwk as InnerDidJwk, DidJwkCreateOptions as InnerCreateOptions},
};

pub struct DidJwk(pub InnerDidJwk);

pub fn did_jwk_resolve(uri: &str) -> Arc<ResolutionResult> {
    let resolution_result = InnerDidJwk::resolve(uri);
    Arc::new(ResolutionResult(resolution_result))
}

#[derive(Default)]
pub struct DidJwkCreateOptions {
    pub key_manager: Option<Arc<dyn KeyManager>>,
    pub dsa: Option<Dsa>,
}

pub fn did_jwk_create(options: Option<DidJwkCreateOptions>) -> Result<Arc<BearerDid>> {
    let inner_options = options.map(|o| InnerCreateOptions {
        dsa: o.dsa,
        key_manager: match o.key_manager {
            None => None,
            Some(km) => Some(Arc::new(ToInnerKeyManager(km))),
        },
    });

    let inner_bearer_did = InnerDidJwk::create(inner_options)?;
    Ok(Arc::new(BearerDid(inner_bearer_did)))
}
