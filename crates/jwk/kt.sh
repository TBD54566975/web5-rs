#!/bin/bash

set -e

cargo build --release
cargo run --bin uniffi-bindgen generate --library target/release/libjwk.dylib --language kotlin --out-dir target/out-kt

mkdir -p ../../examples/KotlinExampleApp/src/main/resources/natives
mv target/release/libjwk.dylib ../../examples/KotlinExampleApp/src/main/resources/natives
mv target/out-kt/com/example/jwk.kt ../../examples/KotlinExampleApp/src/main/kotlin/com/example