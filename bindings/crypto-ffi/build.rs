fn main() {
    uniffi::generate_scaffolding("src/crypto_ffi.udl").unwrap();
}
