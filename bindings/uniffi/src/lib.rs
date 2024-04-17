pub mod jwk;

use ::jwk::JwkError;
use crypto::{ed25519::Ed25199, CryptoError, CurveOperations};
use jwk::Jwk;

pub fn hello_world() -> String {
    "hello world".to_string()
}

pub trait SomeTrait: Send + Sync {
    fn some_func(&self) -> String;
}
pub struct SomeTraitA;
impl SomeTrait for SomeTraitA {
    fn some_func(&self) -> String {
        "a".to_string()
    }
}
pub struct SomeTraitB;
impl SomeTrait for SomeTraitB {
    fn some_func(&self) -> String {
        "b".to_string()
    }
}
impl SomeTraitA {
    fn new() -> Self {
        SomeTraitA {}
    }
}
impl SomeTraitB {
    fn new() -> Self {
        SomeTraitB {}
    }
}

#[uniffi::export]
fn hello_ffi() {
    println!("Hello from Rust!");
}

#[uniffi::export]
trait MyTrait: Send + Sync {
    fn something(&self) -> String;
}

struct Ed25199_2 {}

impl Ed25199_2 {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate(&self) -> Result<jwk::Jwk, CryptoError> {
        let jwk = Ed25199::generate()?;
        Ok(jwk::Jwk::from(jwk::Jwk(jwk)))
    }
}

uniffi::include_scaffolding!("web5");
