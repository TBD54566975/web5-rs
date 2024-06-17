use crate::errors::RcbResult;
use std::sync::Arc;
use web5::apid::dids::methods::did_web::DidWeb;

pub struct RcbDidWeb(DidWeb);

impl RcbDidWeb {
    pub fn from_uri(uri: &str) -> RcbResult<Self> {
        let did_web = DidWeb::from_uri(uri).map_err(|e| Arc::new(e.into()))?;
        Ok(Self(did_web))
    }

    // 🚧
    // pub fn resolve(_uri: &str) -> ResolutionResult {
    //
    // }

    pub fn get_data(&self) -> DidWeb {
        self.0.clone()
    }
}
