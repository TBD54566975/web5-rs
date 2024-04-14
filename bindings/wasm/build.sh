#!/bin/bash

if ! command -v wasm-pack &> /dev/null; then
    cargo install wasm-pack
    if [ $? -ne 0 ]; then
        echo "Failed to install wasm-pack, exiting."
        exit 1
    fi
fi

wasm-pack build --target nodejs --out-dir ../js/pkg
rm ../LICENSE