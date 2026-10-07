import { defineConfig } from "vite-plus";

// Workspace root: format, lint and staged checks for every package (Vite+
// does not apply nested lint/fmt blocks). The page: apps/web/vite.config.ts.
const FROZEN = [
  // The last 2020 build, the reference of every browser comparison: never rewritten.
  "packages/e2e/fixtures/**",
  // Goldens are written by the Rust tests (UPDATE_GOLDEN=1).
  "tests/golden/**",
  // Tailwind 1.2.0 as shipped: its bytes are the rendering.
  "**/vendor/**",
  // The Parcel 1 app, untouched until it moves to apps/web.
  "wasm-demo/**",
  "target/**",
  // Managed by qol-mini (.claude/qol-mini.lock.json): one entry per managed
  // skill, never .claude/skills/ as a whole (local skills are checked).
  ".claude/skills/vp/**",
  ".claude/skills/npm-vulnerability-check/**",
  ".claude/skills/qa-pr/**",
  // 2020 files, rewritten or removed with the cleanup.
  "README.md",
  ".vscode/**",
];

export default defineConfig({
  staged: {
    "*.{ts,js,json,css,scss,html,md,yaml}": "vp check --fix",
  },
  fmt: { ignorePatterns: FROZEN },
  lint: {
    ignorePatterns: FROZEN,
    plugins: ["eslint", "typescript", "unicorn", "oxc", "import", "promise", "node", "vitest"],
    jsPlugins: [{ name: "vite-plus", specifier: "vite-plus/oxlint-plugin" }],
    categories: {
      correctness: "error",
      suspicious: "error",
      pedantic: "error",
      perf: "error",
    },
    rules: {
      "vite-plus/prefer-vite-plus-imports": "error",
      // Style imports are side effects by design (Vite bundles them).
      "import/no-unassigned-import": ["error", { allow: ["**/*.css", "**/*.scss"] }],
      // Inferred callback parameters, DOM nodes and Playwright pages cannot be
      // made deeply readonly; the rest must be.
      "typescript/prefer-readonly-parameter-types": [
        "error",
        {
          ignoreInferredTypes: true,
          allow: [
            { from: "lib", name: ["HTMLElement", "Element"] },
            { from: "package", package: "playwright-core", name: ["Page", "Browser"] },
            { from: "package", package: "@types/node", name: "Buffer" },
          ],
        },
      ],
    },
    options: {
      typeAware: true,
      typeCheck: true,
      denyWarnings: true,
      reportUnusedDisableDirectives: "error",
    },
  },
});
