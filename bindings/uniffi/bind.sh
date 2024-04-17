#!/bin/bash

set -e

cargo build --release

./kotlin.sh
./swift.sh