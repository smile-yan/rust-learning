# VitePress 迁移实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 把章节内容从打包进 JS 的 `chapters.json` 迁移为「每章一个 md 文件 → 构建后每章一个独立 html 页面」的 VitePress 站点，每页保留 CodeMirror 在线编辑器与练习功能。

**Architecture:** VitePress 静态站点生成器。每章 md 的 front matter 承载 `title/module/order/code/hint/exercises`，正文为原 `theory` markdown；自定义 `<RustPlayground />` Vue 组件（CodeMirror 6）通过 `useData()` 读取 front matter 渲染编辑器。一次性脚本 `scripts/migrate-to-md.mjs` 从 `js/chapters.json` 生成 73 个 md 与侧边栏配置，生成后旧 SPA 文件全部删除。

**Tech Stack:** VitePress 1.x、Vue 3、CodeMirror 6（沿用现有依赖）、Node 内置 `node --test`（迁移脚本测试，不新增测试框架）。

**Spec:** `docs/superpowers/specs/2026-08-10-vitepress-migration-design.md`

## Global Constraints

- md 是唯一数据源；迁移完成后删除 `js/chapters.json`，不再有 JSON 数据文件。
- 模块 slug 映射（固定）：`基础入门 → basic`、`中等应用 → intermediate`、`高级应用 → advanced`、`Rust 哲学 → philosophy`、`Q & A → qa`。
- 章节文件名：`chapters/<module-slug>/<order 两位数>-<title-slug>.md`，order 从 1 开始。
- 正文图片路径统一改写为 `/images/xxx.svg`（绝对路径，`public/` 原样拷贝到产物根）。
- 不引入 Tailwind、Prism；代码高亮用 VitePress 内建 Shiki；样式走组件 scoped CSS。
- 构建产物目录为 `.vitepress/dist/`。
- 运行接口注入约定不变：`window.RUST_PLAYGROUND.evaluateUrl`，默认 `http://localhost:9001/evaluate.json`。
- 测试只用 Node 内置 `node:test`，不新增 vitest/jest 等依赖。
- 项目根目录：`/Users/yanshili/me/projects/rust-projects`（下文所有相对路径相对此目录）。

---

### Task 1: VitePress 骨架（依赖、config、首页、示例章节）

**Files:**
- Modify: `package.json`
- Delete: `vite.config.js`
- Create: `.vitepress/config.mts`
- Create: `.vitepress/sidebar.ts`（临时手写，Task 3 由脚本重新生成）
- Create: `index.md`
- Create: `chapters/basic/01-hello-world.md`（手工示例，Task 3 会被脚本产物覆盖）
- Modify: `.gitignore`

**Interfaces:**
- Produces: `.vitepress/sidebar.ts` 导出 `export const sidebar: SidebarItem[]`（VitePress sidebar 配置数组），Task 3 的迁移脚本会重写此文件但保持同一导出名；`chapters/<slug>/<file>.md` 的 front matter 字段集（`title/module/order/code/hint/exercises`）。

- [ ] **Step 1: 删除 `vite.config.js`，更新 `package.json`**

删除 `vite.config.js`（VitePress 不使用它，残留会干扰构建）。

`package.json` 改为：

```json
{
  "name": "rust-learning",
  "version": "0.3.0",
  "description": "Rust 学习之旅 · 交互式教程",
  "type": "module",
  "scripts": {
    "dev": "vitepress dev",
    "build": "vitepress build",
    "preview": "vitepress preview",
    "test": "node --test scripts/"
  },
  "dependencies": {
    "vue": "^3.5.40",
    "codemirror": "^6.0.2",
    "@codemirror/view": "^6.43.6",
    "@codemirror/state": "^6.7.1",
    "@codemirror/language": "^6.12.4",
    "@codemirror/lang-rust": "^6.0.2",
    "@codemirror/theme-one-dark": "^6.1.3",
    "@codemirror/commands": "^6.10.4",
    "@lezer/highlight": "^1.2.3"
  },
  "devDependencies": {
    "vitepress": "^1.6.3"
  }
}
```

（移除 `@vitejs/plugin-vue`、`marked`、`vite`；新增 `vitepress`。）

- [ ] **Step 2: 安装依赖**

Run: `npm install`
Expected: 成功，`node_modules/.bin/vitepress` 存在。

- [ ] **Step 3: 创建 `.vitepress/config.mts`**

```ts
import { defineConfig } from "vitepress";
import { sidebar } from "./sidebar";

export default defineConfig({
  title: "Rust 学习之旅",
  description: "Rust 交互式教程",
  lang: "zh-CN",
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
```

- [ ] **Step 4: 创建临时 `.vitepress/sidebar.ts`**

```ts
export const sidebar = [
  {
    text: "基础入门",
    items: [{ text: "Hello World println!", link: "/chapters/basic/01-hello-world" }]
  }
];
```

- [ ] **Step 5: 创建首页 `index.md`**

```markdown
---
layout: home
hero:
  name: Rust 学习之旅
  text: 交互式 Rust 教程
  tagline: 五大模块 73 章，边学边练，浏览器里直接写 Rust
  actions:
    - theme: brand
      text: 开始学习
      link: /chapters/basic/01-hello-world
features:
  - title: 基础入门
    details: 从 Hello World 到所有权，打好 Rust 基础
  - title: 在线练习
    details: 每章内置 CodeMirror 编辑器，一键运行代码
  - title: 课后练习
    details: 每章配有编程练习，点击即可载入编辑器
---
```

- [ ] **Step 6: 创建手工示例章节 `chapters/basic/01-hello-world.md`**

从 `js/chapters.json` 第 1 章拷贝内容，格式如下（这是 Task 3 脚本要生成的目标格式的样板）：

````markdown
---
title: "Hello World println!"
module: "基础入门"
order: 1
code: |
  fn main() {
      println!("Hello, world!");
  }
hint: "尝试修改 name 的值，或在 println! 中新增一个变量。"
exercises:
  - title: "打印个人信息"
    description: "定义姓名、年龄两个变量，用一行 println! 输出『我叫 XXX，今年 XX 岁』。"
    code_template: |
      fn main() {
          // 在这里定义 name 和 age
      }
---

# Hello World println!

欢迎来到 Rust 学习之旅！（此处从 chapters.json 的 theory 字段完整拷贝正文，图片路径 `](images/` 改写为 `](/images/`）

<RustPlayground />
````

注意：此步 `<RustPlayground />` 组件还不存在，dev 时会渲染为未知组件警告——属于预期，Task 4 解决。若想此步页面完全无警告，可暂时不写最后一行，Task 3 重新生成时补上。

- [ ] **Step 7: 更新 `.gitignore`**

在 `.gitignore` 末尾追加：

```
.vitepress/dist
.vitepress/cache
```

- [ ] **Step 8: 验证 dev 与 build**

Run: `npm run dev`，浏览器打开终端输出地址
Expected: 首页 hero 正常显示；侧边栏出现「基础入门 > Hello World println!」；点击可进入章节页，正文渲染、上一篇/下一篇存在。

Run: `npm run build`
Expected: 成功；`.vitepress/dist/index.html` 与 `.vitepress/dist/chapters/basic/01-hello-world.html` 存在。

- [ ] **Step 9: Commit**

```bash
git add package.json package-lock.json .gitignore .vitepress index.md chapters
git rm vite.config.js
git commit -m "feat: VitePress 骨架（config、首页、示例章节）"
```

---

### Task 2: 迁移脚本 `scripts/migrate-to-md.mjs`（TDD）

**Files:**
- Create: `scripts/migrate-to-md.mjs`
- Test: `scripts/migrate-to-md.test.mjs`

**Interfaces:**
- Consumes: `js/chapters.json`（结构：`[{ name, chapters: [{ title, theory, code, hint, exercises: [{ title, description, code_template }] }] }]`）。
- Produces: 导出 `slugify(title: string): string`、`buildFrontMatter(ch: object, moduleName: string, order: number): string`、`buildMarkdown(ch: object, moduleName: string, order: number): string`、`main(argv: string[]): void`；Task 3 以 `node scripts/migrate-to-md.mjs` 运行，产出 `chapters/**/*.md` 并重写 `.vitepress/sidebar.ts`。

- [ ] **Step 1: 写失败测试 `scripts/migrate-to-md.test.mjs`**

```js
import { test } from "node:test";
import assert from "node:assert/strict";
import { mkdtempSync, readFileSync, existsSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { writeFileSync } from "node:fs";
import {
  slugify,
  buildFrontMatter,
  buildMarkdown,
  main
} from "./migrate-to-md.mjs";

test("slugify 提取 ASCII 词", () => {
  assert.equal(slugify("Hello World println!"), "hello-world-println");
  assert.equal(slugify("常量与变量 let / let mut"), "let-let-mut");
  assert.equal(slugify("Q & A 入门"), "q-and-a");
});

test("slugify 纯中文标题回退为 chapter", () => {
  assert.equal(slugify("所有权"), "chapter");
});

test("buildFrontMatter 生成合法结构", () => {
  const fm = buildFrontMatter(
    {
      title: '常量与变量 let / let mut',
      code: 'fn main() {\n    let x = 5;\n}',
      hint: '注意 mut。',
      exercises: [
        { title: '累加器', description: '循环 5 次。', code_template: 'fn main() {\n}' }
      ]
    },
    "基础入门",
    2
  );
  assert.match(fm, /^---\n/);
  assert.match(fm, /title: "常量与变量 let \/ let mut"\n/);
  assert.match(fm, /module: "基础入门"\n/);
  assert.match(fm, /order: 2\n/);
  // code 块标量：key 后换行，内容每行缩进 2 空格
  assert.match(fm, /code: \|\n  fn main\(\) \{\n      let x = 5;\n  \}\n/);
  // exercises 中 code_template 的缩进深于 key（key 在 4 空格处，内容至少 6 空格）
  assert.match(fm, /    code_template: \|\n      fn main\(\) \{\n      \}\n/);
  assert.match(fm, /\n---\n$/);
});

test("buildFrontMatter 无 code/exercises 时不出现对应字段与组件标记", () => {
  const fm = buildFrontMatter({ title: "Q&A", theory: "x" }, "Q & A", 1);
  assert.ok(!fm.includes("code:"));
  assert.ok(!fm.includes("exercises:"));
});

test("buildMarkdown 改写图片路径并按条件追加组件", () => {
  const withCode = buildMarkdown(
    { title: "t", theory: "\n# t\n\n![图](images/a.svg)\n", code: "fn main() {}" },
    "基础入门",
    1
  );
  assert.ok(withCode.includes("](/images/a.svg)"));
  assert.ok(withCode.trimEnd().endsWith("<RustPlayground />"));

  const noCode = buildMarkdown({ title: "t", theory: "# t\n" }, "Q & A", 1);
  assert.ok(!noCode.includes("RustPlayground"));
});

test("main 端到端：fixture json 生成 md 与 sidebar.ts", () => {
  const dir = mkdtempSync(join(tmpdir(), "migrate-"));
  const fixture = join(dir, "chapters.json");
  writeFileSync(
    fixture,
    JSON.stringify([
      {
        name: "基础入门",
        chapters: [
          {
            title: "Hello World println!",
            theory: "\n# Hello\n\n![图](images/a.svg)\n",
            code: "fn main() {}",
            hint: "h",
            exercises: [
              { title: "练习1", description: "d", code_template: "fn main() {\n}" }
            ]
          }
        ]
      }
    ])
  );
  main([fixture, dir]);
  const mdPath = join(dir, "chapters", "basic", "01-hello-world-println.md");
  assert.ok(existsSync(mdPath));
  const md = readFileSync(mdPath, "utf8");
  assert.ok(md.includes('title: "Hello World println!"'));
  assert.ok(md.includes("](/images/a.svg)"));
  assert.ok(md.includes("<RustPlayground />"));
  const sidebar = readFileSync(join(dir, ".vitepress", "sidebar.ts"), "utf8");
  assert.ok(sidebar.includes("/chapters/basic/01-hello-world-println"));
  assert.ok(sidebar.includes("基础入门"));
});
```

- [ ] **Step 2: 运行测试确认失败**

Run: `node --test scripts/migrate-to-md.test.mjs`
Expected: FAIL（`Cannot find module './migrate-to-md.mjs'`）。

- [ ] **Step 3: 实现 `scripts/migrate-to-md.mjs`**

```js
#!/usr/bin/env node
// 一次性迁移脚本：js/chapters.json -> chapters/**/*.md + .vitepress/sidebar.ts
// 用法：node scripts/migrate-to-md.mjs [jsonPath] [projectRoot]
import { readFileSync, writeFileSync, mkdirSync, rmSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const MODULE_SLUGS = {
  "基础入门": "basic",
  "中等应用": "intermediate",
  "高级应用": "advanced",
  "Rust 哲学": "philosophy",
  "Q & A": "qa"
};

export function slugify(title) {
  const ascii = String(title)
    .toLowerCase()
    .replace(/&/g, " and ")
    .match(/[a-z0-9]+/g);
  return ascii ? ascii.join("-") : "chapter";
}

// 多行文本转为 YAML 块标量内容，每行缩进 spaces 空格（空行不缩进）
function indentBlock(text, spaces) {
  const pad = " ".repeat(spaces);
  return String(text)
    .replace(/\s+$/, "")
    .split("\n")
    .map((line) => (line.trim() ? pad + line : ""))
    .join("\n");
}

// JSON 字符串是合法的 YAML 双引号标量
const yamlStr = (s) => JSON.stringify(String(s));

export function buildFrontMatter(ch, moduleName, order) {
  let out = "---\n";
  out += `title: ${yamlStr(ch.title)}\n`;
  out += `module: ${yamlStr(moduleName)}\n`;
  out += `order: ${order}\n`;
  if (ch.code) {
    out += `code: |\n${indentBlock(ch.code, 2)}\n`;
  }
  if (ch.hint) {
    out += `hint: ${yamlStr(ch.hint)}\n`;
  }
  if (Array.isArray(ch.exercises) && ch.exercises.length > 0) {
    out += "exercises:\n";
    for (const ex of ch.exercises) {
      out += `  - title: ${yamlStr(ex.title)}\n`;
      out += `    description: ${yamlStr(ex.description)}\n`;
      out += `    code_template: |\n${indentBlock(ex.code_template, 6)}\n`;
    }
  }
  out += "---\n";
  return out;
}

export function buildMarkdown(ch, moduleName, order) {
  const body = String(ch.theory || "")
    .trim()
    .replace(/]\(images\//g, "](/images/");
  const hasPlayground = Boolean(ch.code) || (Array.isArray(ch.exercises) && ch.exercises.length > 0);
  let md = buildFrontMatter(ch, moduleName, order) + "\n" + body + "\n";
  if (hasPlayground) {
    md += "\n<RustPlayground />\n";
  }
  return md;
}

export function main(argv = process.argv.slice(2)) {
  const root = dirname(dirname(fileURLToPath(import.meta.url)));
  const jsonPath = argv[0] || join(root, "js", "chapters.json");
  const outRoot = argv[1] || root;

  const modules = JSON.parse(readFileSync(jsonPath, "utf8"));
  const chaptersDir = join(outRoot, "chapters");
  rmSync(chaptersDir, { recursive: true, force: true });

  const sidebar = [];
  let count = 0;
  for (const mod of modules) {
    const slug = MODULE_SLUGS[mod.name];
    if (!slug) throw new Error(`未知模块名: ${mod.name}`);
    const items = [];
    mod.chapters.forEach((ch, i) => {
      const order = i + 1;
      const filename = `${String(order).padStart(2, "0")}-${slugify(ch.title)}.md`;
      const rel = `chapters/${slug}/${filename}`;
      const abs = join(outRoot, rel);
      mkdirSync(dirname(abs), { recursive: true });
      writeFileSync(abs, buildMarkdown(ch, mod.name, order));
      items.push({ text: ch.title, link: `/${rel.replace(/\.md$/, "")}` });
      count++;
    });
    sidebar.push({ text: mod.name, items });
  }

  const sidebarTs =
    "// 本文件由 scripts/migrate-to-md.mjs 生成，请勿手改\n" +
    "export const sidebar = " +
    JSON.stringify(sidebar, null, 2) +
    ";\n";
  mkdirSync(join(outRoot, ".vitepress"), { recursive: true });
  writeFileSync(join(outRoot, ".vitepress", "sidebar.ts"), sidebarTs);

  console.log(`已生成 ${count} 个章节 md 与 .vitepress/sidebar.ts`);
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  main();
}
```

- [ ] **Step 4: 运行测试确认通过**

Run: `node --test scripts/migrate-to-md.test.mjs`
Expected: 5 个测试全部 PASS。

- [ ] **Step 5: Commit**

```bash
git add scripts/migrate-to-md.mjs scripts/migrate-to-md.test.mjs
git commit -m "feat: chapters.json -> md 迁移脚本（含 node:test 测试）"
```

---

### Task 3: 运行迁移，生成 73 个章节 md

**Files:**
- Create: `chapters/**/*.md`（73 个，脚本生成）
- Modify: `.vitepress/sidebar.ts`（脚本重写）
- Delete: `chapters/basic/01-hello-world.md` 手工示例（被脚本 `rmSync` 清目录后重新生成覆盖）

**Interfaces:**
- Consumes: Task 2 的 `main(argv)`；Task 1 的 sidebar 导出约定。
- Produces: 最终内容树；`.vitepress/sidebar.ts`（5 个模块、73 条链接）。

- [ ] **Step 1: 运行迁移脚本**

Run: `node scripts/migrate-to-md.mjs`
Expected: 输出 `已生成 73 个章节 md 与 .vitepress/sidebar.ts`。

- [ ] **Step 2: 校验生成数量与结构**

Run:

```bash
find chapters -name "*.md" | wc -l        # 期望 73
ls chapters                                # 期望 advanced basic intermediate philosophy qa
node --test scripts/                       # 迁移脚本测试仍全部通过
```

Expected: 73；5 个模块目录；测试全绿。

- [ ] **Step 3: 抽查 3 个 md 文件内容**

Run: `head -30 chapters/basic/01-hello-world-println.md`、`head -40 chapters/intermediate/*.md | head -60`、`head -20 chapters/qa/$(ls chapters/qa | head -1)`
Expected: front matter 结构正确（`code: |` 块缩进 2 空格、`code_template: |` 块缩进 6 空格）；正文图片为 `](/images/`；含 code 或 exercises 的文件末尾有 `<RustPlayground />`。

- [ ] **Step 4: 构建验证 73 个独立 html**

Run: `npm run build && find .vitepress/dist/chapters -name "*.html" | wc -l`
Expected: 构建成功；输出 73。确认 `.vitepress/dist/chapters/basic/01-hello-world-println.html` 等存在，`.vitepress/dist/index.html` 存在。

- [ ] **Step 5: dev 抽查侧边栏**

Run: `npm run dev`，浏览器检查
Expected: 侧边栏 5 个模块分组、章节数量与顺序正确（基础入门 17 / 中等应用 20 / 高级应用 11 / Rust 哲学 7 / Q & A 18）；正文渲染、SVG 插图显示、代码块有 Shiki 高亮；上一篇/下一篇导航可用。（`<RustPlayground />` 仍未注册，页面底部有未知组件警告，预期内。）

- [ ] **Step 6: Commit**

```bash
git add chapters .vitepress/sidebar.ts
git commit -m "feat: 迁移 73 个章节为独立 md 文件"
```

---

### Task 4: RustPlayground 组件（CodeMirror 编辑器 + 运行 + 练习）

**Files:**
- Create: `.vitepress/theme/index.ts`
- Create: `.vitepress/theme/components/RustPlayground.vue`

**Interfaces:**
- Consumes: md 页面 front matter 的 `code`（string，初始载入编辑器）、`hint`（string）、`exercises`（`{ title, description, code_template }[]`）；全局 `window.RUST_PLAYGROUND.evaluateUrl`（Task 1 已在 config head 注入）。
- Produces: 全局注册的 `<RustPlayground />` 组件，md 中直接使用；行为与原 `src/app.js` 一致：Rust 高亮、行号、4 空格 Tab/缩进、`Ctrl/Cmd+Enter` 运行、练习一键载入、明暗主题跟随。

- [ ] **Step 1: 创建 `.vitepress/theme/index.ts`**

```ts
import DefaultTheme from "vitepress/theme";
import type { Theme } from "vitepress";
import RustPlayground from "./components/RustPlayground.vue";

export default {
  extends: DefaultTheme,
  enhanceApp({ app }) {
    app.component("RustPlayground", RustPlayground);
  }
} satisfies Theme;
```

- [ ] **Step 2: 创建 `.vitepress/theme/components/RustPlayground.vue`**

逻辑从原 `src/app.js` 迁移（`insertTab`/`unindent`/主题扩展/亮色 HighlightStyle/`runCode`/`renderOutput` 行为保持一致），明暗主题用 VitePress 的 `useData().isDark` 驱动：

```vue
<script setup>
import { ref, computed, onMounted, onBeforeUnmount, watch } from "vue";
import { useData } from "vitepress";
import { EditorView, keymap } from "@codemirror/view";
import { basicSetup } from "codemirror";
import { rust } from "@codemirror/lang-rust";
import { oneDark } from "@codemirror/theme-one-dark";
import { syntaxHighlighting, HighlightStyle, indentUnit } from "@codemirror/language";
import { tags } from "@lezer/highlight";

const { frontmatter, isDark } = useData();

const fm = computed(() => frontmatter.value);
const initialCode = computed(() => fm.value.code || "");
const hint = computed(() => fm.value.hint || "");
const exercises = computed(() =>
  Array.isArray(fm.value.exercises) ? fm.value.exercises : []
);

const editorEl = ref(null);
const outputText = ref("点击「运行」按钮查看输出结果");
const outputClass = ref("output-muted");
const elapsedTime = ref(null);
const hintVisible = ref(false);
let editor = null;

const TAB_INDENT = "    ";

function insertTab(view) {
  const { state, dispatch } = view;
  const selection = state.selection.main;
  if (selection.empty) {
    const insertLen = TAB_INDENT.length;
    dispatch({
      changes: { from: selection.head, insert: TAB_INDENT },
      selection: { anchor: selection.head + insertLen, head: selection.head + insertLen },
      userEvent: "input.indent"
    });
    return true;
  }
  const changes = [];
  let pos = selection.from;
  while (pos <= selection.to) {
    const line = state.doc.lineAt(pos);
    changes.push({ from: line.from, insert: TAB_INDENT });
    pos = line.to + 1;
  }
  dispatch({
    changes,
    selection: selection.map(state.changes(changes)),
    userEvent: "input.indent"
  });
  return true;
}

function unindent(view) {
  const { state, dispatch } = view;
  const selection = state.selection.main;
  const changes = [];
  let pos = selection.from;
  while (pos <= selection.to) {
    const line = state.doc.lineAt(pos);
    let remove = 0;
    while (remove < TAB_INDENT.length && remove < line.text.length && line.text[remove] === " ") {
      remove++;
    }
    if (remove > 0) {
      changes.push({ from: line.from, to: line.from + remove });
    }
    pos = line.to + 1;
  }
  if (changes.length > 0) {
    dispatch({
      changes,
      selection: selection.map(state.changes(changes)),
      userEvent: "input.indent"
    });
  }
  return true;
}

function buildExtensions(dark) {
  const themeExtension = EditorView.theme({
    "&": { height: "100%" },
    ".cm-scroller": { overflow: "auto" },
    ".cm-content": {
      fontFamily: "'JetBrains Mono', 'Fira Code', Consolas, monospace",
      fontSize: "14px",
      backgroundColor: "transparent",
      color: dark ? "#e2e8f0" : "#1e293b"
    },
    ".cm-gutters": {
      backgroundColor: dark ? "#111827" : "#f8fafc",
      color: dark ? "#6b7280" : "#94a3b8",
      borderRight: dark ? "1px solid #374151" : "1px solid #e2e8f0"
    },
    ".cm-activeLineGutter": { backgroundColor: dark ? "#1f2937" : "#e2e8f0" },
    ".cm-activeLine": { backgroundColor: dark ? "#1f2937" : "#f1f5f9" },
    ".cm-selectionBackground": { backgroundColor: dark ? "#2563eb66" : "#bfdbfe" },
    ".cm-cursor": { borderLeftColor: dark ? "#e2e8f0" : "#1e293b" },
    ".cm-lineNumbers": { color: dark ? "#6b7280" : "#94a3b8" }
  });

  const lightHighlight = HighlightStyle.define([
    { tag: tags.keyword, color: "#d73a49" },
    { tag: tags.controlKeyword, color: "#d73a49" },
    { tag: tags.typeName, color: "#6f42c1" },
    { tag: tags.className, color: "#6f42c1" },
    { tag: tags.tagName, color: "#22863a" },
    { tag: tags.name, color: "#24292e" },
    { tag: tags.variableName, color: "#24292e" },
    { tag: tags.string, color: "#032f62" },
    { tag: tags.comment, color: "#6a737d", fontStyle: "italic" },
    { tag: tags.number, color: "#005cc5" },
    { tag: tags.operator, color: "#d73a49" },
    { tag: tags.punctuation, color: "#24292e" },
    { tag: tags.function(tags.variableName), color: "#6f42c1" },
    { tag: tags.propertyName, color: "#005cc5" }
  ]);

  return [
    basicSetup,
    rust(),
    indentUnit.of(TAB_INDENT),
    themeExtension,
    dark ? oneDark : syntaxHighlighting(lightHighlight, { fallback: true }),
    keymap.of([
      { key: "Tab", run: insertTab, preventDefault: true },
      { key: "Shift-Tab", run: unindent, preventDefault: true },
      { key: "Ctrl-Enter", run: () => { runCode(); return true; }, preventDefault: true },
      { key: "Cmd-Enter", run: () => { runCode(); return true; }, preventDefault: true }
    ])
  ];
}

function initEditor(doc) {
  editor = new EditorView({
    doc,
    extensions: buildExtensions(isDark.value),
    parent: editorEl.value
  });
}

async function runCode() {
  if (!editor) return;
  outputText.value = "⏳ 正在编译运行，请稍候...";
  outputClass.value = "output-plain";
  const startTime = performance.now();
  const code = editor.state.doc.toString();
  const evaluateUrl =
    (window.RUST_PLAYGROUND && window.RUST_PLAYGROUND.evaluateUrl) || "/evaluate.json";

  try {
    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), 25000);
    const res = await fetch(evaluateUrl, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        version: "stable",
        edition: "2021",
        crateType: "bin",
        mode: "debug",
        tests: false,
        optimize: "0",
        code
      }),
      signal: controller.signal
    });
    clearTimeout(timeoutId);
    if (!res.ok) throw new Error(`服务器返回 HTTP ${res.status}`);
    renderOutput(await res.json());
  } catch (err) {
    outputText.value =
      err.name === "AbortError"
        ? "❌ 请求超时，请检查网络或稍后重试。"
        : `❌ 运行失败: ${err.message}`;
    outputClass.value = "output-error";
  } finally {
    elapsedTime.value = (performance.now() - startTime) / 1000;
  }
}

function renderOutput(data) {
  const stdout = typeof data.stdout === "string" ? data.stdout : "";
  const stderr = typeof data.stderr === "string" ? data.stderr : "";
  const result = typeof data.result === "string" ? data.result : "";
  const error = data.error !== null && data.error !== undefined ? String(data.error) : "";
  const output = stdout || result;

  if (error.length > 0) {
    outputText.value = error;
    outputClass.value = "output-error";
  } else if (stderr.length > 0 && output.length === 0) {
    outputText.value = stderr;
    outputClass.value = "output-warn";
  } else if (output.length > 0) {
    outputText.value = stderr.length > 0 ? `${output}\n${stderr}` : output;
    outputClass.value = "output-ok";
  } else {
    outputText.value = "（程序没有输出）";
    outputClass.value = "output-plain";
  }
}

function clearOutput() {
  outputText.value = "点击「运行」按钮查看输出结果";
  outputClass.value = "output-muted";
  elapsedTime.value = null;
}

function loadExercise(ex) {
  if (editor && ex && typeof ex.code_template === "string") {
    editor.dispatch({
      changes: { from: 0, to: editor.state.doc.length, insert: ex.code_template }
    });
    clearOutput();
    editorEl.value?.scrollIntoView({ behavior: "smooth", block: "start" });
  }
}

function resetCode() {
  if (editor) {
    editor.dispatch({
      changes: { from: 0, to: editor.state.doc.length, insert: initialCode.value }
    });
    clearOutput();
  }
}

watch(isDark, () => {
  if (editor) {
    const doc = editor.state.doc.toString();
    editor.destroy();
    initEditor(doc);
  }
});

onMounted(() => initEditor(initialCode.value));
onBeforeUnmount(() => editor?.destroy());
</script>

<template>
  <div class="rust-playground">
    <div v-if="exercises.length" class="exercises">
      <div class="section-title">课后练习</div>
      <div v-for="(ex, i) in exercises" :key="i" class="exercise-item">
        <div class="exercise-head">
          <span class="exercise-title">{{ i + 1 }}. {{ ex.title }}</span>
          <button class="btn" @click="loadExercise(ex)">载入编辑器</button>
        </div>
        <div class="exercise-desc">{{ ex.description }}</div>
      </div>
    </div>

    <div class="toolbar">
      <button class="btn btn-primary" @click="runCode">▶ 运行 (Ctrl/Cmd+Enter)</button>
      <button class="btn" @click="resetCode">重置代码</button>
      <button class="btn" @click="clearOutput">清空输出</button>
      <button v-if="hint" class="btn" @click="hintVisible = !hintVisible">提示</button>
      <span v-if="elapsedTime !== null" class="elapsed">{{ elapsedTime.toFixed(2) }}s</span>
    </div>

    <div v-if="hint && hintVisible" class="hint">💡 {{ hint }}</div>

    <div ref="editorEl" class="editor"></div>

    <pre class="output" :class="outputClass">{{ outputText }}</pre>
  </div>
</template>

<style scoped>
.rust-playground {
  margin: 24px 0;
  border: 1px solid var(--vp-c-divider);
  border-radius: 8px;
  overflow: hidden;
}
.section-title {
  font-weight: 600;
  padding: 12px 16px 0;
}
.exercises {
  border-bottom: 1px solid var(--vp-c-divider);
  padding-bottom: 8px;
}
.exercise-item {
  padding: 8px 16px;
}
.exercise-head {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 8px;
}
.exercise-title {
  font-weight: 500;
}
.exercise-desc {
  color: var(--vp-c-text-2);
  font-size: 14px;
  margin-top: 4px;
}
.toolbar {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
  padding: 12px 16px;
}
.btn {
  border: 1px solid var(--vp-c-divider);
  border-radius: 6px;
  padding: 4px 12px;
  font-size: 13px;
  cursor: pointer;
  background: var(--vp-c-bg-soft);
  color: var(--vp-c-text-1);
}
.btn:hover {
  border-color: var(--vp-c-brand-1);
}
.btn-primary {
  background: var(--vp-c-brand-1);
  border-color: var(--vp-c-brand-1);
  color: #fff;
}
.elapsed {
  color: var(--vp-c-text-2);
  font-size: 13px;
  margin-left: auto;
}
.hint {
  margin: 0 16px 12px;
  padding: 8px 12px;
  border-radius: 6px;
  background: var(--vp-c-bg-soft);
  font-size: 14px;
}
.editor {
  height: 320px;
  border-top: 1px solid var(--vp-c-divider);
  border-bottom: 1px solid var(--vp-c-divider);
}
html.dark .editor {
  background: #111827;
}
.output {
  margin: 0;
  padding: 12px 16px;
  font-family: 'JetBrains Mono', 'Fira Code', Consolas, monospace;
  font-size: 13px;
  white-space: pre-wrap;
  min-height: 48px;
}
.output-muted { color: var(--vp-c-text-3); }
.output-plain { color: var(--vp-c-text-1); }
.output-ok { color: #4ade80; }
.output-warn { color: #fb923c; }
.output-error { color: #f87171; }
</style>
```

- [ ] **Step 3: 构建验证**

Run: `npm run build`
Expected: 构建成功（VitePress 会把组件打包进各章节页 chunk）。

- [ ] **Step 4: dev 人工验证**

Run: `npm run dev`，抽查 `chapters/basic/01-hello-world-println`、`chapters/intermediate/` 下任一章、`chapters/qa/` 下任一章：
Expected:
- 含 code 的章节：编辑器载入 front matter 的初始代码、Rust 高亮、行号正常；点击「运行」向 `http://localhost:9001/evaluate.json` 发 POST（本地无后端时显示 `❌ 运行失败: ...`，预期内）。
- 练习「载入编辑器」按钮把 `code_template` 替换进编辑器；「重置代码」恢复初始代码。
- 切换深色/浅色主题，编辑器主题跟随。
- Q & A 中无 code/exercises 的章节不渲染 playground。

- [ ] **Step 5: Commit**

```bash
git add .vitepress/theme
git commit -m "feat: RustPlayground 组件（CodeMirror 编辑器 + 运行 + 练习载入）"
```

---

### Task 5: 清理旧 SPA 文件与部署链路更新

**Files:**
- Delete: `app.html`、`index.html`、`src/`（`app.js`、`style.css`）、`js/`（`chapters.json`）、`scripts/validate_chapters.py`、`public/css/`、`public/libs/`、`public/images/_review.html`
- Modify: `scripts/deploy-frontend.sh`
- Modify: `README.md`

**Interfaces:**
- Consumes: 前面任务的产物目录 `.vitepress/dist/`、config head 中的 evaluateUrl 注入串。
- Produces: 部署脚本上传 `.vitepress/dist/*`；CI 流程（`.github/` 调用 deploy-frontend.sh）不变。

注意：保留 `public/images/` 下所有 `module*.svg` 与 `favicon.svg`（章节与站点图标引用）。`js/` 目录整个删除——`chapters.json` 已迁移为 md，是唯一数据源。

- [ ] **Step 1: 删除旧文件**

```bash
git rm -r app.html index.html src js scripts/validate_chapters.py public/css public/libs
git rm public/images/_review.html
```

（`js/` 删除后，本计划的工作目录 `js/` 即不复存在；此后所有内容维护都在 `chapters/` 的 md 文件中进行。）

- [ ] **Step 2: 确认无残留引用**

Run: `grep -rn "chapters.json\|app.html\|validate_chapters" --include="*.sh" --include="*.yml" --include="*.yaml" --include="*.json" --include="*.md" . | grep -v node_modules | grep -v docs/superpowers`
Expected: 只剩本步骤要改的文件（deploy-frontend.sh、README.md、.github workflow 若有）中的引用；逐一处理。

- [ ] **Step 3: 更新 `scripts/deploy-frontend.sh`**

两处改动：

1. 版本号注入与 evaluateUrl 替换目标从 `dist/app.html`、`dist/index.html` 改为所有构建产物 html：

```bash
# 把版本号与生产环境 evaluateUrl 注入到所有 HTML
find .vitepress/dist -name "*.html" | while read -r f; do
    sed -i.bak \
        -e "s|__VERSION__|$VERSION|g" \
        -e "s|evaluateUrl: \"http://localhost:9001/evaluate.json\"|evaluateUrl: \"$EVALUATE_URL\"|g" \
        "$f"
    rm -f "$f.bak"
done
```

（原第 22–32 行两段 sed 整段替换为上面这段。若新首页不显示版本号，可删去 `__VERSION__` 那一行 sed——VitePress 页面没有该占位符，保留也无害。）

2. 上传目录：

```bash
scp -o ConnectTimeout=30 -o ServerAliveInterval=30 -o ServerAliveCountMax=3 \
    -r .vitepress/dist/* frontend-deploy:"$FRONTEND_WEB_ROOT/"
```

- [ ] **Step 4: 检查 `.github/` workflow 是否需要同步**

Run: `grep -rn "dist\|app.html\|chapters.json" .github/`
Expected: workflow 只调用 `scripts/deploy-frontend.sh` 则无需改动；如有直接引用 `dist/` 的路径，改为 `.vitepress/dist/`。

- [ ] **Step 5: 重写 `README.md`**

用以下内容整体替换 `README.md`：

````markdown
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
│   ├── migrate-to-md.mjs     # （历史）chapters.json -> md 迁移脚本
│   └── deploy-frontend.sh    # 推送标签时触发的部署脚本（CI 调用）
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
````

- [ ] **Step 6: 全量验证**

Run: `npm install && npm run build && find .vitepress/dist -name "*.html" | wc -l`
Expected: 构建成功；74 个 html（73 章 + 首页）。

Run: `npm run preview`，浏览器抽查 3 个不同模块章节页 + 首页
Expected: 全部正常（正文、插图、高亮、编辑器、练习、主题切换、侧边栏、上下篇导航、搜索）。

Run: `node --test scripts/`
Expected: 迁移脚本测试全绿。

- [ ] **Step 7: Commit**

```bash
git add -A
git commit -m "refactor: 移除旧 SPA（app.html/src/chapters.json），更新部署脚本与 README"
```

---

## Self-Review 记录

- Spec 覆盖：md 格式（Task 2/3）、目录结构（Task 1/3）、RustPlayground（Task 4）、config 与 evaluateUrl 注入（Task 1/5）、迁移脚本（Task 2/3）、旧文件清理（Task 5）、README/部署（Task 5）、验证清单（各 Task 末尾 + Task 5 Step 6）。
- 已知取舍：手工示例章节（Task 1 Step 6）会被 Task 3 脚本清目录后重新生成，文件名可能因 slug 规则从 `01-hello-world.md` 变为 `01-hello-world-println.md`；`.vitepress/sidebar.ts` 与首页 action 链接以脚本产物为准——Task 3 Step 1 后若首页 action 链接失效，执行者需同步修改 `index.md` 的 `link: /chapters/basic/01-hello-world-println`。
- 类型一致性：`slugify`/`buildFrontMatter`/`buildMarkdown`/`main` 在测试与实现中签名一致；front matter 字段名（`code`/`hint`/`exercises`/`code_template`）在脚本、组件、README 三处一致。
