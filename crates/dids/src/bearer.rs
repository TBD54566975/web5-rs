use crate::{document::DidDocument, identifier::DidIdentifier};
use crypto::key_manager::KeyManager;
use std::sync::Arc;

#[derive(Debug)]
pub struct BearerDid<T: KeyManager> {
    pub identifier: DidIdentifier,
    pub key_manager: Arc<T>,
    pub document: DidDocument,
}
