set shell := ["bash", "-uc"]

build:
  cargo build --release

test:
  cargo test

lint:
  cargo clippy --workspace
  cargo fmt

bind:
  just bind-ts
  just bind-kotlin
  just bind-swift

bind-ts:
  cargo build --release --package web5-wasm
  if ! command -v wasm-pack &> /dev/null; then cargo install wasm-pack; fi
  wasm-pack build --target nodejs --out-dir ../../binded/ts/pkg bindings/wasm
  rm binded/LICENSE
  rm binded/ts/pkg/.gitignore

bind-kotlin:
  cargo build --release --package web5-uniffi
  cargo run --package web5-uniffi \
    --bin uniffi-bindgen \
    generate --library target/release/libweb5.dylib \
    --language kotlin \
    --out-dir target/bindgen-kotlin
  cp target/release/libweb5.dylib binded/kt/src/main/resources/natives
  cp target/bindgen-kotlin/web5/sdk/web5.kt binded/kt/src/main/kotlin/web5/sdk

bind-swift:
  cargo build --release --package web5-uniffi
  cargo run --package web5-uniffi \
    --bin uniffi-bindgen \
    generate --library target/release/libweb5.dylib \
    --language swift \
    --out-dir target/bindgen-swift
  mkdir -p target/xcframework-staging
  mv target/bindgen-swift/web5.swift binded/swift/Sources/UniFFI
  mv target/bindgen-swift/web5FFI.modulemap target/xcframework-staging/module.modulemap
  mv target/bindgen-swift/web5FFI.h target/xcframework-staging/
  rm -rf binded/swift/libweb5-rs.xcframework
  xcodebuild -create-xcframework \
    -library target/release/libweb5.dylib \
    -headers target/xcframework-staging \
    -output binded/swift/libweb5-rs.xcframework

test-bindings:
  just test-ts
  just test-kotlin
  just test-swift

test-ts:
  cd binded/ts && npm i && npm test

test-kotlin:
  # TODO the `sudo cp` is not what we want
  cd binded/kt && \
    sudo cp src/main/resources/natives/libweb5.dylib ~/Library/Java/JavaVirtualMachines/jdk-17.0.8_7.jdk/Contents/Home/bin && \
    mvn clean test

test-swift:
  cd binded/swift && \
    swift package clean && \
    swift test