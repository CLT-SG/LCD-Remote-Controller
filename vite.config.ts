import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

// Vite config for the Vue front-end. The frontend runs on Vite during development
// and is bundled into ./dist/ for production where it is served by the embedded
// Rust HTTP server (so the desktop WebView and mobile browsers consume the same
// asset bundle).
export default defineConfig(async () => ({
  plugins: [vue()],
  clearScreen: false,
  server: {
    port: 5173,
    strictPort: true,
    host: "0.0.0.0",
    // Forward API + websocket calls during dev to the embedded Rust HTTP server.
    proxy: {
      "/api": {
        target: "http://127.0.0.1:8765",
        changeOrigin: true,
      },
    },
  },
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    target: "es2021",
    sourcemap: true,
    outDir: "dist",
    emptyOutDir: true,
  },
}));
