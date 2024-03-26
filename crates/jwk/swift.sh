#!/bin/bash

cargo build --release
cargo run --bin uniffi-bindgen generate --library ../../target/release/libjwk.dylib --language swift --out-dir target/swift-bindings

mkdir target/xcframework-staging
mv target/swift-bindings/jwk.swift ../../web5-swift/Sources/UniFFI
mv target/swift-bindings/jwkFFI.modulemap target/xcframework-staging/module.modulemap
mv target/swift-bindings/jwkFFI.h target/xcframework-staging/

xcodebuild -create-xcframework \
  -library ../../target/release/libjwk.dylib \
  -headers ./target/xcframework-staging \
  -output ./target/libweb5-rs.xcframework