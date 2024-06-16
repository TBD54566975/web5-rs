use crate::errors::RcbResult;
use std::sync::Arc;
use web5::apid::dids::methods::did_web::DidWeb as InnerDidWeb;

pub struct RcbDidWeb(InnerDidWeb);

impl RcbDidWeb {
    pub fn from_uri(uri: &str) -> RcbResult<Self> {
        let inner = InnerDidWeb::from_uri(uri).map_err(|e| Arc::new(e.into()))?;
        Ok(Self(inner))
    }

    // 🚧
    // pub fn resolve(_uri: &str) -> ResolutionResult {
    //
    // }

    pub fn get_data(&self) -> InnerDidWeb {
        self.0.clone()
    }
}
