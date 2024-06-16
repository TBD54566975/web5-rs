set shell := ["bash", "-uc"]

# Setup local development environment
setup:
  #!/bin/bash
  git submodule update --init --recursive
  if [[ "$(cargo 2>&1)" == *"rustup could not choose a version of cargo to run"* ]]; then
    rustup default 1.78.0
  fi

build: setup
  cargo build --workspace

test: setup
  cargo test --workspace

lint: setup
  cargo clippy --workspace
  cargo fmt

bind: setup
  just bind-kotlin
  # #234 temporarily commenting out swift because kotlin is the sole focus
  # just bind-swift 

bind-kotlin: setup
  cargo build --release --package web5-uniffi
  cargo run --package web5-uniffi \
    --bin uniffi-bindgen \
    generate --library target/release/libweb5_uniffi.dylib \
    --language kotlin \
    --out-dir target/bindgen-kotlin
  cp target/release/libweb5_uniffi.dylib bound/kt/src/main/resources/natives
  cp target/bindgen-kotlin/web5/sdk/web5.kt bound/kt/src/main/kotlin/web5/sdk
  cd bound/kt && ./fix-load.sh

bind-swift: setup
  cargo build --release --package web5-uniffi
  cargo run --package web5-uniffi \
    --bin uniffi-bindgen \
    generate --library target/release/libweb5_uniffi.dylib \
    --language swift \
    --out-dir target/bindgen-swift
  mkdir -p target/xcframework-staging
  mv target/bindgen-swift/web5.swift bound/swift/Sources/UniFFI
  mv target/bindgen-swift/web5FFI.modulemap target/xcframework-staging/module.modulemap
  mv target/bindgen-swift/web5FFI.h target/xcframework-staging/
  rm -rf bound/swift/libweb5-rs.xcframework
  xcodebuild -create-xcframework \
    -library target/release/libweb5_uniffi.dylib \
    -headers target/xcframework-staging \
    -output bound/swift/libweb5-rs.xcframework

test-bound: setup
  just test-kotlin
  just test-swift

test-kotlin: setup
  cd bound/kt && mvn clean test

test-swift: setup
  cd bound/swift && \
    swift package clean && \
    swift test