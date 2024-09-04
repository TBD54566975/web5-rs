use crate::{
    crypto::key_manager::{KeyManager, ToInnerKeyManager},
    dids::{bearer_did::BearerDid, resolution::resolution_result::ResolutionResult},
    errors::Result,
};
use std::sync::Arc;
use web5::dids::{
    data_model::{service::Service, verification_method::VerificationMethod},
    methods::did_dht::{DidDht as InnerDidDht, DidDhtCreateOptions as InnerDidDhtCreateOptions},
};

pub async fn did_dht_resolve(uri: &str, gateway_url: Option<String>) -> Arc<ResolutionResult> {
    let resolution_result = InnerDidDht::resolve(uri, gateway_url).await;
    Arc::new(ResolutionResult(resolution_result))
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

pub async fn did_dht_create(options: Option<DidDhtCreateOptions>) -> Result<Arc<BearerDid>> {
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

    let inner_bearer_did = InnerDidDht::create(inner_options).await?;
    Ok(Arc::new(BearerDid(inner_bearer_did)))
}

pub async fn did_dht_publish(
    bearer_did: Arc<BearerDid>,
    gateway_url: Option<String>,
) -> Result<()> {
    Ok(InnerDidDht::publish(bearer_did.0.clone(), gateway_url).await?)
}
