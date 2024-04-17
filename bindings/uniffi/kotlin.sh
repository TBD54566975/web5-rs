#!/bin/bash

set -e

cargo build --release
cargo run --bin uniffi-bindgen generate --library ../../target/release/libweb5.dylib --language kotlin --out-dir target/bindgen-kotlin

cp ../../target/release/libweb5.dylib ../kt/src/main/resources/natives
cp target/bindgen-kotlin/web5/sdk/web5.kt ../kt/src/main/kotlin/web5/sdk