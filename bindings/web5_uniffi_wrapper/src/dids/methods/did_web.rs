use crate::{
    crypto::key_manager::{KeyManager, ToInnerKeyManager},
    dids::{bearer_did::BearerDid, resolution::resolution_result::ResolutionResult},
    errors::Result, get_rt,
};
use std::sync::Arc;
use web5::{
    crypto::dsa::Dsa,
    dids::{
        data_model::{service::Service, verification_method::VerificationMethod},
        methods::did_web::{
            DidWeb as InnerDidWeb, DidWebCreateOptions as InnerDidWebCreateOptions,
        },
    },
};

pub fn did_web_resolve(uri: &str) -> Result<Arc<ResolutionResult>> {
    let rt = get_rt()?;
    let resolution_result = rt.block_on(InnerDidWeb::resolve(uri));
    Ok(Arc::new(ResolutionResult(resolution_result)))
}

#[derive(Default)]
pub struct DidWebCreateOptions {
    pub key_manager: Option<Arc<dyn KeyManager>>,
    pub dsa: Option<Dsa>,
    pub service: Option<Vec<Service>>,
    pub controller: Option<Vec<String>>,
    pub also_known_as: Option<Vec<String>>,
    pub verification_method: Option<Vec<VerificationMethod>>,
}

pub fn did_web_create(
    domain: String,
    options: Option<DidWebCreateOptions>,
) -> Result<Arc<BearerDid>> {
    let inner_options = options.map(|o| InnerDidWebCreateOptions {
        dsa: o.dsa,
        key_manager: match o.key_manager {
            None => None,
            Some(km) => Some(Arc::new(ToInnerKeyManager(km))),
        },
        service: o.service,
        controller: o.controller,
        also_known_as: o.also_known_as,
        verification_method: o.verification_method,
    });

    let inner_bearer_did = InnerDidWeb::create(&domain, inner_options)?;
    Ok(Arc::new(BearerDid(inner_bearer_did)))
}
