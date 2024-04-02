use crypto::key_manager::KeyManager;

use crate::{
    did::DID,
    document::{Document, VerificationMethodSelector},
    portable_did::PortableDID,
};

#[derive(Debug)]
pub struct BearerDID<T: KeyManager> {
    pub did: DID,
    pub key_manager: T,
    pub document: Document,
}

impl<T: KeyManager> BearerDID<T> {
    pub fn to_portable_did() -> Result<PortableDID, String> {
        // TODO: Implement the logic to convert BearerDID to PortableDID
        unimplemented!()
    }

    pub fn from_portable_did() -> Result<Self, String> {
        // TODO: Implement the logic to convert BearerDID to PortableDID
        unimplemented!()
    }

    pub fn get_signer<'a>(
        &'a self,
        selector: Option<VerificationMethodSelector>,
    ) -> Result<impl Fn(&[u8]) -> Result<Vec<u8>, String> + 'a, String> {
        let vm = self
            .document
            .select_verification_method(selector)
            .map_err(|err| format!("Failed to select verification method: {}", err))?;

        let key_alias = self.key_manager.alias(&vm.public_key_jwk).unwrap();

        let signer = move |payload: &[u8]| {
            self.key_manager
                .sign(&key_alias, payload)
                .map_err(|err| format!("Failed to sign payload: {}", err))
        };

        Ok(signer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto::{key::KeyType, key_manager::local_key_manager::LocalKeyManager};
    use crate::{did::parse, document::VerificationMethod};

    #[test]
    fn test_get_signer() {
        let key_manager = LocalKeyManager::new_in_memory();
        let did = parse("did:example:123").unwrap();
        let key_alias = key_manager.generate_private_key(KeyType::Ed25519).unwrap();

        let public_key = key_manager
            .get_public_key(&key_alias)
            .expect("KeyManagerError occurred")
            .expect("PublicKey not found");

        let method1 = VerificationMethod {
            id: format!("{}#{}", did, key_alias),
            controller: did.uri.to_string(),
            r#type: "JsonWebKey".to_string(),
            public_key_jwk: public_key.clone(),
        };

        let method2 = VerificationMethod {
            id: "did:example:123#key2".to_string(),
            controller: "did:example:123".to_string(),
            r#type: "JsonWebKey".to_string(),
            public_key_jwk: public_key.clone(),
        };

        let document = Document {
            id: "did:example:123".to_string(),
            verification_method: vec![method1.clone(), method2.clone()],
            assertion_method: Some(vec![method1.id.clone()]),
            authentication: Some(vec![method2.id.clone()]),
            ..Default::default()
        };

        let bearer_did = BearerDID {
            did: did.clone(),
            key_manager,
            document,
        };

        let selector = Some(VerificationMethodSelector::ID(format!(
            "{}#{}",
            did.clone(),
            key_alias
        )));
        let signer = bearer_did.get_signer(selector).unwrap();

        let payload = b"test payload";
        let signature = signer(payload).unwrap();

        assert_ne!(signature.len(), 0, "Signature shouldn't be zero length")
    }
}
