# Crypto

`Crypto` is a library for cryptographic primitives in Rust, essential for Web5.

This crate should _not_ include any binding specific code, and should be usable within any Rust application, conforming
to the Rust API guidelines. All binding related code should be placed in the `bindings` folder at the root of the workspace.

## Build

This crate is set to build with the workspace by default.

To build this crate only, run:

```bash
cargo build -p crypto
```

## Test

This crate is set to test with the workspace by default.

To test this crate only, run:

```bash
cargo test -p crypto
```