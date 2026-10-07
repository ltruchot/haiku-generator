---
name: characterization-tests
description: The tests that pin what the generator produces and what the page renders - Rust goldens, exhaustive enumeration of the sentence patterns, browser fingerprints compared with the 2020 build. Use when a golden or a fingerprint test fails, before a refactoring or an upgrade, and when a change of output is wanted and must be shown exactly.
paths: "src/characterization/**, tests/golden/**, packages/e2e/**"
---

# Pinning behaviour

Nothing here is a specification: the tests describe what the 2020 program
does, oddities included. A failure means the output moved. Either the change
is a regression, or it is wanted and its exact extent goes in the commit.

## Rust: `src/characterization/`, goldens in `tests/golden/`

| File                            | Pins                                                                                                                                             |
| ------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| `grammar.rs`                    | Every noun × number × article, adjective × gender × number, verb × number, with feet; the category tables                                        |
| `helpers.rs`                    | Table tests of the pure helpers (`add_words`, plural, feminine, `check_haiku_form`, blacklists)                                                  |
| `enumeration.rs`, `odometer.rs` | The **exact set** of lines each of the 5 patterns can produce: an odometer `Chooser` walks the whole decision tree of the real closures          |
| `patterns.rs`                   | That set against the goldens, and a converged Monte-Carlo of the real `RandChooser` reaching the same set (release builds; `#[ignore]` in debug) |
| `haiku.rs`, `retry.rs`          | Properties of a haiku; the retry and reshuffle loop, through a scripted chooser                                                                  |
| `seam.rs`                       | The only test file that builds the production chooser                                                                                            |

- Run: `mise exec -- cargo test` and `mise exec -- cargo test --release`
  (the release run adds the full Monte-Carlo, about 6 s).
- No seeded-RNG golden: the category tables are `HashMap`s, their iteration
  order changes per process. Sets and sorted dumps only.
- A wanted change of output: make it, then `UPDATE_GOLDEN=1 mise exec --
cargo test`, then **read `git diff --stat tests/golden` and the diff**. The
  commit states what was added and removed (see the "raie" commit: 1,098
  lines added, each with the word, nothing else removed). A few tests hold
  counts derived from the goldens (`haiku.rs`): update them from the failure
  message, with the comment that explains the number.
- Known oddities, pinned on purpose, not to be fixed in passing: the error
  line `#err#generate_haiku#no combination found` can be a third line; a
  pattern can serve two lines after a reshuffle; a verb can repeat between
  two intransitive lines; "bigarré" and "doux" carry another adjective's id
  and are never drawn.

## Browser: `packages/e2e/`

`fixtures/legacy-dist/` is the last build deployed in 2020, served at `/`.
Today's page is `apps/web/dist`, built for its deploy path and served with
the headers of `render.yaml`.

- `behaviour.ts`: what the page does (haiku, night mode, modal), for both.
- `fingerprint.ts`: DOM tree, computed style and box of every element,
  full-page screenshot; 2 widths × day, night, modal. `web.test.ts` requires
  **equal** fingerprints, to the pixel. Both pages are captured in the same
  Chromium, in the same run: no stored screenshot, no font or OS variance.
- An accepted difference is code, not a tolerance: `accepted()` in
  `web.test.ts` applies it to the 2020 page before the capture (remove the
  three share links, new `og:image`). Add a line there, never a threshold.
- What the comparison sets aside, in `read()`: scripts and style sheet links,
  `style` attributes (compared as computed styles), content hashes in file
  names, and what the 2020 minifier did to the icons (`role`, `<title>`,
  `viewBox`, path data).
- Screenshots land in `.local/screenshots/` (`legacy-*`, `web-*`).
  A pixel diff: PIL `ImageChops.difference(a, b).getbbox()` names the box.

## Traps

| Symptom                                    | Cause                                                                                                     | Fix                                                                |
| ------------------------------------------ | --------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| `apps/web/dist missing`                    | e2e run alone                                                                                             | `vp run web#ready`, or `vp run ready`                              |
| `screenshots differ`, DOM and layout equal | A shape or an image differs by anti-aliasing                                                              | Diff the two PNGs; an svgo-rewritten path gave 55 pixels once      |
| A style lands on the wrong element         | Selector wider than the element that had the inline style                                                 | The layout diff names the element and the property                 |
| Class order differs in night mode          | `classList.replace` keeps the position, the 2020 code removed then added                                  | Keep remove + add                                                  |
| `listen EADDRINUSE` on port 0              | Another process took every IPv4 ephemeral port (a port-forwarding loop of the editor's remote server did) | The server falls back to `[::1]`; find the culprit with `ss -tanp` |
| Tests hang 30 s each                       | The page never shows a haiku (wasm blocked by CSP, module missing)                                        | Run one test: `vp -C packages/e2e test -t "loads without"`         |
