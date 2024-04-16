#!/bin/bash

set -e

cargo build --release
cargo run --bin uniffi-bindgen generate --library ../../target/release/libweb5.dylib --config uniffi.toml --language kotlin --out-dir target/bindgen-kotlin

mv ../../target/release/libweb5.dylib ../kt/src/main/resources/natives
mv target/bindgen-kotlin/web5/sdk/web5.kt ../kt/src/main/kotlin/web5/sdk