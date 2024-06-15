#!/usr/bin/env bash

set -o errexit
set -o pipefail
set -o nounset
set -o xtrace

cd "$(dirname "$0")"

TARGET_DIR="./dist"

rm -rf "$TARGET_DIR"
mkdir -p "$TARGET_DIR"

cargo build --target wasm32-unknown-unknown --release 

wasm-bindgen --target web ../target/wasm32-unknown-unknown/release/online.wasm --out-dir "$TARGET_DIR"
