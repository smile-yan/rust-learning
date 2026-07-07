# Rust 学习之旅

一个前后端分离的 Rust 交互式学习网站。前端使用 **Vue 3** 管理状态，后端使用 **Rust + Axum + Docker** 在本地编译执行 Rust 代码，无需访问外网 CDN 或 Rust Playground。

![Vue](https://img.shields.io/badge/Vue-3-4FC08D?logo=vue.js&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-blue.svg)

## 特性

- 📚 **四大模块，53 个章节**：基础入门、中等应用、高级应用、Q \u0026 A 常见问题
- 📝 **在线编辑器**：基于 CodeMirror 6，支持 Rust 语法高亮、行号、括号匹配、自动补全
- ▶️ **一键运行**：后端通过 Docker 容器本地编译执行 Rust 代码
- 🌙 **深色/浅色主题**：自动跟随系统偏好，支持手动切换并持久化
- 📱 **响应式布局**：适配桌面端与移动端，左侧边栏按模块分组
- ⌨️ **快捷键支持**：`Ctrl/Cmd + Enter` 快速运行代码
- 🔒 **Docker 沙箱隔离**：每次运行独立的 Docker 容器，限制网络、权限、内存与 CPU
- 🚀 **无外部 CDN 依赖**：Vue、CodeMirror、Tailwind CSS、Marked.js 全部本地化
- ⚡ **Vue 3 Composition API**：响应式状态管理与模板渲染

## 前后端分离架构

本项目采用**前后端分离**架构：

- **前端**：纯静态 HTML + Vue 3 + CodeMirror 6，不依赖构建工具（如 Vite/Webpack），直接由浏览器加载运行。
- **后端**：Rust + Axum 提供 HTTP 服务，主要暴露 `POST /evaluate.json` 接口用于编译运行 Rust 代码，同时可作为静态文件服务器托管前端。
- **通信**：前端通过 `fetch` 向后端 `/evaluate.json` 发送代码，后端在 Docker 容器中编译运行后返回结果。

### 接口约定

前端期望后端提供：

| 方法 | 路径 | 说明 |
|------|------|------|
| `GET` | `/` | 返回 `index.html` |
| `GET` | `/js/*`、`/css/*`、`/libs/*` 等 | 返回静态资源 |
| `POST` | `/evaluate.json` | 接收 Rust 代码，返回运行结果 |

`/evaluate.json` 请求体示例：

```json
{
  "version": "stable",
  "edition": "2021",
  "crateType": "bin",
  "mode": "debug",
  "tests": false,
  "optimize": "0",
  "code": "fn main() { println!(\"Hello\"); }"
}
```

`/evaluate.json` 响应体示例：

```json
{
  "result": "Hello\n",
  "error": ""
}
```

## 环境要求

- [Rust](https://www.rust-lang.org/)（用于构建后端）
- [Docker](https://www.docker.com/)（用于沙箱编译运行 Rust 代码）
- [Node.js](https://nodejs.org/)（仅当需要重新打包 CodeMirror bundle 时）
- 已预装 Rust 镜像：`rust:1.79-slim`（首次运行会自动拉取）

## 快速开始（前后端一体）

最简单的运行方式是让后端同时托管前端静态文件：

```bash
# 1. 构建后端
cd backend
cargo build --release

# 2. 在前端项目根目录启动后端（STATIC_DIR 默认为 ../，即项目根目录）
cd ..
./backend/target/release/rust-learning-backend
```

打开浏览器访问：

```
http://localhost:3000
```

后端默认监听 `0.0.0.0:3000`，同时提供前端资源和 `/evaluate.json` 接口。

## 前后端分离部署

如果你希望前端和后端分别部署（例如前端用 Nginx，后端单独运行），按以下步骤操作。

### 1. 前端部署

前端是纯静态资源，可直接部署到任意静态文件服务器。

#### 方式一：Nginx

将项目根目录（包含 `index.html`、`css/`、`js/`、`libs/`）复制到 Nginx 的站点目录，例如 `/var/www/rust-learning`：

```nginx
server {
    listen 80;
    server_name rust-learning.example.com;
    root /var/www/rust-learning;
    index index.html;

    location / {
        try_files $uri $uri/ /index.html;
    }

    location /evaluate.json {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

#### 方式二：Caddy

将项目根目录复制到 Caddy 的站点目录，例如 `/var/www/rust-learning`，然后创建 `/etc/caddy/Caddyfile`：

```caddyfile
rust-learning.example.com {
    root * /var/www/rust-learning
    file_server
    try_files {path} /index.html

    handle /evaluate.json {
        reverse_proxy localhost:3000
    }
}
```

重载 Caddy：

```bash
sudo systemctl reload caddy
```

Caddy 会自动处理 HTTPS（如果域名可解析），配置比 Nginx 更简洁。

#### 方式三：Python 临时服务器（开发测试）

```bash
# 在项目根目录运行
python3 -m http.server 8080
```

然后访问 `http://localhost:8080`。

#### 方式四：任意静态托管

GitHub Pages、Vercel、Netlify、对象存储 CDN 等均可直接部署。

### 2. 后端部署

```bash
cd backend
cargo build --release

# 启动后端，不托管前端静态文件或托管到另一个目录
PORT=3000 ./target/release/rust-learning-backend
```

如果后端不托管前端，确保前端能通过同源或 CORS 访问到 `/evaluate.json`。后端已默认开启 `CorsLayer::permissive()`，允许跨域请求。

### 3. 配置前端 API 地址

默认情况下，前端代码中调用的是同源的 `/evaluate.json`：

```js
const res = await fetch("/evaluate.json", { ... });
```

如果后端部署在不同域名或端口，需要修改 `js/app.js` 中的请求地址。

#### 步骤

1. 打开 `js/app.js`。
2. 找到 `runCode` 函数中的 `fetch` 调用：

```js
const res = await fetch("/evaluate.json", {
  method: "POST",
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify({ ... }),
  signal: controller.signal
});
```

3. 将 `/evaluate.json` 替换为完整后端地址，例如：

```js
const res = await fetch("http://localhost:3000/evaluate.json", {
  // ...
});
```

或部署到公网时：

```js
const res = await fetch("https://api.rust-learning.example.com/evaluate.json", {
  // ...
});
```

#### 注意事项

- 如果前端和后端部署在**相同域名和端口**，保持 `/evaluate.json` 即可。
- 如果部署在**不同域名或端口**，后端已默认开启 `CorsLayer::permissive()`，允许跨域请求。
- 修改完成后，建议同步更新 `index.html` 中 `js/app.js` 的版本参数（如 `?v=4`），避免浏览器缓存旧代码：

```html
<script type="module" src="js/app.js?v=4"></script>
```

- 如果通过 Nginx/Caddy 反向代理 `/evaluate.json`，则不需要修改前端代码，保持 `/evaluate.json` 即可。

## 环境变量

| 变量 | 默认值 | 说明 |
|------|--------|------|
| `PORT` | `3000` | 后端监听端口 |
| `STATIC_DIR` | `../` | 前端静态文件目录（相对 backend 目录） |
| `CONCURRENCY` | `4` | 最大并发编译任务数 |
| `TIMEOUT_SECONDS` | `25` | 单次编译运行超时时间 |
| `MEMORY_LIMIT_MB` | `256` | Docker 容器内存限制 |
| `DOCKER_IMAGE` | `rust:1.79-slim` | 编译运行使用的 Docker 镜像 |

## 本地构建与打包

### 构建后端

```bash
cd backend
cargo build --release
```

产物位于 `backend/target/release/rust-learning-backend`。

### 重新打包 CodeMirror bundle

如果你修改了 `codemirror-entry.js` 或需要更新 CodeMirror 版本：

```bash
# 项目根目录
npm install
npm run build:codemirror
```

生成的 `libs/codemirror-bundle.js` 已包含完整的 CodeMirror 6 运行时、Rust 语言支持、oneDark 主题，以及自定义高亮所需的 `syntaxHighlighting`、`HighlightStyle`、`tags`。

`codemirror-entry.js` 内容如下：

```js
export { EditorView, keymap } from "@codemirror/view";
export { basicSetup } from "codemirror";
export { oneDark } from "@codemirror/theme-one-dark";
export { rust } from "@codemirror/lang-rust";
export { syntaxHighlighting, HighlightStyle } from "@codemirror/language";
export { tags } from "@lezer/highlight";
```

### 前端是否需要打包？

**不需要**。本项目前端不依赖 Vite、Webpack 等构建工具，浏览器直接加载 `index.html` 中的原生 ES Module。部署时只需将静态文件上传即可。

### 完整的生产部署示例

假设项目根目录为 `/opt/rust-learning`：

```bash
# 1. 构建后端
cd /opt/rust-learning/backend
cargo build --release

# 2. 可选：重新打包前端依赖
cd /opt/rust-learning
npm install
npm run build:codemirror

# 3. 启动后端（托管前端）
STATIC_DIR=/opt/rust-learning /opt/rust-learning/backend/target/release/rust-learning-backend
```

或使用 systemd 服务管理后端进程。

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

## 运行原理

点击「运行」后，前端会将编辑器中的代码通过 `fetch` POST 到 `/evaluate.json`。

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
