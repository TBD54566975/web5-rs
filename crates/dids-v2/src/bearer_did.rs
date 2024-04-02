use crypto::key_manager::{local_key_manager::LocalKeyManager, KeyManager};

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

pub fn from_portable_did(portable_did: PortableDID) -> Result<BearerDID<LocalKeyManager>, String> {
    let key_manager = LocalKeyManager::new_in_memory();

    key_manager
        .import_private_keys(portable_did.private_keys)
        .map_err(|e| format!("Failed to import private keys: {:?}", e))?;

    let did = DID::parse(&portable_did.uri).map_err(|e| format!("Failed to parse DID: {:?}", e))?;

    let bearer_did = BearerDID {
        did,
        key_manager,
        document: portable_did.document.clone(),
    };

    Ok(bearer_did)
}

impl<T: KeyManager> BearerDID<T> {
    pub fn to_portable_did(&self) -> Result<PortableDID, String> {
        let private_keys = self
            .key_manager
            .export_private_keys()
            .expect("failed to export private keys");

        let portable_did = PortableDID {
            uri: self.did.uri.clone(),
            private_keys: private_keys,
            document: self.document.clone(),
        };

        Ok(portable_did)
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
    use crate::document::VerificationMethod;
    use crypto::key::KeyType;

    #[test]
    fn test_from_portable_did() {
        let document = Document {
            id: "did:example:123".to_string(),
            ..Default::default()
        };
        let did = DID::parse(&document.id).unwrap();
        let key_manager = LocalKeyManager::new_in_memory();
        let _ = key_manager
            .generate_private_key(KeyType::Secp256k1)
            .unwrap();
        let private_keys = key_manager.export_private_keys().unwrap();

        let portable_did = PortableDID {
            uri: did.clone().uri,
            private_keys: private_keys.clone(),
            document: document.clone(),
        };

        let bearer_did = from_portable_did(portable_did).unwrap();

        assert_eq!(bearer_did.did, did);
        assert_eq!(bearer_did.document, document);
        assert_eq!(bearer_did.key_manager.export_private_keys().unwrap(), private_keys);
    }

    #[test]
    fn test_to_portable_did() {
        let document = Document {
            id: "did:example:123".to_string(),
            ..Default::default()
        };
        let did = DID::parse(&document.id).unwrap();
        let key_manager = LocalKeyManager::new_in_memory();
        let _ = key_manager
            .generate_private_key(KeyType::Secp256k1)
            .unwrap();

        let bearer_did = BearerDID {
            document: document.clone(),
            did: did.clone(),
            key_manager,
        };

        let portable_did = bearer_did.to_portable_did().unwrap();

        assert_eq!(portable_did.uri, did.uri.clone());
        assert_eq!(portable_did.private_keys.len(), 1);
        assert_eq!(portable_did.document, document);
    }

    #[test]
    fn test_get_signer() {
        let key_manager = LocalKeyManager::new_in_memory();
        let did = DID::parse("did:example:123").unwrap();
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
