import { defineConfig } from "vite-plus";

export default defineConfig({
  test: { include: ["tests/**/*.test.ts"], testTimeout: 60_000, hookTimeout: 60_000 },
});
