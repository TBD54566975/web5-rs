set shell := ["bash", "-uc"]

# Setup local development environment
setup:
  #!/bin/bash
  source bin/activate-hermit
  git submodule update --init --recursive
  if [[ "$(cargo 2>&1)" == *"rustup could not choose a version of cargo to run"* ]]; then
    rustup default 1.78.0
    case $(uname -sm) in
    "Darwin arm64")
      rustup target add aarch64-apple-darwin ;;
    "Darwin x86_64")
      rustup target add x86_64-apple-darwin ;;
    "Linux aarch64")
      rustup target add aarch64-unknown-linux-gnu ;;
    "Linux x86_64")
      rustup target add x86_64-unknown-linux-gnu ;;
    esac
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

bind-kotlin: setup
  case $(uname -sm) in
  "Darwin arm64")
    TARGET_ARCH="aarch64-apple-darwin" ;;
  "Darwin x86_64")
    TARGET_ARCH="x86_64-apple-darwin" ;;
  "Linux aarch64")
    TARGET_ARCH="aarch64-unknown-linux-gnu" ;;
  "Linux x86_64")
    TARGET_ARCH="x86_64-unknown-linux-gnu" ;;
  esac
  cargo build --release --package web5_uniffi --target $TARGET_ARCH
  cp target/$TARGET_ARCH/release/libweb5_uniffi.* \
  bound/kt/src/main/resources/
  cargo run --package web5_uniffi \
  --bin uniffi-bindgen \
  generate --library bound/kt/src/main/resources/libweb5_uniffi.* \
  --language kotlin \
  --out-dir target/bindgen-kotlin
  cp target/bindgen-kotlin/web5/sdk/rust/web5.kt bound/kt/src/main/kotlin/web5/sdk/rust/UniFFI.kt

test-bound: setup
  just test-kotlin

test-kotlin: setup
  cd bound/kt && mvn clean test

