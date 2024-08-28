// use web5::dids::methods::did_jwk::DidJwk;

use web5::json::JsonValue;

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn create_did_jwk() -> String {
    // DidJwk::create(None).unwrap().did.uri
    format!("{:?}", JsonValue::Null)
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
