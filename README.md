# Rust 学习之旅

一个基于 **VitePress** 的 Rust 交互式学习网站：每章一个 Markdown 文件，构建后每章对应一个独立 HTML 页面；章节页内嵌 **CodeMirror 6** 在线编辑器，可直接在浏览器里编写并运行 Rust 代码。

![License](https://img.shields.io/badge/License-MIT-blue.svg)

欢迎访问 rust 学习 playground: [https://rust.smileyan.cn/](https://rust.smileyan.cn/)

## 特性

- 📚 **五大模块，73 个章节**：基础入门、中等应用、高级应用、Rust 哲学、Q & A 常见问题
- 📄 **每章一个 md 文件**：正文、初始代码、课后练习全部写在该章的 md 里，构建后生成独立 HTML 页面
- 📝 **在线编辑器**：基于 CodeMirror 6，支持 Rust 语法高亮、行号、括号匹配、`Ctrl/Cmd + Enter` 运行（需配合后端接口）
- 🌙 **深色/浅色主题**：VitePress 内建主题切换，编辑器主题自动跟随
- 🧩 **章节练习**：练习一键载入编辑器
- 🔍 **本地全文搜索**：VitePress 内建 local search

## 项目结构

```
.
├── index.md                  # 首页（VitePress home 布局）
├── chapters/                 # 章节内容，每章一个 md（唯一数据源）
│   ├── basic/                # 基础入门（17 章）
│   ├── intermediate/         # 中等应用（20 章）
│   ├── advanced/             # 高级应用（11 章）
│   ├── philosophy/           # Rust 哲学（7 章）
│   └── qa/                   # Q & A（18 章）
├── .vitepress/
│   ├── config.mts            # 站点配置（标题、侧边栏、evaluateUrl 注入）
│   ├── sidebar.ts            # 侧边栏（由 scripts/migrate-to-md.mjs 生成）
│   └── theme/                # 自定义主题与 RustPlayground 组件
├── public/images/            # 章节 SVG 插图与 favicon
├── scripts/
│   ├── migrate-to-md.mjs      # （历史）chapters.json -> md 迁移脚本
│   ├── migrate-to-md.test.mjs # 迁移脚本测试
│   └── deploy-frontend.sh     # 推送标签时触发的部署脚本（CI 调用）
└── package.json
```

### 章节 md 格式

```markdown
---
title: "常量与变量 let / let mut"
module: "基础入门"
order: 2
code: |
  fn main() {
      let x = 5;
  }
hint: "注意 mut 与 shadowing 的区别。"
exercises:
  - title: "累加器"
    description: "声明一个可变的 counter，循环 5 次每次加 1。"
    code_template: |
      fn main() {
          let mut counter = 0;
      }
---

（正文 markdown，图片使用 /images/xxx.svg 绝对路径）

<RustPlayground />
```

新增章节：在对应模块目录下按上述格式新建 md（文件名 `<order两位数>-<slug>.md`），并在 `.vitepress/sidebar.ts` 的对应模块中添加一条 `{ text, link }`。

## 本地预览

```bash
npm install
npm run dev
```

构建与产物预览：

```bash
npm run build      # 产物在 .vitepress/dist/
npm run preview
```

运行迁移脚本的测试（node:test）：

```bash
npm test
```

## 运行 Rust 代码

编辑器中的「运行」按钮调用 `window.RUST_PLAYGROUND.evaluateUrl` 配置的接口执行 Rust 代码（默认 `http://localhost:9001/evaluate.json`，在 `.vitepress/config.mts` 的 head 中配置）。**本仓库只包含前端页面**，在线执行需要搭配后端服务：

- 推荐后端：[smile-yan/rust-playground-backend](https://github.com/smile-yan/rust-playground-backend)
- 或配置为 Rust 官方 Playground 的 `https://play.rust-lang.org/evaluate.json`

## 部署

先执行 `npm run build`，然后将 `.vitepress/dist/` 目录的全部内容上传到任意静态托管服务（Nginx、Caddy、GitHub Pages、Vercel、Netlify 或对象存储 CDN）。

### 自动部署

推送 `v*` 标签时，GitHub Actions 会自动执行 `scripts/deploy-frontend.sh`：安装依赖、构建、注入生产环境 `evaluateUrl`，再把 `.vitepress/dist/` 上传到服务器：

```bash
git tag v0.3.0
git push origin v0.3.0
```

#### 前置条件

1. 在 GitHub 仓库 **Settings → Secrets and variables → Actions** 中配置 Secrets：
   - `SSH_PRIVATE_KEY`：用于登录前端服务器的 SSH 私钥
   - `FRONTEND_HOST`、`FRONTEND_USER`、`FRONTEND_WEB_ROOT`：前端服务器信息
   - `EVALUATE_URL`：生产环境 `/evaluate.json` 完整地址
2. 把 CI 使用的 SSH 公钥添加到前端服务器的 `~/.ssh/authorized_keys`。

## 技术栈

| 名称 | 用途 |
|------|------|
| [VitePress](https://vitepress.dev/) | 静态站点生成（md → 独立 html、侧边栏、搜索、主题） |
| [Vue 3](https://cn.vuejs.org/) | RustPlayground 交互组件 |
| [CodeMirror 6](https://codemirror.net/) | Rust 代码在线编辑器（含 oneDark 主题） |

## 许可证

本项目采用 [MIT 许可证](LICENSE) 开源。

Copyright (c) 2026 Rust 学习之旅贡献者
