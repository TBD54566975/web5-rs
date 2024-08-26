use web5::dids::resolution::resolution_result::{
    ResolutionResult as InnerResolutionResult, ResolutionResultResolveOptions,
};

pub struct ResolutionResult(pub InnerResolutionResult);

impl ResolutionResult {
    pub fn resolve(uri: &str, options: Option<ResolutionResultResolveOptions>) -> Self {
        Self(InnerResolutionResult::resolve(uri, options))
    }

    pub fn get_data(&self) -> InnerResolutionResult {
        self.0.clone()
    }
}
