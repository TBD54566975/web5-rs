use std::{collections::HashMap, sync::RwLock};

use super::{
    dsa::ed25519::{Ed25519Generator, Ed25519Signer},
    jwk::Jwk,
};

pub struct InMemoryKeyManager {
    map: RwLock<HashMap<String, Jwk>>,
}

impl InMemoryKeyManager {
    pub fn new() -> Self {
        Self {
            map: RwLock::new(HashMap::new()),
        }
    }

    pub fn generate_key_material(&self) -> Jwk {
        let private_jwk = Ed25519Generator::generate();
        self.import_key(private_jwk)
    }

    pub fn get_signer(&self, public_jwk: Jwk) -> Ed25519Signer {
        let map_lock = self.map.read().unwrap();
        let private_jwk = map_lock.get(&public_jwk.compute_thumbprint()).unwrap();
        Ed25519Signer::new(private_jwk.clone())
    }

    pub fn import_key(&self, private_jwk: Jwk) -> Jwk {
        let mut public_jwk = private_jwk.clone();
        public_jwk.d = None;

        let mut map_lock = self.map.write().unwrap();
        map_lock.insert(public_jwk.compute_thumbprint(), private_jwk);
        public_jwk
    }
}
