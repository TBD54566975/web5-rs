#!/bin/bash
case $(uname -sm) in
  "Darwin arm64")
    echo "aarch64-apple-darwin" ;;
  "Darwin x86_64")
    echo "x86_64-apple-darwin" ;;
  "Linux aarch64")
    echo "aarch64-unknown-linux-gnu" ;;
  "Linux x86_64")
    echo "x86_64-unknown-linux-gnu" ;;
esac