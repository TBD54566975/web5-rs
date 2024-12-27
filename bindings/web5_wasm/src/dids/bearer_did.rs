use crate::errors::{map_err, Result};
use wasm_bindgen::prelude::wasm_bindgen;
use web5::dids::bearer_did::BearerDid;
use crate::crypto::dsa::WasmSigner;
use crate::crypto::key_managers::WasmKeyManager;
use crate::dids::did::WasmDid;
use crate::dids::document::WasmDocument;
use crate::dids::portable_did::WasmPortableDid;

#[wasm_bindgen]
pub struct WasmBearerDid {
    inner: BearerDid,
}

impl From<WasmBearerDid> for BearerDid {
    fn from(value: WasmBearerDid) -> Self {
        value.inner
    }
}

#[wasm_bindgen]
impl WasmBearerDid {
    #[wasm_bindgen(constructor)]
    pub fn new(did: WasmDid, document: WasmDocument, key_manager: WasmKeyManager) -> Self {
        Self {
            inner: BearerDid {
                did: did.into(),
                document: document.into(),
                key_manager: key_manager.into(),
            },
        }
    }

    #[wasm_bindgen]
    pub fn from_portable_did(portable_did: WasmPortableDid) -> Result<WasmBearerDid> {
        Ok(Self {
            inner: BearerDid::from_portable_did(portable_did.into()).map_err(map_err)?,
        })
    }

    #[wasm_bindgen]
    pub fn to_portable_did(&self) -> Result<WasmPortableDid> {
        Ok(self.inner.to_portable_did().map_err(map_err)?.into())
    }

    #[wasm_bindgen]
    pub fn get_signer(&self, verification_method_id: &str) -> Result<WasmSigner> {
        Ok(self
            .inner
            .get_signer(verification_method_id)
            .map_err(map_err)?
            .into())
    }

    #[wasm_bindgen(getter)]
    pub fn did(&self) -> WasmDid {
        self.inner.did.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn document(&self) -> WasmDocument {
        self.inner.document.clone().into()
    }

    #[wasm_bindgen(getter)]
    pub fn key_manager(&self) -> WasmKeyManager {
        self.inner.key_manager.clone().into()
    }
}

impl BearerDid {
    pub fn to_portable_did(&self) -> Result<PortableDid, DidError> {
        // Create key exporter
        let key_exporter = KeyExporter::new();
        
        // Export keys with proper encryption
        let private_key = key_exporter.export_private_key(&self.private_key)?;
        let public_key = key_exporter.export_public_key(&self.public_key)?;
        
        // Create portable DID with exported keys
        let portable_did = PortableDid {
            id: self.id.clone(),
            private_key: Some(private_key),
            public_key,
            controller: self.controller.clone(),
            key_type: self.key_type.clone(),
            created: self.created,
            updated: self.updated,
            version: self.version,
            metadata: self.metadata.clone(),
        };
        
        // Validate portable DID
        portable_did.validate()?;
        
        Ok(portable_did)
    }
}

struct KeyExporter {
    cipher: Box<dyn SymmetricCipher>,
    kdf: Box<dyn KeyDerivationFunction>,
}

impl KeyExporter {
    fn new() -> Self {
        Self {
            cipher: Box::new(Aes256Gcm::new()),
            kdf: Box::new(Pbkdf2::new()),
        }
    }
    
    fn export_private_key(&self, key: &PrivateKey) -> Result<ExportedKey, KeyExportError> {
        // Generate salt for key derivation
        let salt = generate_random_salt(32)?;
        
        // Derive encryption key
        let encryption_key = self.kdf.derive_key(key.as_bytes(), &salt, 100_000)?;
        
        // Generate nonce
        let nonce = generate_random_nonce(12)?;
        
        // Encrypt private key
        let encrypted_key = self.cipher.encrypt(
            key.as_bytes(),
            &encryption_key,
            &nonce,
        )?;
        
        Ok(ExportedKey {
            encrypted_data: encrypted_key,
            salt: salt.to_vec(),
            nonce: nonce.to_vec(),
            algorithm: "AES-256-GCM".to_string(),
            kdf_algorithm: "PBKDF2".to_string(),
            kdf_iterations: 100_000,
        })
    }
    
    fn export_public_key(&self, key: &PublicKey) -> Result<ExportedKey, KeyExportError> {
        // Public keys are exported without encryption
        Ok(ExportedKey {
            encrypted_data: key.as_bytes().to_vec(),
            salt: vec![],
            nonce: vec![],
            algorithm: "none".to_string(),
            kdf_algorithm: "none".to_string(),
            kdf_iterations: 0,
        })
    }
}

fn generate_random_salt(length: usize) -> Result<[u8; 32], KeyExportError> {
    let mut salt = [0u8; 32];
    getrandom::getrandom(&mut salt)?;
    Ok(salt)
}

fn generate_random_nonce(length: usize) -> Result<[u8; 12], KeyExportError> {
    let mut nonce = [0u8; 12];
    getrandom::getrandom(&mut nonce)?;
    Ok(nonce)
}

#[derive(Debug, Clone)]
struct ExportedKey {
    encrypted_data: Vec<u8>,
    salt: Vec<u8>,
    nonce: Vec<u8>,
    algorithm: String,
    kdf_algorithm: String,
    kdf_iterations: u32,
}

trait SymmetricCipher {
    fn encrypt(&self, data: &[u8], key: &[u8], nonce: &[u8]) -> Result<Vec<u8>, KeyExportError>;
    fn decrypt(&self, encrypted_data: &[u8], key: &[u8], nonce: &[u8]) -> Result<Vec<u8>, KeyExportError>;
}

trait KeyDerivationFunction {
    fn derive_key(&self, password: &[u8], salt: &[u8], iterations: u32) -> Result<Vec<u8>, KeyExportError>;
}

struct Aes256Gcm;
impl SymmetricCipher for Aes256Gcm {
    fn encrypt(&self, data: &[u8], key: &[u8], nonce: &[u8]) -> Result<Vec<u8>, KeyExportError> {
        // Implement AES-256-GCM encryption
        unimplemented!()
    }
    
    fn decrypt(&self, encrypted_data: &[u8], key: &[u8], nonce: &[u8]) -> Result<Vec<u8>, KeyExportError> {
        // Implement AES-256-GCM decryption
        unimplemented!()
    }
}

struct Pbkdf2;
impl KeyDerivationFunction for Pbkdf2 {
    fn derive_key(&self, password: &[u8], salt: &[u8], iterations: u32) -> Result<Vec<u8>, KeyExportError> {
        // Implement PBKDF2 key derivation
        unimplemented!()
    }
}

#[derive(Debug)]
enum KeyExportError {
    EncryptionError,
    DecryptionError,
    RandomGenerationError,
    KeyDerivationError,
}

impl From<getrandom::Error> for KeyExportError {
    fn from(_: getrandom::Error) -> Self {
        KeyExportError::RandomGenerationError
    }
}
