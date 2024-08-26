use web5::dids::methods::did_jwk::{DidJwk, DidJwkCreateOptions};

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

#[flutter_rust_bridge::frb(sync)]
pub fn create_did_jwk() -> String {
    DidJwk::create(Some(DidJwkCreateOptions::default())).unwrap().did.uri
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
