import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import { resolve } from "path";

function removeCrossorigin() {
  return {
    name: "remove-crossorigin",
    transformIndexHtml(html) {
      return html
        .replace(/<script([^>]*?)\s+crossorigin([^>]*)>/gi, "<script$1$2>")
        .replace(/<link([^>]*?)\s+crossorigin([^>]*)>/gi, "<link$1$2>");
    },
  };
}

export default defineConfig({
  plugins: [vue(), removeCrossorigin()],
  base: "./",
  publicDir: "public",
  resolve: {
    alias: {
      // 页面模板直接写在 HTML 中，需要带编译器的 Vue 版本
      vue: "vue/dist/vue.esm-bundler.js",
    },
  },
  build: {
    outDir: "dist",
    emptyOutDir: true,
    rollupOptions: {
      input: {
        index: resolve(__dirname, "index.html"),
        app: resolve(__dirname, "app.html"),
      },
      output: {
        entryFileNames: "assets/[name]-[hash].js",
        chunkFileNames: "assets/[name]-[hash].js",
        assetFileNames: (assetInfo) => {
          const info = assetInfo.name.split(".");
          const ext = info[info.length - 1];
          if (/\.(png|jpe?g|gif|svg|webp|ico)$/i.test(assetInfo.name)) {
            return "images/[name]-[hash][extname]";
          }
          return "assets/[name]-[hash][extname]";
        },
      },
    },
  },
});
