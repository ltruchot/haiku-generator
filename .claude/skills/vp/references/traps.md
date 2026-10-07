# Traps

Each row names a symptom, its cause and the fix. Rows marked *measured* come from projects
using this skill, not from the Vite+ docs.

## `vp check`

| Symptom | Cause | Fix |
|---|---|---|
| `TS4111: Property 'X' comes from an index signature` | `noPropertyAccessFromIndexSignature` | `process.env["X"]`, not `process.env.X` |
| Oxlint `no-unsafe-*` on `JSON.parse` or `res.json()` | the platform returns `any` | type the result `unknown` and narrow it; never cast |
| `strict-void-return` on `(e) => set.add(e.code)` | an arrow returns a value where a `void` callback is expected | a block body: `(e) => { set.add(e.code); }` |
| `vitest(no-conditional-in-test)` | an `if`, `?:`, `?.` or `??` inside `it` or `test` | move the logic to a module-level helper |
| `consistent-function-scoping` on a helper inside `page.evaluate` | the callback runs in the browser and cannot use module helpers | inline the code in the callback |
| A package-level `lint` or `fmt` block changes nothing in `vp check` | `vp check` reads the root blocks only | `lint.overrides` or `fmt.overrides` in the root config |
| Type-aware lint fails on `compilerOptions.baseUrl` | tsgolint does not support it | use `paths` without `baseUrl` |

## Running

| Symptom | Cause | Fix |
|---|---|---|
| `needs a target package`, exit 1 | a bare builtin at the workspace root, in a non-interactive shell | `vp -C <dir> <command>`, or `defaultPackage` |
| A task does not see an environment variable | tasks run in a clean environment | `cache.env` or `cache.untrackedEnv` on the task |
| A task prints nothing new and ends at once | cache hit | `vp run --no-cache <task>`; `cache: false` if it must always run |
| `EBUSY` or `Device or resource busy` when a task starts a binary, and the same command passes outside `vp run` *(measured)* | the task cache tracks file access, and a statically linked binary cannot start under it | `vp run --no-cache <task>`, or `cache: false` on the task |
| `vp test` runs the Playwright specs | Vitest collects `*.spec.*` | `test.include`, or name them `*.e2e.ts` |
| `vp install --frozen-lockfile` fails in CI on a `node@runtime` entry *(measured)* | pnpm 11+ manages `devEngines.runtime` itself and records it in the lockfile | pin Node in `.node-version`, or `pnpm config set --global runtimeOnFail ignore` |
| `ERR_PNPM_IGNORED_BUILDS: <pkg>` | a dependency has an install script that pnpm did not run | `allowBuilds` in `pnpm-workspace.yaml`, `true` or `false` per package, after reading the script |

## Dev server

`server: { host: true, port: 4062, strictPort: true }`: `host: true` listens on every
address, which an editor on another host needs to forward the port (WSL, a container, SSH);
`strictPort` exits when the port is taken instead of moving to the next one.

| Symptom | Cause | Fix |
|---|---|---|
| `Port 4062 is already in use`, and you started the server | an earlier run still holds it | find the PID with `ss -ltnp \| grep 4062` and stop that PID; never `pkill -f <pattern>`, which matches your own shell |
| Same error under WSL while `ss -ltn` shows nothing *(measured)* | a Windows process holds the port: WSL shares the network | `netstat.exe -ano -p tcp \| grep ':4062 '`, then `tasklist.exe /FI "PID eq <pid>"` |
| The holder is the editor *(measured)* | VS Code and Cursor keep an auto-forwarded port bound after the server stops | Ports panel, Stop Forwarding Port; `remote.autoForwardPorts` and `remote.restoreForwardedPorts` to `false` in the user settings |

## VS Code

Extension: `VoidZero.vite-plus-extension-pack`. In `.vscode/settings.json`:

```json
{
  "editor.defaultFormatter": "oxc.oxc-vscode",
  "[javascript]": { "editor.defaultFormatter": "oxc.oxc-vscode" },
  "[javascriptreact]": { "editor.defaultFormatter": "oxc.oxc-vscode" },
  "[typescript]": { "editor.defaultFormatter": "oxc.oxc-vscode" },
  "[typescriptreact]": { "editor.defaultFormatter": "oxc.oxc-vscode" },
  "oxc.disableNestedConfig": true,
  "oxc.fmt.disableNestedConfig": true,
  "editor.formatOnSave": true,
  "editor.formatOnSaveMode": "file",
  "editor.codeActionsOnSave": { "source.fixAll.oxc": "explicit" }
}
```

- The `[language]` blocks are required: a user-level `[language]` setting outranks the
  workspace `editor.defaultFormatter`.
- `formatOnSaveMode` is `"file"` because Oxfmt does not format a range.
