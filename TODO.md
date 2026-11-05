# TODO

## Where we stopped

- 2026-11-05: the modernized page is live on Render since 2026-10-17
  (`haiku-generator-1mfx.onrender.com`). The Vite+ 1.5.0 push failed its
  Render build (Render's own pnpm install); branch
  `fix/render-skip-install-deps` sets `SKIP_INSTALL_DEPS=true`. Next: merge
  it, confirm the deploy goes live, and that the Blueprint applied the env
  var (else set it in the dashboard).

## Later

- Hub: apply `docs/hub-patch/` to `Gods-Academy/developers-hub` if not done
  → `docs/hub-patch/README.md`, then its `curl` checks.
- After the first deploy behind the hub: tag `v0.1.0`.
- Words that can never be drawn, as "raie" could not: "bigarré" (id of
  "hâlé") and "doux" (id of "paisible"); `AdjId::Grand` and
  `AdjId::Diaphane` have no row. Each fix adds lines to the goldens: owner's
  call → skill `haiku-grammar-data`.
- `src/haiku.rs`: a line found at the 10,001st try is replaced by the error
  line but its words are already blacklisted. Pinned by a test, not fixed.
- Remote branches from 2020 to delete: 11 `dependabot/*` (the packages are
  gone with Parcel) and `feat/ontology` if it is abandoned. Local branches
  already merged: `chore/modernize`, `fix/lockfile-pnpm-12`,
  `fix/render-build-rustup-home`.
- `.vscode/launch.json` still debugs the binary of 2020; harmless.
- Skills that would serve other repos, once a second one needs them
  (qol-mini's entry rule): `changelog-upgrade`, `characterization-tests`,
  `render-subpath-deploy`.
