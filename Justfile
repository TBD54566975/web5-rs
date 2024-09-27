set shell := ["bash", "-uc"]

# Setup local development environment
setup:
  #!/bin/bash
  source bin/activate-hermit
  if [ ! -d ".git/modules/web5-spec" ]; then
    git submodule update --init --recursive
  fi
  if [[ "$(cargo 2>&1)" == *"rustup could not choose a version of cargo to run"* ]]; then
    rustup default 1.76.0 # TODO undo this
    rustup target add aarch64-apple-darwin
  fi
  if ! command -v wasm-pack >/dev/null || [[ "$(wasm-pack --version)" != "wasm-pack 0.13.0" ]]; then
    cargo install wasm-pack --version 0.13.0
  fi

docs: setup
  cargo doc --open --no-deps

build: setup
  cargo build --workspace

test: setup
  cargo test --workspace

lint: setup
  cargo clippy --workspace --exclude web5_c
  cargo clippy --package web5_c -- -A clippy::not_unsafe_ptr_arg_deref
  cargo fmt

bind: setup
  just bind-kotlin

bindc: setup
  cargo build --release --package web5_c

bind-kotlin: setup
  mkdir -p bound/kt/src/main/resources
  cargo build --release --package web5_uniffi --target aarch64-apple-darwin
  cp target/aarch64-apple-darwin/release/libweb5_uniffi.dylib \
    bound/kt/src/main/resources/libweb5_uniffi_aarch64_apple_darwin.dylib
  cargo run --package web5_uniffi \
    --bin uniffi-bindgen \
    generate --library bound/kt/src/main/resources/libweb5_uniffi_aarch64_apple_darwin.dylib \
    --language kotlin \
    --out-dir target/bindgen-kotlin
  sed -i '' 's/findLibraryName(componentName)/detectSystemTarget()/' target/bindgen-kotlin/web5/sdk/rust/web5.kt
  cp target/bindgen-kotlin/web5/sdk/rust/web5.kt bound/kt/src/main/kotlin/web5/sdk/rust/UniFFI.kt

test-bound: setup
  just test-kotlin

test-kotlin: setup
  cd bound/kt && mvn clean test

wasm: setup
  (cd bindings/web5_wasm; wasm-pack build --target nodejs --out-dir ../../bound/typescript/pkg)
