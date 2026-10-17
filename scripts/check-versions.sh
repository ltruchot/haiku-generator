#!/usr/bin/env bash
# A version written twice must be the same twice. mise.toml is the reference:
#   - the wasm-bindgen CLI and the wasm-bindgen crate (their output formats
#     are tied: a mismatch fails at build time with an obscure schema error);
#   - Rust and `rust-version` of Cargo.toml.
set -euo pipefail
cd "$(dirname "$0")/.."

pin() { sed -nE "s|^$1 = .*\"([0-9][0-9.]*)\".*|\1|p" "$2" | head -n 1; }

cli=$(pin '"github:wasm-bindgen/wasm-bindgen"' mise.toml)
crate=$(sed -nE 's|^wasm-bindgen = "=([0-9.]+)"|\1|p' Cargo.toml)
rust=$(sed -nE 's|^rust = \{ version = "([0-9]+\.[0-9]+)\.[0-9]+".*|\1|p' mise.toml)
msrv=$(pin rust-version Cargo.toml)

status=0
if [ -z "$cli" ] || [ "$cli" != "$crate" ]; then
  echo "wasm-bindgen: CLI '$cli' (mise.toml) != crate '$crate' (Cargo.toml, must be pinned as \"=x.y.z\")" >&2
  status=1
fi
if [ -z "$rust" ] || [ "$rust" != "$msrv" ]; then
  echo "Rust: '$rust' (mise.toml) != rust-version '$msrv' (Cargo.toml)" >&2
  status=1
fi
exit $status
