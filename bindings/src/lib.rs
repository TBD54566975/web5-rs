use jwk::{Jwk, JwkError};
use std::sync::Arc;

pub fn hello_world() {
    println!("hello world");
}

// contructor
pub fn new_jwk(alg: String, kty: String) -> Arc<Jwk> {
    Arc::new(Jwk {
        alg,
        kty,
        ..Default::default()
    })
}

uniffi::include_scaffolding!("web5");
