# A.I.ku, haiku generator

An artificial poet that composes haikus in French:
https://developers.gods.academy/loic-truchot/projects/haiku-generator/

An experiment from 2020, to try a few technologies together and to play with
the French language:

- the poet is a Rust crate: sentence patterns, hand-entered words, agreement
  in gender and number, and a count of feet until a line has 5 or 7 of them;
- it is compiled to WebAssembly and runs in the page, with no server;
- the page is one HTML file styled with Tailwind 1.2.0;
- the pictures come from Unsplash.

## Layout

| Path                      | What                                                                                                                  |
| ------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| `src/`, `Cargo.toml`      | The crate: grammar (`noun.rs`, `adj.rs`, `verb.rs`), word data (`*_data.rs`), patterns (`combination.rs`), `haiku.rs` |
| `tests/golden/`           | Every form and every line the crate can produce, as text files                                                        |
| `apps/web/`               | The page                                                                                                              |
| `packages/e2e/`           | Browser tests against the 2020 build kept in `fixtures/legacy-dist/`                                                  |
| `scripts/`, `render.yaml` | wasm build, Render build and blueprint                                                                                |

## Tools

[mise](https://mise.jdx.dev) installs Rust and the wasm tools at the versions
of `mise.toml`; [Vite+](https://viteplus.dev) (`vp`) owns Node, pnpm and every
JS command.

```sh
mise install          # Rust, wasm32 target, wasm-bindgen, wasm-opt, cargo-audit
vp install            # JS dependencies
```

## Commands

```sh
mise exec -- cargo run        # a haiku in the terminal
mise run wasm                 # build the wasm module into apps/web/src/wasm
vp -C apps/web dev            # the page, http://localhost:4062
vp run ready                  # the whole gate: Rust checks, wasm, lint, build, browser tests
bash scripts/render-build.sh  # what Render runs: the site in site/
```

A change of the words or of the grammar shows in the goldens: review the
diff, then `UPDATE_GOLDEN=1 mise exec -- cargo test` rewrites them.

## Deploy

Render builds the static site from `render.yaml` once CI is green on
`master`. The public address is a rewrite in another repo: see
`docs/hub-patch/README.md`.

## License

[CC BY-NC 4.0](LICENSE).
