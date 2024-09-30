use crate::{errors::Result, get_rt};
use web5::dids::resolution::resolution_result::ResolutionResult as InnerResolutionResult;

pub struct ResolutionResult(pub InnerResolutionResult);

impl ResolutionResult {
    pub fn resolve(uri: &str) -> Result<Self> {
        let rt = get_rt()?;
        Ok(Self(rt.block_on(InnerResolutionResult::resolve(uri))))
    }

    pub fn get_data(&self) -> InnerResolutionResult {
        self.0.clone()
    }
}
