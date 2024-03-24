#!/bin/bash

cargo build --release
cargo run --bin uniffi-bindgen generate --library ../../target/release/libjwk.dylib --language kotlin --out-dir out