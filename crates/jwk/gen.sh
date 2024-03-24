#!/bin/bash

cargo build --release
cargo run --bin uniffi-bindgen generate --config uniffi.toml --library ../../target/release/libjwk.dylib --language kotlin --out-dir out-kt
cargo run --bin uniffi-bindgen generate --library ../../target/release/libjwk.dylib --language swift --out-dir out-swift