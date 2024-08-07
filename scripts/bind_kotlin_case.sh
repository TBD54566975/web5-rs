#!/bin/bash
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
export TARGET_ARCH