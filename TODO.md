# TODO

Plan of the modernization: phases 0 to 6 below. Branch `chore/modernize`.

## Where we stopped

- Phase 0 (toolchain, workspace root) and the browser half of phase 1 are
  committed: `vp -C packages/e2e test` pins the 2020 build.
- The old `Cargo.lock` (wasm-bindgen 0.2.60) does not compile on Rust 1.99:
  until the wasm-bindgen bump, run the Rust tests with
  `mise exec rust@1.70.0 -- cargo test`.

## Later

- Phase 1, Rust: characterization tests on the untouched crate (goldens of the
  grammar, set of producible lines per pattern, properties of a haiku), then
  the `Chooser` seam in place of `ThreadRng` and the exhaustive enumeration.
- Phase 2, Rust, one commit each: `cargo fmt`, edition 2021, wasm-bindgen
  0.2.129, rand 0.8, 0.9, 0.10, unicode-segmentation, `LazyLock`, edition 2024,
  `rlib` and one module tree.
- Phase 3, front: `wasm-demo/` becomes `apps/web/` on Vite+, strict TypeScript,
  Tailwind 1.2.0 vendored, wasm built by `scripts/build-wasm.sh`, served under
  `/loic-truchot/projects/haiku-generator/`; the fingerprints must equal the
  2020 ones.
- Phase 4, the four accepted changes, one commit each: `aiku.wtf` URLs,
  Google Analytics removed, "raie", tweet in a new tab.
- Phase 5, deploy: `render.yaml`, `scripts/render-build.sh`, CI,
  `docs/hub-patch/developers-hub.diff`.
- Phase 6, cleanup: Makefile and AWS scripts, stale files, LICENSE, version
  0.1.0, README.
- Skills and `.claude/settings.json` (hook after a `.rs` edit) once the Rust
  toolchain builds the crate.
