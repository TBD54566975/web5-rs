#!/bin/bash

set -e

(cd ../../; cargo build --release)
cargo run --bin uniffi-bindgen generate --library ../../target/release/libjwk.dylib --config uniffi.toml --language swift --out-dir target/bindgen-swift

mkdir -p target/xcframework-staging
mv target/bindgen-swift/jwk.swift ../../web5-swift/Sources/UniFFI
mv target/bindgen-swift/jwkFFI.modulemap target/xcframework-staging/module.modulemap
mv target/bindgen-swift/jwkFFI.h target/xcframework-staging/

rm -rf ../../web5-swift/libweb5-rs.xcframework
xcodebuild -create-xcframework \
  -library ../../target/release/libjwk.dylib \
  -headers target/xcframework-staging \
  -output ../../web5-swift/libweb5-rs.xcframework