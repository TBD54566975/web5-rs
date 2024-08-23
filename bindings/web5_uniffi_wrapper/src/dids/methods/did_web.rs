use crate::{
    crypto::key_manager::KeyManager, dids::resolution::resolution_result::ResolutionResult,
    errors::Result,
};
use std::sync::Arc;
use web5::{crypto::dsa::Dsa, dids::methods::did_web::DidWeb as InnerDidWeb};

pub async fn did_web_resolve(uri: &str) -> Result<Arc<ResolutionResult>> {
    let resolution_result = InnerDidWeb::resolve(uri);
    Ok(Arc::new(ResolutionResult(resolution_result)))
}

#[derive(Default)]
pub struct DidWebCreateOptions {
    pub key_manager: Option<Arc<dyn KeyManager>>,
    pub dsa: Option<Dsa>,
}
