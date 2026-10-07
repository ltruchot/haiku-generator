---
name: vp
description: Vite+ (vp) in a pnpm workspace. Run `vp <command>` instead of pnpm, npm, vite, vitest, oxlint or oxfmt directly. Use before touching package.json, vite.config.ts or pnpm-workspace.yaml, and before running install, dev, build, test, lint or a task.
paths:
  - "**/package.json"
  - "**/vite.config.ts"
  - "pnpm-workspace.yaml"
---

# Vite+ (`vp`)

Written against Vite+ 1.0: Vite 8, Rolldown, Vitest 5, Oxlint, Oxfmt, tsdown, Vite Task.
`vp --version` and `vp toolchain` print what this project runs.

The docs that match the installed version are local: `node_modules/vite-plus/docs/guide/` and
`node_modules/vite-plus/docs/config/`. Read the page before guessing a flag. Online:
<https://viteplus.dev/guide/>.

## Rule 1: `vp <command>`, never the tool underneath

| Task | Command | Notes |
|---|---|---|
| Install | `vp install` | CI: `vp install --frozen-lockfile`, which fails if the lockfile would change |
| Add, remove a dependency | `vp -C apps/web add <pkg>` (`-D` dev, `-E` exact), `vp remove <pkg>` | then move the version to the catalog, see below |
| Dev server | `vp -C apps/web dev` | <http://localhost:4060> |
| Production build | `vp -C apps/web build` | apps; libraries build with `vp pack` |
| Serve the build | `vp -C apps/web preview` | after `vp build` |
| Format, lint, types | `vp check`, `vp check --fix` | from the root, whole workspace; `vp check --fix <file>` for one file |
| Tests | `vp -C apps/web test` | runs once; `vp test watch` watches; `vp test run --coverage` |
| A script or a task | `vp run <name>`, short form `vpr <name>` | `package.json` script or `run.tasks` entry |
| A one-off binary | `vp exec <bin>` (local), `vp dlx <pkg>` or `vpx <pkg>` (downloaded) | `vp dlx` adds no dependency |
| Versions | `vp toolchain`, `vp why <pkg>`, `vp outdated`, `vp pm view <pkg> version` | `vp why` shows the package-manager graph only |

- `vp build` is the builtin and cannot be overridden; `vp run build` is the script or task of
  that name. Same for `dev`, `test`, `preview`, `lint`, `fmt`, `check`, `pack`.
- `vp pm <subcommand>` forwards a fixed list to the package manager (`view`, `list`, `audit`,
  `cache`, `config`, `publish`, `pack`, `ci`, `approve-builds`, `patch`, ...), not anything.
  Extra arguments go after `--`. `vp pm --help` prints the list.

## Targeting a package

- `vp -C <dir> <command>` runs any command as if started in `<dir>`. Use it for builtins.
- A positional directory (`vp dev apps/web`) only sets Vite's `root` and exists on `dev`,
  `build` and `preview` only. Prefer `-C`.
- `vp test <arg>` filters test files, and `vp pack <arg>` names entry files: neither targets a
  package.
- At the workspace root, a bare `vp dev`, `vp build`, `vp preview` or `vp pack` asks which
  package, and in a non-interactive shell exits 1 with `needs a target package`.
  `defaultPackage` in the root `vite.config.ts` fixes the target.
- `-F`, `--filter` selects packages on `vp run`, `vp exec` and the package-manager commands
  only. On `vp dev` and `vp build`, `-f` filters debug logs.

## `vp run`

- `-r` every package, `-t` this package and its dependencies, `-F <pattern>` pnpm filter
  syntax, `-w` the workspace root, `<package>#<task>` one task of one package.
- `--parallel` drops dependency ordering. `--cache` and `--no-cache` override the cache.
- A task name lives in `vite.config.ts` (`run.tasks`) or in `package.json`, never in both.
- Tasks are cached by default, scripts are not. Set `cache: false` on a dev server and on
  anything that must always run.
- A task runs in a clean environment: only a short list (`PATH`, `HOME`, `CI`, ...) passes.
  Another variable goes in `cache.env` (part of the cache key) or `cache.untrackedEnv`.
- A command string is read by a shell. `&&` chains are split into sub-tasks cached one by
  one. An array `command` is a sequence of commands, not an argument vector.

## Configuration: one `vite.config.ts`

- `import { defineConfig } from "vite-plus"` in config files. Other files keep
  `from "vite"`. Tests import from `vite-plus/test`.
- Blocks: `run` (with `run.tasks`), `fmt`, `lint`, `check`, `test`, `pack`, `staged`, `create`,
  plus `defaultPackage`, beside Vite's own `server`, `build`, `preview`, `plugins`.
- No `vitest.config.ts`, `tsdown.config.ts`, `.oxlintrc.json` or `.oxfmtrc.json`.
- Keep `lint` and `fmt` in the root config. `vp check` reads the root blocks only. A rule for
  one package goes in `lint.overrides` or `fmt.overrides`, with globs relative to the root.
- Type checks run in `vp check` when `lint.options.typeAware` and `lint.options.typeCheck`
  are on.
- Vitest collects `*.test.*` and `*.spec.*`. When Playwright specs share the repo, set
  `test.include` or name them `*.e2e.ts`.

## Dependencies in a pnpm workspace

- Shared versions live in `catalog:` in `pnpm-workspace.yaml`; a package lists
  `"<dep>": "catalog:"`, a workspace dependency `"workspace:*"`.
- `minimumReleaseAge` is in minutes and defaults to 1440 since pnpm 11. A release younger
  than that fails to install: take the previous version, or list the package in
  `minimumReleaseAgeExclude` when the reason is known.
- Exact versions: `vp add -E`, or `savePrefix: ""`.
- `vite-plus` aliases `vite` and pins `vitest` through `overrides`. Bump the three together:
  `node_modules/vite-plus/docs/guide/upgrade-project.md`.

## Node and the package manager

- Node resolves from the nearest of, in order: `.node-version`, `devEngines.runtime`,
  `engines.node`, `.nvmrc`. `vp env pin <version>` writes the pin, `vp env current` shows
  what resolved and from where, `vp env doctor` diagnoses.
- The package manager comes from `packageManager`, then `devEngines.packageManager`. With
  both, `packageManager` drives and a mismatch warns.
- `vp env`, `vp node <file>`, `vp upgrade` need the global CLI. `node: command not found`
  means no global Node: `vp node <file>`.

## Git hooks

`"prepare": "vp config"` installs the dispatcher in `.vite-hooks/_` (ignored by git) and sets
`core.hooksPath`. It does not write the hook: `.vite-hooks/pre-commit` is a committed file that
runs `vp staged`, which reads the `staged` block of the root `vite.config.ts`.
`vp hooks status` shows the state. Never `git commit --no-verify`.

## More

- [references/traps.md](references/traps.md): lint and type errors that recur, a dev server
  port that stays busy, the editor settings. Read it when a `vp check` error or a port error
  is not obvious.
