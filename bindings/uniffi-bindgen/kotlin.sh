#!/bin/bash

set -e

(cd ../../; cargo build --release)
cargo run --bin uniffi-bindgen generate --library ../../target/release/libjwk.dylib --config uniffi.toml --language kotlin --out-dir target/bindgen-kotlin

mkdir -p ../../web5-kt/src/main/resources/natives
mv ../../target/release/libjwk.dylib ../../web5-kt/src/main/resources/natives
mv target/bindgen-kotlin/web5/sdk/jwk.kt ../../web5-kt/src/main/kotlin/web5/sdk