# TODO

## Where we stopped

- Branch `chore/modernize`, 2026-10-07: the modernization is done and
  committed locally, `vp run ready` green. Nothing is pushed, nothing is
  deployed. `master` is still the 2020 code.
- Next step is the owner's: push the branch, open the pull request (CI runs
  for the first time there), merge.

## Later

- First CI run: confirm `vp install --frozen-lockfile` accepts the lockfile
  written by pnpm 10, and that `jdx/mise-action` installs from `mise.lock`.
- Render: create the static site from `render.yaml`, read the first build
  log (can a static build install mise and Rust?), then apply
  `docs/hub-patch/` to `Gods-Academy/developers-hub` → `docs/hub-patch/README.md`.
  Record the answers in the skill `render-subpath-deploy`, section "Not
  verified yet".
- After the first deploy: tag `v0.1.0`.
- Words that can never be drawn, as "raie" could not: "bigarré" (id of
  "hâlé") and "doux" (id of "paisible"); `AdjId::Grand` and
  `AdjId::Diaphane` have no row. Each fix adds lines to the goldens: owner's
  call → skill `haiku-grammar-data`.
- `src/haiku.rs`: a line found at the 10,001st try is replaced by the error
  line but its words are already blacklisted. Pinned by a test, not fixed.
- Remote branches from 2020 to delete: 11 `dependabot/*` (the packages are
  gone with Parcel) and `feat/ontology` if it is abandoned.
- `.vscode/launch.json` still debugs the binary of 2020; harmless.
- Skills that would serve other repos, once a second one needs them
  (qol-mini's entry rule): `changelog-upgrade`, `characterization-tests`,
  `render-subpath-deploy`.
