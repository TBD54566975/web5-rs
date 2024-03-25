#!/bin/bash

cargo build --release
cp ../../target/release/libjwk.dylib ../../examples/kotlin/src/main/resources/natives
cargo run --bin uniffi-bindgen generate --config uniffi.toml --library ../../target/release/libjwk.dylib --language kotlin --out-dir ../../examples/kotlin/src/main/kotlin