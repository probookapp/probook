import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";
import path from "path";

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [react(), tailwindcss()],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },

  build: {
    chunkSizeWarningLimit: 1600,
    rollupOptions: {
      output: {
        manualChunks: {
          // React core
          "vendor-react": ["react", "react-dom", "react-router-dom"],
          // PDF generation (largest dependency)
          "vendor-pdf": ["@react-pdf/renderer"],
          // Rich text editor
          "vendor-editor": [
            "@tiptap/react",
            "@tiptap/starter-kit",
            "@tiptap/extension-color",
            "@tiptap/extension-font-family",
            "@tiptap/extension-text-style",
            "@tiptap/extension-underline",
          ],
          // Data fetching & state
          "vendor-state": ["@tanstack/react-query", "zustand", "react-hook-form", "@hookform/resolvers"],
          // i18n
          "vendor-i18n": ["i18next", "react-i18next", "i18next-browser-languagedetector"],
          // Tauri plugins
          "vendor-tauri": [
            "@tauri-apps/api",
            "@tauri-apps/plugin-dialog",
            "@tauri-apps/plugin-fs",
            "@tauri-apps/plugin-opener",
            "@tauri-apps/plugin-process",
            "@tauri-apps/plugin-updater",
          ],
        },
      },
    },
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
