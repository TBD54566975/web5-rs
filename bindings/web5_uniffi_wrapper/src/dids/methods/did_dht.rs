use crate::{
    crypto::key_manager::{KeyManager, ToInnerKeyManager},
    dids::{bearer_did::BearerDid, resolution::resolution_result::ResolutionResult},
    errors::Result,
    get_rt,
};
use std::sync::Arc;
use web5::dids::{
    data_model::{service::Service, verification_method::VerificationMethod},
    methods::did_dht::{DidDht as InnerDidDht, DidDhtCreateOptions as InnerDidDhtCreateOptions},
};

pub fn did_dht_resolve(uri: &str, gateway_url: Option<String>) -> Result<Arc<ResolutionResult>> {
    let rt = get_rt()?;
    let resolution_result = rt.block_on(InnerDidDht::resolve(uri, gateway_url));
    Ok(Arc::new(ResolutionResult(resolution_result)))
}

#[derive(Default)]
pub struct DidDhtCreateOptions {
    pub publish: Option<bool>,
    pub gateway_url: Option<String>,
    pub key_manager: Option<Arc<dyn KeyManager>>,
    pub service: Option<Vec<Service>>,
    pub controller: Option<Vec<String>>,
    pub also_known_as: Option<Vec<String>>,
    pub verification_method: Option<Vec<VerificationMethod>>,
}

pub fn did_dht_create(options: Option<DidDhtCreateOptions>) -> Result<Arc<BearerDid>> {
    let inner_options = options.map(|o| InnerDidDhtCreateOptions {
        publish: o.publish,
        gateway_url: o.gateway_url,
        key_manager: match o.key_manager {
            None => None,
            Some(km) => Some(Arc::new(ToInnerKeyManager(km))),
        },
        service: o.service,
        controller: o.controller,
        also_known_as: o.also_known_as,
        verification_method: o.verification_method,
    });

    let rt = get_rt()?;
    let inner_bearer_did = rt.block_on(InnerDidDht::create(inner_options))?;
    Ok(Arc::new(BearerDid(inner_bearer_did)))
}

pub fn did_dht_publish(bearer_did: Arc<BearerDid>, gateway_url: Option<String>) -> Result<()> {
    let rt = get_rt()?;
    Ok(rt.block_on(InnerDidDht::publish(bearer_did.0.clone(), gateway_url))?)
}
