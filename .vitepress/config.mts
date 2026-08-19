import { defineConfig } from "vitepress";
import { sidebar } from "./sidebar";

export default defineConfig({
  title: "Rust 学习之旅",
  description: "Rust 交互式教程",
  lang: "zh-CN",
  srcExclude: ["README.md", "docs/**"],
  head: [
    [
      "script",
      {},
      `window.RUST_PLAYGROUND = { evaluateUrl: "http://localhost:9001/evaluate.json" };`
    ],
    ["link", { rel: "icon", href: "/images/favicon.svg" }]
  ],
  themeConfig: {
    nav: [{ text: "首页", link: "/" }],
    sidebar,
    outline: { label: "本页目录" },
    docFooter: { prev: "上一篇", next: "下一篇" },
    darkModeSwitchLabel: "主题",
    sidebarMenuLabel: "目录",
    returnToTopLabel: "回到顶部",
    search: { provider: "local" }
  }
});
