import { defineConfig } from "vite-plus";

// Deploy target: BASE=/loic-truchot/projects/haiku-generator/ vp build
// Relative by default, so dist/ opens from any path.
export default defineConfig({
  base: process.env["BASE"] ?? "./",
  server: { port: 4062, strictPort: true, host: true },
  preview: { port: 4063, strictPort: true, host: true },
  build: {
    // 2020 pages are compared with this one: same browsers in mind, and file
    // names as `name.hash.ext`, which the e2e fingerprint reduces to `name.ext`.
    rolldownOptions: {
      output: {
        entryFileNames: "assets/[name].[hash:8].js",
        chunkFileNames: "assets/[name].[hash:8].js",
        assetFileNames: "assets/[name].[hash:8][extname]",
      },
    },
  },
});
