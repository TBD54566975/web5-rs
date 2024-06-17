use crate::{dids::resolution_result::RcbResolutionResult, errors::RcbResult};
use std::sync::Arc;
use web5::apid::dids::methods::did_web::DidWeb;

pub struct RcbDidWeb(pub DidWeb);

pub fn rcb_did_web_resolve(uri: &str) -> RcbResult<Arc<RcbResolutionResult>> {
    let resolution_result = DidWeb::resolve(uri).map_err(|e| Arc::new(e.into()))?;
    Ok(Arc::new(RcbResolutionResult(resolution_result)))
}

impl RcbDidWeb {
    pub fn from_uri(uri: &str) -> RcbResult<Self> {
        let did_web = DidWeb::from_uri(uri).map_err(|e| Arc::new(e.into()))?;
        Ok(Self(did_web))
    }

    pub fn get_data(&self) -> DidWeb {
        self.0.clone()
    }
}
