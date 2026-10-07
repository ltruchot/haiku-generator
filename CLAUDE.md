> Session handoff: read `TODO.md` first — where the last session stopped, and what is left.
> Open a `todo/` file only for the item you work on. An item done → delete its line and its `todo/` file.

# haiku-generator

A.I.ku: a French haiku generator. The grammar and the word data are a Rust
crate (repo root), compiled to WebAssembly and shown by one static page.
Served at https://developers.gods.academy/loic-truchot/projects/haiku-generator/
(Render static site, behind the rewrites of the `Gods-Academy/developers-hub` repo).

The project dates from 2020 and is being modernized **without changing what
the page shows or does**. That constraint outranks every cleanup.

## Rules

- The developer speaks French: answer in French. Everything in the repo (code,
  comments, docs, commits) is US English, except the page's text and the word
  data, which are French.
- The page renders and behaves like the 2020 build kept in
  `packages/e2e/fixtures/legacy-dist/`. The only accepted differences: the
  Facebook, Twitter and short-edition links are gone and the three icons
  left are grouped (centered on a phone, at the left on a wide screen), Google Analytics is gone, `og:image` points to the new address, the word "raie" can be drawn,
  and the three inline `style` attributes live in the style sheet. Each one is
  written as code in `packages/e2e/tests/web.test.ts` (`accepted`). Anything
  else that changes a fingerprint or a golden is a regression, however
  sensible it looks.
- Never edit `packages/e2e/fixtures/legacy-dist/` or the vendored Tailwind
  1.2.0 file: their bytes are the reference.
- Never "fix" the page's HTML (the `<ul>` inside a `<p>` is invalid; the
  parser's recovery is part of the reference DOM), nor the grammar rules or the
  word data, as a side effect of another change. A wanted change of output is
  its own commit, with the golden diff that shows exactly what it adds.
- Goldens and fingerprints first, the change after. One dependency, one major
  version, one commit; its changelog read before the bump.
- "Done" = `vp run ready` green. Never weaken a gate (lint rule, clippy lint,
  test, golden) to pass: fix the code. Never `git commit --no-verify`.
- Commit only when asked, on a branch, never on `master` directly. Small
  conventional commits (`feat:`, `fix:`, `chore:`, `test:`, `docs:`).
- Record a new fact or trap the same turn, in the file that owns it (table
  below): short bullets with the command, the error text and the fix.

## Where the detail lives

Load the skill before the work, and record what you learn in it.

| Work                                                            | Skill                               |
| --------------------------------------------------------------- | ----------------------------------- |
| Words, categories, sentence patterns; a word that never appears | `haiku-grammar-data`                |
| A golden or a fingerprint fails; proving a change of output     | `characterization-tests`            |
| Cargo.toml, the wasm build, the module in the page              | `rust-wasm-build`                   |
| A tool version, `mise.toml`, `mise.lock`                        | `mise-toolchain`                    |
| Updating a dependency, a toolchain, an edition                  | `changelog-upgrade`                 |
| `render.yaml`, the build script, CI, the base path, the hub     | `render-subpath-deploy`             |
| Any `vp` command, `package.json`, `vite.config.ts`              | `vp` (managed by qol-mini)          |
| npm audit, a CVE, before an npm bump                            | `npm-vulnerability-check` (managed) |
| Reviewing a pull request                                        | `qa-pr` (managed)                   |

Managed skills (`.claude/qol-mini.lock.json`) are never edited here: what is
specific to this repo goes in a local skill beside them.

## Environment

- Rust, the wasm target, `wasm-bindgen` and `wasm-opt` come from mise:
  `mise.toml` is the only place a toolchain version is written, `mise.lock`
  is committed. Run them as `mise exec -- cargo …` (no Rust on the PATH).
- Node and pnpm are owned by Vite+: every JS command goes through `vp`
  (`vp install`, `vp check`, `vp test`, `vp run <task>`), never `pnpm`, `npm`
  or `npx` directly. Node and pnpm are not in `mise.toml`.
- npm packages must be 3 days old to be installed (`minimumReleaseAge` in
  `pnpm-workspace.yaml`); mise applies its own release-age guard.
- Local state (screenshots, scratch) goes in `.local/`, gitignored.

## Architecture

- `src/` is one crate, `cdylib` + `rlib`. `lib.rs`: `compose()` (three lines
  or the errors) and `generate()`, its wasm export. `main.rs`: the CLI.
- A haiku: the 5 sentence patterns (`combination.rs`) are shuffled, line n
  takes pattern n and retries until its feet cover 5, 7, 5; reshuffle after
  1000 failed tries, `#err#…` after 10,001. Blacklists keep a noun, an
  adjective or a season from coming twice.
- Every random choice goes through `Chooser` (`random.rs`, the only file
  that imports `rand`): the tests swap it for an odometer and enumerate every
  line a pattern can produce.
- `apps/web/`: `index.html` (the 2020 markup), `src/main.ts`, `src/index.scss`,
  `src/wasm/` (generated, gitignored), `public/vendor/tailwind-1.2.0.min.css`.
  No framework, no router, no storage, no request to another site.
- `packages/e2e/`: Playwright through Vitest; `fixtures/legacy-dist/` is the
  2020 build.

## Commands

| Command                                   | Does                                                                                                                                |
| ----------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| `vp run ready`                            | The whole gate: versions, rustfmt, clippy, Rust tests (debug and release), cargo audit, wasm, `vp check`, page build, browser tests |
| `mise run rust-check`                     | The Rust half of it                                                                                                                 |
| `mise run wasm`                           | Build the module into `apps/web/src/wasm/`                                                                                          |
| `mise exec -- cargo test`                 | Rust tests; `--release` adds the full Monte-Carlo                                                                                   |
| `UPDATE_GOLDEN=1 mise exec -- cargo test` | Rewrite the goldens: read the diff                                                                                                  |
| `vp check --fix`                          | Format and lint the JS side                                                                                                         |
| `vp -C apps/web dev`                      | The page on http://localhost:4062 (needs the module built)                                                                          |
| `vp -C packages/e2e test`                 | Browser tests (needs `vp run web#ready`)                                                                                            |
| `bash scripts/render-build.sh`            | What Render runs: the site in `site/`                                                                                               |

## Traps with no file trigger

- zsh: an unquoted `--include=*` or `web#ready` glob fails (`no matches
found`); quote them.
- `pkill -f <pattern>` also kills the shell whose command line holds the
  pattern. Kill by PID.
- Port 4060 is taken on this machine by the editor's port forwarding; the
  dev server uses 4062, the preview 4063.
- `vp` uses the pnpm of the machine (10.x from the global mise config), not
  the `devEngines` one: the lockfile is written by pnpm 10.
- `vp --help` after a subcommand of `vp dlx <pkg>` is taken by `vp`, not by
  the package.
