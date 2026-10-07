---
name: render-subpath-deploy
description: How the page gets from this repo to https://developers.gods.academy/loic-truchot/projects/haiku-generator/ - Render static site, build script, security headers, the rewrite in the developers-hub repo, checks after a deploy. Use before touching render.yaml, scripts/render-build.sh, .github/workflows/ci.yml or the base path, and when a deploy did not happen or the public page is broken.
paths: "render.yaml, scripts/render-build.sh, .github/workflows/**, docs/hub-patch/**, apps/web/vite.config.ts"
---

# Deploy under a sub-path

## The chain

1. A push to `master` runs `.github/workflows/ci.yml` (`vp run ready`).
2. Render deploys the static site `haiku-generator` when that check is green
   (`autoDeployTrigger: checksPass`): `bash scripts/render-build.sh`, publish
   `./site`.
3. `site/` mirrors the public URL:
   `site/loic-truchot/projects/haiku-generator/index.html`. The same path
   works on the `onrender.com` host and behind the hub.
4. `developers.gods.academy` belongs to another Render service (repo
   `Gods-Academy/developers-hub`), which rewrites the path to this site.
   Render: one domain, one service. This site has no custom domain.

The hub is another repo: its two rewrite rules and its listing entry are in
`docs/hub-patch/` as a patch, with the steps.

## Base path

- `apps/web/vite.config.ts`: `base: process.env["BASE"] ?? "./"`. Relative by
  default; the deploy build sets `BASE=/loic-truchot/projects/haiku-generator/`.
- In `index.html`, a source asset is `./src/assets/…`, a file of `public/` is
  `/name` (Vite prefixes the base). Never a hand-written absolute path: the
  2020 page had one (`/public/…`) and it broke under a sub-path.
- `robot-profil.jpg` stays in `public/`, unhashed: `og:image` names its
  absolute URL, which must be stable.
- File names are `name.hash.ext` (`rolldownOptions.output`): the e2e
  fingerprint strips that shape.

## Headers

All in `render.yaml`, none in the hub. The CSP is `default-src 'self'` with
`script-src 'self' 'wasm-unsafe-eval'`: the second token is what lets the
browser compile the module. No inline script, no inline style: a `style`
attribute or a `<script>` block in `index.html` is blocked.
`packages/e2e/tests/server.ts` reads the `/*` headers out of `render.yaml`
and sends them, so a violation fails locally.

## After a push

- `gh run watch <id> --exit-status`: a red run blocks the deploy with no
  other sign.
- A push whose **last** commit touches only `buildFilter.ignoredPaths`
  (docs, `.claude/`, `packages/e2e/`, tests) may deploy nothing, not even
  the code commits before it. Push the code commit last, or deploy by hand.
- Then the `curl` checks of `docs/hub-patch/README.md`.

## Not verified yet

Record the answer here once the first deploy has run.

- Whether a Render **static** build can install mise and a Rust toolchain
  (home directory writable, downloads allowed). The script logs the PATH and
  which tools exist. Fallback: install rustup directly in the script.
- Whether Render serves `.wasm` as `application/wasm`. If not, the glue falls
  back to a slower non-streaming compile and logs a warning.
- Whether `vp install --frozen-lockfile` accepts in CI a lockfile written by
  the pnpm 10 this machine uses (`devEngines` asks for 12.5.1).
- The site's real host: Render may suffix the name (`haiku-generator-xxxx`).
