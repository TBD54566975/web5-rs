use crate::{
    dsa::Ed25519Signer,
    inner::keys::{InMemoryKeyManager as InnerInMemoryKeyManager, Jwk as InnerJwk},
};
use std::sync::{Arc, RwLock};

pub struct Jwk(Arc<RwLock<InnerJwk>>);

// 🚧 lots of unwrap()'s, should use Result instead
impl Jwk {
    pub fn new(
        alg: String,
        kty: String,
        crv: String,
        d: Option<String>,
        x: String,
        y: Option<String>,
    ) -> Self {
        Self {
            0: Arc::new(RwLock::new(InnerJwk {
                alg,
                kty,
                crv,
                d,
                x,
                y,
            })),
        }
    }

    pub fn from_inner(inner_jwk: InnerJwk) -> Self {
        Self {
            0: Arc::new(RwLock::new(inner_jwk)),
        }
    }

    pub fn to_inner(&self) -> InnerJwk {
        let jwk = self.0.read().unwrap();
        InnerJwk {
            alg: jwk.alg.clone(),
            kty: jwk.kty.clone(),
            crv: jwk.crv.clone(),
            d: jwk.d.clone(),
            x: jwk.x.clone(),
            y: jwk.y.clone(),
        }
    }

    pub fn get_alg(&self) -> String {
        self.0.read().unwrap().alg.clone()
    }

    pub fn set_alg(&self, alg: String) {
        let mut jwk = self.0.write().unwrap();
        jwk.alg = alg;
    }

    pub fn get_kty(&self) -> String {
        self.0.read().unwrap().kty.clone()
    }

    pub fn set_kty(&self, kty: String) {
        let mut jwk = self.0.write().unwrap();
        jwk.kty = kty;
    }

    pub fn get_crv(&self) -> String {
        self.0.read().unwrap().crv.clone()
    }

    pub fn set_crv(&self, crv: String) {
        let mut jwk = self.0.write().unwrap();
        jwk.crv = crv;
    }

    pub fn get_d(&self) -> Option<String> {
        self.0.read().unwrap().d.clone()
    }

    pub fn set_d(&self, d: Option<String>) {
        let mut jwk = self.0.write().unwrap();
        jwk.d = d;
    }

    pub fn get_x(&self) -> String {
        self.0.read().unwrap().x.clone()
    }

    pub fn set_x(&self, x: String) {
        let mut jwk = self.0.write().unwrap();
        jwk.x = x;
    }

    pub fn get_y(&self) -> Option<String> {
        self.0.read().unwrap().y.clone()
    }

    pub fn set_y(&self, y: Option<String>) {
        let mut jwk = self.0.write().unwrap();
        jwk.y = y;
    }
}

pub struct InMemoryKeyManager(InnerInMemoryKeyManager);

impl InMemoryKeyManager {
    pub fn new() -> Self {
        Self {
            0: InnerInMemoryKeyManager {},
        }
    }

    pub fn generate_key_material(&self) -> Arc<Jwk> {
        let inner_jwk = self.0.generate_key_material();
        Arc::new(Jwk::from_inner(inner_jwk))
    }

    pub fn get_signer(&self, public_key: Arc<Jwk>) -> Arc<Ed25519Signer> {
        let inner_ed25519_signer = self.0.get_signer(public_key.to_inner());
        Arc::new(Ed25519Signer::from_inner(inner_ed25519_signer))
    }

    pub fn import_key(&self, private_key: Arc<Jwk>) -> Arc<Jwk> {
        let inner_jwk = self.0.import_key(private_key.to_inner());
        Arc::new(Jwk::from_inner(inner_jwk))
    }
}
