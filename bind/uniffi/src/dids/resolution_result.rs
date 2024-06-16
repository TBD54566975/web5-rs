use web5::apid::dids::resolution_result::ResolutionResult as InnerResolutionResult;

pub struct RcbResolutionResult(InnerResolutionResult);

impl RcbResolutionResult {
    pub fn new(uri: &str) -> Self {
        Self(InnerResolutionResult::new(uri))
    }

    pub fn get_data(&self) -> InnerResolutionResult {
        self.0.clone()
    }
}
