use crate::inner::dsa::Ed25519Signer;

#[derive(Default, Clone)]
pub struct Jwk {
    pub alg: String,
    pub kty: String,
    pub crv: String,
    pub d: Option<String>,
    pub x: String,
    pub y: Option<String>,
}

pub struct InMemoryKeyManager {}

impl InMemoryKeyManager {
    pub fn generate_key_material(&self) -> Jwk {
        println!("Invoked InMemoryKeyManager.generate_key_material()");
        Jwk {
            ..Default::default()
        }
    }

    pub fn get_signer(&self, _public_key: Jwk) -> Ed25519Signer {
        println!("Invoked InMemoryKeyManager.get_signer()");
        Ed25519Signer {}
    }

    pub fn import_key(&self, _private_key: Jwk) -> Jwk {
        println!("Invoked InMemoryKeyManager.import_key()");
        Jwk {
            ..Default::default()
        }
    }
}
