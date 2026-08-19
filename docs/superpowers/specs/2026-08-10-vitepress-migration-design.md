# VitePress 迁移设计：章节内容从 JS 打包改为独立 md → 独立 html 页面

日期：2026-08-10
状态：已获用户批准（方案 A）

## 背景与目标

当前站点（`rust-learning`）的全部章节内容存放在 `js/chapters.json`（5 个模块、73 章，字段：`title` / `theory` / `code` / `hint` / `exercises[]`），构建时被整个打包进 JS bundle，运行时由 `app.html` + Vue SPA 动态渲染。用户不满意「内容写在 js 里」这一形态。

目标：

1. 每章内容是一个独立的 `.md` 文件（md 是唯一数据源，`chapters.json` 废弃删除）。
2. 发布后每章对应一个独立的 `.html` 页面。
3. 站点目录（侧边栏）由框架生成；每章的**正文、初始代码、课后练习**全部放在同一个 md 文件里。
4. 每个章节页保留交互能力：CodeMirror 在线编辑器、运行按钮、练习一键载入编辑器。

## 方案选型

采用 **VitePress**（方案 A）：

- md → 独立 html、侧边栏目录、上一篇/下一篇导航、深色/浅色主题切换均为内建能力。
- 支持在 md 中嵌入 Vue 组件，CodeMirror 编辑器封装为 `<RustPlayground />` 组件嵌入每页。
- 明确放弃：现有自定义封面页样式、Tailwind Play CDN、Prism（由 VitePress 内建 Shiki 高亮替代）。外观切换为 VitePress 主题体系，用户已确认接受。

备选方案 B（自建 Node 构建脚本保留现有外观）与方案 C（保留 SPA 运行时加载 md）已在讨论中排除：B 是重复造 SSG，C 不满足「多个独立 html 页面」的要求。

## md 文件格式

每章一个文件，front matter 承载结构化数据，正文为现有 `theory` 原文：

```markdown
---
title: 常量与变量 let / let mut
module: 基础入门
order: 2
code: |
  fn main() {
      let x = 5;
      // ...
  }
hint: 注意不可变变量与可变变量的区别……
exercises:
  - title: 累加器
    description: 声明一个可变的 counter，循环 5 次每次加 1，最后打印结果。
    code_template: |
      fn main() {
          let mut counter = 0;
          // 使用 for 循环累加
      }
---

（正文 = 现有 theory 的 markdown 原文，含 `images/*.svg` 插图引用）

<RustPlayground />
```

约定：

- `title`、`module` 必填；`order` 为模块内序号（从 1 开始），用于侧边栏排序与上一篇/下一篇导航。
- `code` / `hint` / `exercises` 可选（Q & A 类章节可能没有练习）。
- `<RustPlayground />` 仅在存在 `code` 或 `exercises` 时出现，位于正文末尾；组件通过 `useData()` 读取本页 front matter。
- 正文中的图片路径统一为 `/images/xxx.svg`（VitePress `public/` 目录原样拷贝到产物根）。

## 目录结构

```
├── .vitepress/
│   ├── config.mts                     # 站点配置、5 个模块的侧边栏、导航
│   ├── theme/
│   │   ├── index.ts                   # 自定义主题：注册 RustPlayground 组件
│   │   └── components/
│   │       └── RustPlayground.vue     # CodeMirror 编辑器 + 运行 + 练习载入
│   └── dist/                          # 构建产物（部署上传此目录）
├── chapters/
│   ├── basic/01-hello-world.md        # 基础入门（17 章）
│   ├── intermediate/...               # 中等应用（20 章）
│   ├── advanced/...                   # 高级应用（11 章）
│   ├── philosophy/...                 # Rust 哲学（7 章）
│   └── qa/...                         # Q & A（18 章）
├── index.md                           # 首页（VitePress home/hero 布局）
├── public/images/                     # SVG 插图，沿用现有 public/images 内容
├── scripts/
│   ├── migrate-to-md.mjs              # 一次性迁移脚本：chapters.json → 73 个 md
│   └── deploy-frontend.sh             # 仅修改产物路径
└── package.json
```

文件命名：`chapters/<module-slug>/<order 两位数>-<title-slug>.md`，slug 用英文短名（如 `01-hello-world.md`），保证 URL 稳定且与中文标题解耦。

模块 slug 映射：`基础入门 → basic`、`中等应用 → intermediate`、`高级应用 → advanced`、`Rust 哲学 → philosophy`、`Q & A → qa`。

## 核心组件：RustPlayground.vue

- 基于现有 `src/app.js` 中的 CodeMirror 6 配置迁移：Rust 语法高亮（`@codemirror/lang-rust`）、行号、括号匹配、oneDark 主题跟随站点明暗、`Ctrl/Cmd + Enter` 运行。
- 从 `useData().frontmatter.value` 读取 `code`（初始载入编辑器）、`hint`（提示按钮）、`exercises`（渲染练习列表，点击将 `code_template` 载入编辑器）。
- 运行按钮 POST 到 `evaluateUrl`，接口格式与现在一致（`window.RUST_PLAYGROUND.evaluateUrl`，默认 `http://localhost:9001/evaluate.json`，部署时由 CI 注入生产地址）。配置注入点从 `app.html` 迁移到 `.vitepress/config.mts` 的 `head` 中输出内联脚本。
- 每章页面各自挂载独立编辑器实例，互不影响。

## 站点配置（config.mts）

- `title` / `description` 沿用「Rust 学习之旅」。
- `srcExclude` 排除 README 等；`base` 保持根路径。
- 侧边栏：5 个模块分组，每组按 `order` 列出章节链接；由迁移脚本生成一份 `sidebar` 数据（`.vitepress/sidebar.ts`），config 引入。
- 开启 `lastUpdated`、上一篇/下一篇导航（VitePress 默认基于侧边栏顺序）。

## 迁移流程

1. 新增 `scripts/migrate-to-md.mjs`：读取 `js/chapters.json`，按上述格式生成 73 个 md 与 `.vitepress/sidebar.ts`；正文中的 `images/xxx.svg` 相对路径改写为 `/images/xxx.svg`。
2. 运行脚本生成内容，人工抽查若干章节渲染效果。
3. 删除：`js/chapters.json`、`app.html`、`index.html`、`src/`（app.js、style.css）、`vite.config.js`、`scripts/validate_chapters.py`（被 md 格式取代）。
4. `public/` 仅保留 `images/`（Prism、Tailwind 本地化文件删除）。
5. `package.json`：移除 `@vitejs/plugin-vue`、Prism 相关残留；新增 `vitepress`（devDependency）；scripts 改为：
   - `dev`: `vitepress dev`
   - `build`: `vitepress build`
   - `preview`: `vitepress preview`
6. `scripts/deploy-frontend.sh`：产物路径 `dist/` → `.vitepress/dist/`；evaluateUrl 注入目标从 `app.html` 改为 `.vitepress/config.mts` 生成的 head 配置（构建前由 CI 写入环境变量）。
7. 更新 `README.md`（项目结构、本地预览、部署章节）。

## 验证

- `npm run build` 成功，产物中包含 73 个章节 html（`chapters/basic/01-hello-world.html` 等）与首页 `index.html`。
- `npm run dev` / `preview` 下抽查至少 3 个不同模块的章节页：正文渲染、SVG 插图显示、代码块高亮、编辑器载入初始代码、练习按钮载入模板、运行按钮调用 evaluateUrl（本地无后端时预期报网络错误但请求发出）。
- 侧边栏 5 个模块分组正确、顺序正确；上一篇/下一篇导航工作。
- 深色/浅色切换下编辑器主题跟随。

## 非目标（YAGNI）

- 不保留原封面页与 Tailwind 自定义样式。
- 不做全文搜索增强（用 VitePress 内建本地搜索即可，如默认未开再议）。
- 不做 md 内容校验脚本（front matter 由迁移脚本生成，后续人工维护）。
- 不改动后端 rust-playground-backend。
