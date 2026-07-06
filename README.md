# Rust 学习之旅

一个前后端分离的 Rust 交互式学习网站，前端使用 **Vue 3** 管理状态，后端使用 **Rust + Axum + Docker** 本地编译执行 Rust 代码，无需访问外网 CDN 或 Rust Playground。

![Rust 学习之旅](https://img.shields.io/badge/Vue-3-4FC08D?logo=vue.js&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-blue.svg)

## 特性

- 📚 **三大模块，22 个章节**：
  - **基础入门**：Hello World、变量与可变性、数据类型、函数、控制流、所有权、结构体、枚举与模式匹配、错误处理、集合
  - **中等应用**：泛型、Trait、生命周期、闭包与迭代器、智能指针、并发基础
  - **高级应用**：unsafe Rust、宏编程、异步编程、Web 开发基础、数据库与序列化、项目工程化
- 📝 **在线编辑器**：基于 CodeMirror 6，支持 Rust 语法高亮与主题切换
- ▶️ **一键运行**：后端通过 Docker 容器本地编译执行 Rust 代码
- 🌙 **深色/浅色主题**：自动跟随系统偏好，支持手动切换并持久化
- 📱 **响应式布局**：适配桌面端与移动端，左侧边栏按模块分组
- ⌨️ **快捷键支持**：`Ctrl/Cmd + Enter` 快速运行代码
- 🔒 **Docker 沙箱隔离**：每次运行独立的 Docker 容器，限制网络、权限、内存与 CPU
- 🚀 **无外部 CDN 依赖**：Vue、CodeMirror、Tailwind CSS、Marked.js 全部本地化
- ⚡ **Vue 3 驱动**：使用 Vue 3 Composition API 管理响应式状态与模板渲染

## 在线体验

启动后端服务后，直接在浏览器中打开对应地址即可使用。

## 快速开始

### 1. 环境要求

- [Rust](https://www.rust-lang.org/)（用于构建后端）
- [Docker](https://www.docker.com/)（用于沙箱编译运行 Rust 代码）
- 已预装 Rust 镜像：`rust:1.79-slim`（首次运行会自动拉取）

### 2. 构建后端

```bash
cd /Users/yanshili/me/projects/rust-projects/backend
cargo build --release
```

### 3. 启动后端服务

```bash
# 默认监听 0.0.0.0:3000，并托管前端静态文件
STATIC_DIR=/Users/yanshili/me/projects/rust-projects ./target/release/rust-learning-backend
```

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

后端会同时提供：

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
│   ├── codemirror-bundle.js  # 本地构建的 CodeMirror 6 bundle
│   ├── tailwindcss.js        # 本地缓存的 Tailwind CSS Play CDN
│   ├── vue.esm-browser.prod.js  # Vue 3 生产构建
│   └── marked.min.js         # Marked.js 本地副本
├── backend/                  # Rust + Axum 后端服务
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── codemirror-entry.js       # CodeMirror 打包入口（构建用）
├── tailwind-input.css        # Tailwind 打包输入（构建用）
├── tailwind.config.js        # Tailwind 配置（构建用）
├── package.json              # 仅用于安装构建依赖
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

## 依赖来源说明

为了支持完全离线和无外网访问的部署环境，所有前端依赖均已本地化：

- **Vue 3**：`libs/vue.esm-browser.prod.js`，通过 import map 映射为 `vue`
- **Tailwind CSS**：`libs/tailwindcss.js`，本地缓存的 Play CDN 构建，已移除生产环境警告
- **CodeMirror 6**：`libs/codemirror-bundle.js`，本地 esbuild 打包产物
- **Marked.js**：`libs/marked.min.js`，本地副本

Rust 代码执行不再依赖 `https://play.rust-lang.org/evaluate.json`，而是由后端通过 Docker 容器在本地编译运行。

README 徽章仍使用 `https://img.shields.io`，但仅文档展示，不影响功能。

## 章节数据结构

`js/chapters.json` 是一个 JSON 文件，包含模块数组，每个模块包含若干章节：

```json
[
  {
    "name": "模块名称",
    "chapters": [
      {
        "title": "章节标题",
        "theory": "# Markdown 理论内容",
        "code": "fn main() {\n    println!(\"示例代码\");\n}",
        "hint": "可选的练习提示"
      }
    ]
  }
]
```

字段说明：

- `title`：章节标题，显示在左侧目录中
- `theory`：Markdown 格式的理论讲解内容
- `code`：默认加载到编辑器中的 Rust 示例代码
- `hint`：可选的提示信息，以提示框形式展示在理论区底部

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

## 可用的外部 crate

由于代码在本地 Docker 容器中使用 `cargo build` 编译，标准库的 crate 可直接使用。如果需要额外 crate（如 `tokio`、`serde`、`hyper` 等），需要修改后端生成的 `Cargo.toml` 模板，添加对应依赖。

当前 `Cargo.toml` 模板位于 `backend/src/main.rs` 的 `EvaluateRequest` 处理函数中，默认只包含 `edition` 配置。你可以根据教学需要扩展默认依赖，或在前端请求中增加 `dependencies` 字段并让后端动态写入。

## 添加新依赖

### 前端依赖

如需引入新的前端库，请将 JS/CSS 文件下载到 `libs/` 目录，然后在 `index.html` 或 `js/app.js` 中引用本地路径。

### 后端 Rust 依赖

编辑 `backend/src/main.rs` 中生成 `Cargo.toml` 的代码，在模板里加入需要的 crate。例如：

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

重新构建后端：

```bash
cd backend
cargo build --release
```

涉及本地构建产物的前端依赖（CodeMirror、Tailwind CSS）如需更新，请参考下方「本地构建依赖」章节。

## 本地构建依赖

如果你需要重新生成前端本地依赖文件：

### 重新构建 CodeMirror bundle

```bash
# 1. 安装打包依赖（仅首次）
npm install --no-save esbuild codemirror @codemirror/lang-rust @codemirror/theme-one-dark @codemirror/view

# 2. 执行打包
npx esbuild codemirror-entry.js --bundle --format=esm --platform=browser --target=es2022 --minify --outfile=libs/codemirror-bundle.js
```

`codemirror-entry.js` 内容如下：

```js
export { EditorView, basicSetup } from "codemirror";
export { rust } from "@codemirror/lang-rust";
export { oneDark } from "@codemirror/theme-one-dark";
export { keymap } from "@codemirror/view";
```

### 重新生成 Tailwind CSS 本地文件

直接下载官方 Play CDN 并替换 `libs/tailwindcss.js`，然后移除文件中的 `console.warn("cdn.tailwindcss.com should not be used in production...")` 提示：

```bash
curl -L https://cdn.tailwindcss.com/3.4.10 -o libs/tailwindcss.js
```

### 重新下载 Vue 3 和 Marked.js

```bash
curl -L https://unpkg.com/vue@3.4.21/dist/vue.esm-browser.prod.js -o libs/vue.esm-browser.prod.js
curl -L https://cdn.jsdelivr.net/npm/marked@13.0.2/marked.min.js -o libs/marked.min.js
```

> 上述下载命令需要外网。如果你的构建环境无外网，请在可访问外网的机器上下载后复制到 `libs/` 目录。

## 部署方式

### 推荐部署方式

由于后端需要 Docker 和 Rust 编译环境，推荐在具备 Docker 的服务器上部署：

1. 安装 Rust、Docker
2. 拉取 Rust Docker 镜像：`docker pull rust:1.79-slim`
3. 构建后端：`cd backend && cargo build --release`
4. 启动后端：`STATIC_DIR=/path/to/project ./target/release/rust-learning-backend`
5. 使用 Nginx / Caddy 反向代理到后端端口

### 前端静态文件部署

如果只需要部署前端而不需要单独运行后端，也可以将前端文件部署到任意静态托管服务（GitHub Pages、Vercel、Netlify 等），然后将 API 地址指向独立部署的后端。此时需要修改 `js/app.js` 中的 `/evaluate.json` 为完整后端地址。

### Docker Compose 部署（可选）

可以进一步将后端服务本身也容器化，与 Docker socket 挂载配合使用。

## 注意事项

- 后端服务需要 Docker 守护进程正常运行，并能够拉取/运行 `rust:1.79-slim` 镜像。
- 首次运行某个 Rust 代码时，`cargo build` 需要下载依赖，可能会比较慢。建议提前准备包含常用 crate 的自定义镜像，或在镜像中预编译依赖。
- 示例代码均已控制在数秒内完成；过长或无限循环会被 Docker 超时机制终止。
- 移动端浏览器中，侧边栏可通过顶部菜单按钮展开/收起。
- 部分高级章节（如 Web 服务）受沙箱环境限制无法真正监听端口，示例仅用于展示 API 和验证编译。
- 主题设置会保存在浏览器 `localStorage` 中，刷新页面后仍然生效。
- Docker 容器每次运行都会重新创建临时目录，不会保留任何文件或状态。

## 未来计划

- [ ] 增加章节进度记忆功能
- [ ] 支持代码本地自动保存与恢复
- [ ] 增加更多高级章节（如 FFI、WASM、嵌入式等）
- [ ] 提供 PWA 离线访问能力
- [ ] 增加章节练习与答案验证

## 贡献

欢迎提交 Issue 或 Pull Request 来完善课程内容或修复问题。请保持章节数据 `chapters.json` 格式一致，并在修改后通过本地服务器验证效果。

## 许可证

MIT

Copyright (c) 2026 Rust 学习之旅贡献者
