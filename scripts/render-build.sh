#!/usr/bin/env bash
# Render build (render.yaml): the wasm module, then the page with its deploy
# base, nested under site/ so the published root mirrors the public URL.
# Runs locally too: `bash scripts/render-build.sh`, then serve site/.
set -euo pipefail
cd "$(dirname "$0")/.."

APP="loic-truchot/projects/haiku-generator"
# The mise that wrote mise.lock.
MISE_VERSION="v2026.9.9"

# What the build machine offers, in the deploy log.
echo "PATH=$PATH"
for tool in mise rustup cargo node vp; do
  printf '%s: %s\n' "$tool" "$(command -v "$tool" || echo absent)"
done

# Rust, wasm-bindgen and wasm-opt at the versions of mise.toml and mise.lock.
if ! command -v mise >/dev/null; then
  curl -fsSL https://mise.run | MISE_VERSION="$MISE_VERSION" sh
  export PATH="$HOME/.local/bin:$PATH"
fi
# cargo-audit is for the gate, not for the build: minutes of compilation saved.
export MISE_DISABLE_TOOLS="cargo:cargo-audit"
export MISE_YES=1
# Render's image ships a Rust whose RUSTUP_HOME and CARGO_HOME (/usr/local) are
# read-only: rustup-init cannot write its settings there. Our own homes instead.
export RUSTUP_HOME="$HOME/.rustup" CARGO_HOME="$HOME/.cargo"
mise trust
mise install --locked rust "github:wasm-bindgen/wasm-bindgen" "aqua:WebAssembly/binaryen"
mise run wasm

# Vite+ owns Node and pnpm (devEngines).
if ! command -v vp >/dev/null; then
  curl -fsSL https://vite.plus | bash
  export PATH="$HOME/.vite-plus/bin:$HOME/.local/share/vite-plus/bin:$PATH"
fi
vp install --frozen-lockfile

rm -rf site
mkdir -p "site/$(dirname "$APP")"
BASE="/$APP/" vp -C apps/web build
cp -r apps/web/dist "site/$APP"
