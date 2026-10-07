---
name: rust-wasm-build
description: Build the Rust crate to WebAssembly and load it in the page - cargo, wasm-bindgen, wasm-opt, the JS glue, the browser's randomness. Use before touching Cargo.toml, src/lib.rs, scripts/build-wasm.sh or the import of the module in apps/web/src/main.ts, and when the page stays empty or a wasm build fails.
paths: "Cargo.toml, src/lib.rs, src/random.rs, scripts/build-wasm.sh, apps/web/src/main.ts"
---

# Rust to wasm

## Pipeline

`mise run wasm` runs `scripts/build-wasm.sh`:

1. `cargo build --release --lib --locked --target wasm32-unknown-unknown`
2. `wasm-bindgen --target web --out-dir apps/web/src/wasm <the .wasm>`
3. `wasm-opt -Os` on `haiku_generator_bg.wasm`, in place

`apps/web/src/wasm/` is gitignored and rebuilt. `vp check` type-checks
`main.ts` against the generated `.d.ts`: build the module first (`vp run ready`
does it in that order). No wasm-pack: three pinned tools, no hidden download.

## The crate

- `crate-type = ["cdylib", "rlib"]`: `cdylib` for the module, `rlib` for
  `src/main.rs` (the CLI) and the tests.
- One exported function: `generate() -> js_sys::Array` of three strings, over
  `compose()`. No DOM access from Rust, no `web-sys`.
- `rand` is imported by `src/random.rs` only, behind the `Chooser` trait. A
  rand upgrade touches `RandChooser` and nothing else.
- In the browser, randomness is `crypto.getRandomValues`: `getrandom` is a
  direct dependency for wasm32 only, with the feature `wasm_js`. With
  getrandom 0.3.4 or later the feature alone selects the backend, no
  `RUSTFLAGS --cfg getrandom_backend` needed. Check after a bump:
  `grep -c getRandomValues apps/web/src/wasm/haiku_generator.js` is not 0.

## The page

```ts
import init, { generate } from "./wasm/haiku_generator.js";
import wasmUrl from "./wasm/haiku_generator_bg.wasm?url";
await init({ module_or_path: wasmUrl });
```

`?url` makes Vite emit the file with a content hash under `base`, so the
module loads from the deploy sub-path. Everything in `main.ts` runs after
`init`, as in the 2020 build. No bundler plugin.

## Traps

| Symptom                                                                                          | Cause                                                              | Fix                                                         |
| ------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------ | ----------------------------------------------------------- |
| `error: older versions of the wasm-bindgen crate are incompatible with current versions of Rust` | wasm-bindgen crate older than 0.2.88                               | Bump the crate and the CLI together                         |
| Schema or version mismatch error from `wasm-bindgen`                                             | CLI (`mise.toml`) and crate (`Cargo.toml`, pinned `=x.y.z`) differ | Same version in both: `bash scripts/check-versions.sh`      |
| Page empty, no haiku, test `page.waitForFunction: Timeout`                                       | CSP without `'wasm-unsafe-eval'` in `script-src`                   | `render.yaml`; the e2e server sends those headers           |
| `this lint expectation is unfulfilled` on `dead_code`                                            | The tests read a field the library does not                        | `#[cfg_attr(not(test), expect(dead_code, reason = "..."))]` |
| `cargo clippy --fix` applies nothing                                                             | One suggestion breaks the build, all are rolled back               | `--broken-code`, then `cargo check` and repair              |
