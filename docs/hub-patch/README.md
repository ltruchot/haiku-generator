# Putting the page behind developers.gods.academy

The domain belongs to the hub (`Gods-Academy/developers-hub`), which rewrites
`/loic-truchot/projects/haiku-generator/*` to this repo's own Render static
site. Two things happen outside this repo, in this order:

1. **Render**: create the static site from `render.yaml` (Blueprint, repo
   `ltruchot/haiku-generator`, branch `master`). No custom domain. Note its
   public host, `<name>.onrender.com`: Render may add a suffix to the name.
2. **Hub**: in a clone of `developers-hub`, `git apply -p1 developers-hub.diff`,
   replace `haiku-generator.onrender.com` by the host of step 1 if it differs
   (two lines of `render.yaml`), commit, push `main`. The hub redeploys itself.

`developers-hub.diff` adds the two rewrite rules (bare path and `/*`) and one
entry in the two copies of the listing page. It was generated against the hub
at its commit of 2026-10-07; if it no longer applies, the three edits are
small enough to redo by hand.

Checks once both are live:

```sh
# the security headers, then the wasm file and its content type (application/wasm expected)
curl -sI https://<name>.onrender.com/loic-truchot/projects/haiku-generator/ | grep -i content-security-policy
curl -s https://<name>.onrender.com/loic-truchot/projects/haiku-generator/ | grep -o 'assets/[^"]*\.wasm'
curl -sI https://<name>.onrender.com/loic-truchot/projects/haiku-generator/assets/<file>.wasm | grep -i content-type
# through the hub
curl -sI https://developers.gods.academy/loic-truchot/projects/haiku-generator/robot-profil.jpg | head -n 1
```
