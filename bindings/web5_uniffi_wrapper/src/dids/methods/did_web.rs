use crate::{dids::resolution::resolution_result::ResolutionResult, errors::Result};
use std::sync::Arc;
use web5::dids::methods::did_web::DidWeb as InnerDidWeb;

pub struct DidWeb(pub InnerDidWeb);

pub async fn did_web_resolve(uri: &str) -> Result<Arc<ResolutionResult>> {
    let resolution_result = InnerDidWeb::resolve(uri);
    Ok(Arc::new(ResolutionResult(resolution_result)))
}

impl DidWeb {
    pub async fn from_uri(uri: &str) -> Result<Self> {
        let did_web = InnerDidWeb::from_uri(uri).await?;
        Ok(Self(did_web))
    }

    pub fn get_data(&self) -> InnerDidWeb {
        self.0.clone()
    }
}
