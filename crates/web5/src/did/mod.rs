pub mod method;
mod parsed_did;
pub mod resolver;

use crate::crypto::key_manager::KeyManager;
use std::sync::Arc;

pub struct Did<MethodData> {
    pub uri: String,
    pub key_manager: Arc<dyn KeyManager>,
    pub method_data: MethodData,
}
