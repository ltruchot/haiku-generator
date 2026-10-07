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
  Facebook, Twitter and short-edition links are gone, Google Analytics is
  gone, `og:image` points to the new address, the word "raie" can be drawn,
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
