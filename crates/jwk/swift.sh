#!/bin/bash

set -e

cargo build --release
cargo run --bin uniffi-bindgen generate --library target/release/libjwk.dylib --language swift --out-dir target/out-swift

mkdir -p target/xcframework-staging
mv target/out-swift/jwk.swift ../../web5-swift/Sources/UniFFI
mv target/out-swift/jwkFFI.modulemap target/xcframework-staging/module.modulemap
mv target/out-swift/jwkFFI.h target/xcframework-staging/

rm -rf ../../web5-swift/libweb5-rs.xcframework
xcodebuild -create-xcframework \
  -library target/release/libjwk.dylib \
  -headers target/xcframework-staging \
  -output ../../web5-swift/libweb5-rs.xcframework