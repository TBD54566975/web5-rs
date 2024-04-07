use crate::{document::Document, identifier::Identifier};
use crypto::key_manager::KeyManager;
use std::sync::Arc;

#[derive(Debug)]
pub struct BearerDid<T: KeyManager> {
    pub identifier: Identifier,
    pub key_manager: Arc<T>,
    pub document: Document,
}
