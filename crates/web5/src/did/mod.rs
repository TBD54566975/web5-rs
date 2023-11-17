pub mod method;
mod parse;
pub mod resolver;

use crate::crypto::key_manager::KeyManager;
use std::sync::Arc;

pub trait Did {
    fn uri(&self) -> &str;
    fn key_manager(&self) -> &Arc<dyn KeyManager>;
}
