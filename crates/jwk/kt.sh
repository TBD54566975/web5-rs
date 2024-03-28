#!/bin/bash

set -e

cargo build --release
cargo run --bin uniffi-bindgen generate --library target/release/libjwk.dylib --language kotlin --out-dir target/out-kt

mkdir -p ../../web5-kt/src/main/resources/natives
mv target/release/libjwk.dylib ../../web5-kt/src/main/resources/natives
mv target/out-kt/web5/sdk/jwk.kt ../../web5-kt/src/main/kotlin/web5/sdk