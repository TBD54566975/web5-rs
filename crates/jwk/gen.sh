#!/bin/bash

cargo build --release
cp ../../target/release/libjwk.dylib ../../examples/kotlin/src/main/resources/natives
cargo run --bin uniffi-bindgen generate --config uniffi.toml --library ../../target/release/libjwk.dylib --language kotlin --out-dir ../../examples/kotlin/src/main/kotlin

# swiftc -module-name jwk -emit-library -o libjwks.dylib -emit-module -emit-module-path ./ -parse-as-library -L ../../target/release/ -ljwk -Xcc -fmodule-map-file=out-swift/jwkFFI.modulemap out-swift/jwk.swift
