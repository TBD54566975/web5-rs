#!/bin/bash
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