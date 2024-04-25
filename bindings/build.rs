/// Cargo automatically picks up `build.rs` as a custom build script https://doc.rust-lang.org/cargo/reference/build-scripts.html
/// This build script generates the Rust scaffolded code for UniFFI bindings https://mozilla.github.io/uniffi-rs/tutorial/Rust_scaffolding.html#setup-for-crates-using-udl
///     ex. code like `#[no_mangle]` and `extern "C"` is necessary https://doc.rust-lang.org/nomicon/ffi.html#rust-side
fn main() {
    uniffi::generate_scaffolding("src/web5.udl").unwrap();
}
