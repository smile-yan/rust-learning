# Rust 学习之旅

一个前后端分离的 Rust 交互式学习网站。前端使用 **Vue 3** 管理状态，后端使用 **Rust + Axum + Docker** 在本地编译执行 Rust 代码，无需访问外网 CDN 或 Rust Playground。

![Vue](https://img.shields.io/badge/Vue-3-4FC08D?logo=vue.js&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-blue.svg)

## 特性

- 📚 **四大模块，45 个章节**：基础入门、中等应用、高级应用、Q \u0026 A 常见问题
- 📝 **在线编辑器**：基于 CodeMirror 6，支持 Rust 语法高亮、行号、括号匹配、自动补全
- ▶️ **一键运行**：后端通过 Docker 容器本地编译执行 Rust 代码
- 🌙 **深色/浅色主题**：自动跟随系统偏好，支持手动切换并持久化
- 📱 **响应式布局**：适配桌面端与移动端，左侧边栏按模块分组
- ⌨️ **快捷键支持**：`Ctrl/Cmd + Enter` 快速运行代码
- 🔒 **Docker 沙箱隔离**：每次运行独立的 Docker 容器，限制网络、权限、内存与 CPU
- 🚀 **无外部 CDN 依赖**：Vue、CodeMirror、Tailwind CSS、Marked.js 全部本地化
- ⚡ **Vue 3 Composition API**：响应式状态管理与模板渲染

## 快速开始

### 1. 环境要求

- [Rust](https://www.rust-lang.org/)（用于构建后端）
- [Docker](https://www.docker.com/)（用于沙箱编译运行 Rust 代码）
- 已预装 Rust 镜像：`rust:1.79-slim`（首次运行会自动拉取）

### 2. 构建后端

```bash
cd backend
cargo build --release
```

### 3. 启动后端服务

```bash
# 默认监听 0.0.0.0:3000，并托管前端静态文件
STATIC_DIR=/path/to/project ./target/release/rust-learning-backend
```

> 请将 `/path/to/project` 替换为项目根目录的绝对路径。默认 `STATIC_DIR=../`，因此也可以在前端项目根目录运行 `./backend/target/release/rust-learning-backend`。

环境变量：

| 变量 | 默认值 | 说明 |
|------|--------|------|
| `PORT` | `3000` | 后端监听端口 |
| `STATIC_DIR` | `../` | 前端静态文件目录（相对 backend 目录） |
| `CONCURRENCY` | `4` | 最大并发编译任务数 |
| `TIMEOUT_SECONDS` | `25` | 单次编译运行超时时间 |
| `MEMORY_LIMIT_MB` | `256` | Docker 容器内存限制 |
| `DOCKER_IMAGE` | `rust:1.79-slim` | 编译运行使用的 Docker 镜像 |

### 4. 打开前端

```
http://localhost:3000
```

后端同时提供：

- 前端静态资源：`GET /`
- Rust 编译执行 API：`POST /evaluate.json`

## 项目结构

```
.
├── index.html                # Vue 3 应用挂载点
├── css/
│   └── style.css             # 自定义样式与主题变量
├── js/
│   ├── chapters.json         # 章节数据（JSON 格式）
│   └── app.js                # Vue 3 应用逻辑，调用本地 /evaluate.json
├── libs/
│   ├── codemirror-bundle.js  # 本地 esbuild 打包的 CodeMirror 6 bundle
│   ├── tailwindcss.js        # 本地缓存的 Tailwind CSS Play CDN
│   ├── vue.esm-browser.prod.js  # Vue 3 生产构建
│   └── marked.min.js         # Marked.js 本地副本
├── backend/                  # Rust + Axum 后端服务
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── codemirror-entry.js       # CodeMirror 打包入口（构建用）
├── package.json              # 仅用于安装 CodeMirror 构建依赖
└── README.md
```

## 技术栈

| 名称 | 来源 | 用途 |
|------|------|------|
| [Vue 3](https://cn.vuejs.org/) | 本地 `libs/vue.esm-browser.prod.js` | 响应式 UI 与状态管理（生产构建） |
| [Tailwind CSS](https://www.tailwindcss.cn/) | 本地 `libs/tailwindcss.js` | 原子化 CSS 样式 |
| [CodeMirror 6](https://codemirror.net/) | 本地 `libs/codemirror-bundle.js` | Rust 代码在线编辑器 |
| [Marked.js](https://marked.js.org/) | 本地 `libs/marked.min.js` | Markdown 理论内容渲染 |
| [Axum](https://github.com/tokio-rs/axum) | 后端 Cargo 依赖 | Web 服务与静态文件托管 |
| [Docker](https://www.docker.com/) | 系统依赖 | Rust 代码沙箱编译运行 |

## 本地构建依赖

如果你需要重新生成 `libs/codemirror-bundle.js`：

```bash
# 1. 安装打包依赖（仅首次）
npm install

# 2. 执行打包
npm run build:codemirror
```

`codemirror-entry.js` 导出了前端需要的 CodeMirror 模块：

```js
export { EditorView, keymap } from "@codemirror/view";
export { basicSetup } from "codemirror";
export { oneDark } from "@codemirror/theme-one-dark";
export { rust } from "@codemirror/lang-rust";
export { syntaxHighlighting, HighlightStyle } from "@codemirror/language";
export { tags } from "@lezer/highlight";
```

打包后的 bundle 包含完整的 CodeMirror 6 运行时、Rust 语言支持、oneDark 主题，以及自定义高亮所需的 `syntaxHighlighting`、`HighlightStyle`、`tags`。

## 运行原理

点击「运行」后，前端会将编辑器中的代码通过 `fetch` POST 到同源的 `/evaluate.json`：

```json
{
  "version": "stable",
  "edition": "2021",
  "crateType": "bin",
  "mode": "debug",
  "tests": false,
  "optimize": "0",
  "code": "fn main() { ... }"
}
```

后端接收到请求后：

1. 创建临时目录，写入 `Cargo.toml` 和 `main.rs`
2. 通过 `docker run` 启动隔离容器编译并运行代码
3. 容器限制：无网络、最小权限、256MB 内存、1 CPU、64 进程、25 秒超时
4. 返回 `result`（标准输出）和 `error`（编译/运行错误），前端分别用绿色和红色展示

### Docker 安全配置

```
--network none              # 禁用网络
--cap-drop ALL              # 丢弃所有 Linux capabilities
--security-opt no-new-privileges:true  # 禁止提升权限
--memory 256m               # 内存限制
--memory-swap 256m          # 禁止交换内存
--cpus 1.0                  # CPU 限制
--pids-limit 64             # 进程数限制
```

### 并发控制

后端使用 `tokio::sync::Semaphore` 控制最大并发编译任务数，默认 `4`。超过并发上限的请求返回 `429 Too Many Requests`。

## 注意事项

- 后端服务需要 Docker 守护进程正常运行，并能够拉取/运行 `rust:1.79-slim` 镜像。
- 首次运行某个 Rust 代码时，`cargo build` 需要下载依赖，可能会比较慢。建议提前准备包含常用 crate 的自定义镜像，或在镜像中预编译依赖。
- 示例代码均已控制在数秒内完成；过长或无限循环会被 Docker 超时机制终止。
- 移动端浏览器中，侧边栏可通过顶部菜单按钮展开/收起。
- 主题设置会保存在浏览器 `localStorage` 中，刷新页面后仍然生效。
- 修改 `codemirror-bundle.js` 后，建议同步更新 `js/app.js` 和 `index.html` 中的版本参数（`?v=N`），避免浏览器缓存旧 bundle。

## 许可证

MIT

Copyright (c) 2026 Rust 学习之旅贡献者
