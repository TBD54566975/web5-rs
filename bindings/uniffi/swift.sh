#!/bin/bash

set -e

cargo run --bin uniffi-bindgen generate --library ../../target/release/libweb5.dylib --language swift --out-dir target/bindgen-swift

mkdir -p target/xcframework-staging
mv target/bindgen-swift/web5.swift ../swift/Sources/UniFFI
mv target/bindgen-swift/web5FFI.modulemap target/xcframework-staging/module.modulemap
mv target/bindgen-swift/web5FFI.h target/xcframework-staging/

rm -rf ../swift/libweb5-rs.xcframework
xcodebuild -create-xcframework \
  -library ../../target/release/libweb5.dylib \
  -headers target/xcframework-staging \
  -output ../swift/libweb5-rs.xcframework