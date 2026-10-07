---
name: changelog-upgrade
description: Upgrade one dependency of this repo by one major version, Rust crate or npm package, with its changelog read first and the goldens proving nothing moved. Use when asked to update, bump or modernize a dependency, a toolchain or an edition, and before running cargo update or vp update.
---

# One dependency, one major, one commit

The page and the generator must not change when a tool does. An upgrade is
safe here because the goldens (`tests/golden/`) and the browser fingerprints
(`packages/e2e`) say so, not because it compiles.

## Method

1. **Gate green first**: `vp run ready`. An upgrade on a red tree proves nothing.
2. **Read the changelog** of every major crossed, at the source (the repo's
   `CHANGELOG.md` or release notes, not a summary). Note: renames, removed
   features, MSRV, what changes on `wasm32-unknown-unknown`.
3. **One hop**: `0.7 → 0.8`, never `0.7 → 0.10`. Edit the manifest, then
   update that package only: `mise exec -- cargo update -p <crate>`
   (`vp update <pkg>` for npm). A bare `cargo update` moves everything.
4. **Run the gate**. Goldens untouched (`git status --short tests`), same
   test counts in debug and release, the wasm module builds.
5. **Commit that hop alone**, the message saying what the changelog required.
6. Next hop.

Never run `UPDATE_GOLDEN=1` to make an upgrade pass: a golden that moves is
the upgrade changing the output. Stop and find out why.

## What the 2020 → 2026 path taught

| Hop                                 | What it took                                                                                                       |
| ----------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| wasm-bindgen 0.2.60 → 0.2.129       | Nothing in the code; forced by Rust (older than 0.2.88 does not compile). CLI and crate pinned to the same version |
| rand 0.7 → 0.8                      | Feature `wasm-bindgen` removed: `getrandom 0.2` with `js`, for wasm32. `gen_range(a, b)` → `gen_range(a..b)`       |
| rand 0.8 → 0.9                      | `thread_rng()` → `rng()`, `gen_range` → `random_range`; `getrandom 0.3` with `wasm_js`                             |
| rand 0.9 → 0.10                     | `Rng` → `RngExt`; `getrandom 0.4`                                                                                  |
| unicode-segmentation 1.6 → 1.13     | Nothing: a test asserts graphemes = chars on every string, so Unicode table changes cannot move a foot count       |
| lazy_static → `std::sync::LazyLock` | `pub static X: LazyLock<T> = LazyLock::new(\|\| …);`, call sites unchanged (Deref)                                 |
| edition 2018 → 2021 → 2024          | `cargo fix --edition` had nothing to do; format in a commit of its own first                                       |
| Parcel 1 → Vite+                    | A rewrite of the build, not a bump: see `characterization-tests`                                                   |

- An old lockfile may not build on the current Rust at all. Run the existing
  tests on an older toolchain until the blocking bump is done:
  `mise exec rust@1.70.0 -- cargo test`.
- npm: a version must be 3 days old (`minimumReleaseAge`). The newest is
  often refused; take the one before. Security: skill `npm-vulnerability-check`.
- Rust: `mise exec -- cargo audit` is part of the gate.
