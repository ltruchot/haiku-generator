#!/usr/bin/env bash
# Builds the crate to WebAssembly and its JS glue into apps/web/src/wasm/
# (gitignored): cargo, then wasm-bindgen, then wasm-opt. The three tools come
# from mise.toml: run this as `mise run wasm`.
set -euo pipefail
cd "$(dirname "$0")/.."

out=apps/web/src/wasm
wasm=target/wasm32-unknown-unknown/release/haiku_generator.wasm

cargo build --release --lib --locked --target wasm32-unknown-unknown
rm -rf "$out"
# --target web: an ES module with an explicit init(), no bundler plugin needed.
wasm-bindgen --target web --out-dir "$out" "$wasm"
wasm-opt -Os "$out/haiku_generator_bg.wasm" -o "$out/haiku_generator_bg.wasm"
