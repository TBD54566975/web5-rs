use web5::dids::resolution::resolution_result::ResolutionResult as InnerResolutionResult;

pub struct ResolutionResult(pub InnerResolutionResult);

impl ResolutionResult {
    pub async fn resolve(uri: &str) -> Self {
        Self(InnerResolutionResult::resolve(uri).await)
    }

    pub fn get_data(&self) -> InnerResolutionResult {
        self.0.clone()
    }
}
