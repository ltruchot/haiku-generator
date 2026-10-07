---
name: mise-toolchain
description: Add, pin or bump a tool installed by mise in this repo - Rust, the wasm32 target, wasm-bindgen CLI, binaryen, cargo-audit - and keep mise.lock, Cargo.toml and CI in step. Use before editing mise.toml or mise.lock, when a tool is missing from the PATH, or when a build differs between this machine, CI and Render.
paths: "mise.toml, mise.lock, scripts/check-versions.sh"
---

# Toolchain through mise

`mise.toml` is the only place a toolchain version is written; `mise.lock`
(`[settings] lockfile = true`) is committed with it. Nothing Rust is on the
PATH outside mise: run `mise exec -- cargo …` or a task (`mise run wasm`,
`mise run rust-check`).

## What is pinned, and how

| Tool             | Entry                                                                                                        | Notes                                                      |
| ---------------- | ------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------- |
| Rust             | `rust = { version, profile = "minimal", targets = "wasm32-unknown-unknown", components = "rustfmt,clippy" }` | mise installs rustup itself, then the toolchain through it |
| wasm-bindgen CLI | `"github:wasm-bindgen/wasm-bindgen"`                                                                         | Release binary; not in the mise or aqua registry           |
| wasm-opt         | `"aqua:WebAssembly/binaryen"`                                                                                | Version is the bare number (`133`), not `version_133`      |
| cargo-audit      | `"cargo:cargo-audit"`                                                                                        | Gate only; the Render build disables it                    |

- No `rust-toolchain.toml`: mise exports `RUSTUP_TOOLCHAIN`, which outranks
  it, and it could not pin the wasm tools.
- Node and pnpm are **not** here: Vite+ owns them. Two managers would fight
  over the shims.
- A version written twice must match: the wasm-bindgen CLI and crate, Rust
  and `rust-version`. `bash scripts/check-versions.sh` (first step of
  `mise run rust-check`).

## Bump a tool

1. `mise ls-remote <tool> | tail` for what exists.
2. Edit `mise.toml`, then `mise install <tool>`: it rewrites `mise.lock`.
3. For Rust or wasm-bindgen, edit `Cargo.toml` to match.
4. `vp run ready`, commit `mise.toml` and `mise.lock` together.

An older toolchain for one command, without touching the files:
`mise exec rust@1.70.0 -- cargo test`.

## Traps

| Symptom                                                            | Cause                                                                 | Fix                                                                   |
| ------------------------------------------------------------------ | --------------------------------------------------------------------- | --------------------------------------------------------------------- |
| `mise WARN 1 newer rust release hidden by minimum_release_age`     | mise holds back releases younger than its guard                       | Pin a version that is listed; do not lower the guard                  |
| `mise install --locked` fails with `pnpm@… is not in the lockfile` | A tool of the machine's global mise config is not in this repo's lock | Name the tools: `mise install --locked rust "github:…" "aqua:…"`      |
| A fresh machine refuses the config                                 | Untrusted `mise.toml`                                                 | `mise trust` (the build script does it)                               |
| The Render build compiles for minutes                              | It installs cargo-audit                                               | `MISE_DISABLE_TOOLS="cargo:cargo-audit"` in `scripts/render-build.sh` |
