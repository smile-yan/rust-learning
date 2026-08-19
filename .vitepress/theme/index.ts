import DefaultTheme from "vitepress/theme";
import type { Theme } from "vitepress";
import RustPlayground from "./components/RustPlayground.vue";

export default {
  extends: DefaultTheme,
  enhanceApp({ app }) {
    app.component("RustPlayground", RustPlayground);
  }
} satisfies Theme;
