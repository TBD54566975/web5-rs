use web5::apid::dids::resolution_result::ResolutionResult as InnerResolutionResult;

pub struct ResolutionResult(pub InnerResolutionResult);

impl ResolutionResult {
    pub fn new(uri: &str) -> Self {
        Self(InnerResolutionResult::new(uri))
    }

    pub fn get_data(&self) -> InnerResolutionResult {
        self.0.clone()
    }
}
