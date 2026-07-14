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
- 自定义 Docker 镜像：`rust-learning-playground:1.86`（包含常用 crate，离线编译，需按下面步骤构建）

## 构建 Docker 编译镜像

后端默认使用自定义镜像 `rust-learning-playground:1.86`，该镜像预下载了 `axum`、`tokio`、`serde`、`serde_json`、`reqwest`、`chrono` 等常用依赖，并内置 `Cargo.lock`。容器运行时通过 `cargo build --offline` 编译，**无需访问外网**，适合无法连接 crates.io 的部署环境。

构建命令：

```bash
cd backend
docker build -f Dockerfile.playground -t rust-learning-playground:1.86 .
```

构建完成后，镜像中会缓存所有依赖的 crate 文件。后续每次执行代码时，Docker 容器直接离线编译，无需联网。

### 为什么要自定义镜像？

- 原 `rust:1.79-slim` 等官方镜像不包含任何第三方 crate，运行时必须联网下载，在封闭网络环境无法使用。
- 本项目示例包含 Web 开发（axum）、序列化（serde）、HTTP 请求（reqwest）等，需要预置依赖。
- `--network none` 的沙箱容器无法联网，因此所有依赖必须在镜像构建阶段缓存好。

### 如何扩展依赖？

如果你需要给示例代码添加更多 crate，按以下步骤操作：

1. 修改 `backend/src/main.rs` 中生成 `Cargo.toml` 的 `[dependencies]` 段落。
2. 同步修改 `backend/Dockerfile.playground` 中的 `[dependencies]`，保持一致。
3. 重新构建镜像：`docker build -f Dockerfile.playground -t rust-learning-playground:1.86 .`
4. 在镜像内生成新的 `Cargo.lock`：

   ```bash
   mkdir -p /tmp/offline-lock
   cat > /tmp/offline-lock/Cargo.toml <<'EOF'
   [package]
   name = "playground"
   version = "0.1.0"
   edition = "2021"

   [[bin]]
   name = "playground"
   path = "main.rs"

   [dependencies]
   # 与 Dockerfile.playground 中一致
   EOF
   echo 'fn main() {}' > /tmp/offline-lock/main.rs
   docker run --rm -v /tmp/offline-lock:/project -w /project \
     rust-learning-playground:1.86 sh -c 'cargo generate-lockfile 2>/dev/null; cat Cargo.lock' \
     > backend/src/Cargo.lock.playground
   ```

5. 重新编译后端：`cd backend && cargo build --release`。

> 注意：`Cargo.lock.playground` 必须与镜像中的依赖版本严格一致，否则 `--offline` 编译会失败。

## 快速开始（前后端一体）

最简单的运行方式是让后端同时托管前端静态文件：

```bash
# 1. 构建 Docker 编译镜像（仅需一次）
cd backend
docker build -f Dockerfile.playground -t rust-learning-playground:1.86 .

# 2. 构建后端
cargo build --release

# 3. 在前端项目根目录启动后端（STATIC_DIR 默认为 ../，即项目根目录）
cd ..
./backend/target/release/rust-learning-backend
```

打开浏览器访问：

```
http://localhost:9001
```

后端默认监听 `0.0.0.0:9001`，同时提供前端资源和 `/evaluate.json` 接口。

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
        proxy_pass http://localhost:9001;
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
        reverse_proxy localhost:9001
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
PORT=9001 ./target/release/rust-learning-backend
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
const res = await fetch("http://localhost:9001/evaluate.json", {
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

### 4. 章节 URL 与分享

每个章节都有独立的 URL，方便收藏和分享。

#### URL 格式

```
http://localhost:8080/#chapter/<moduleIdx>/<chapterIdxInModule>
```

例如：

```
http://localhost:8080/#chapter/0/0   # 基础入门 - Hello World
http://localhost:8080/#chapter/1/3   # 中等应用 - 闭包与迭代器
http://localhost:8080/#chapter/3/5   # Q & A - 第 6 个问题
```

#### 行为

- 点击左侧目录中的章节按钮，URL 会自动更新为对应 hash。
- 直接访问带 hash 的 URL，页面加载后会自动跳转到对应章节。
- 浏览器前进/后退按钮可以切换章节。
- hash 路由不需要后端特殊配置，纯静态托管即可支持。

### 5. 使用 Rust 官方 Playground 接口（可选）

如果你不想部署本地后端，可以直接使用 Rust 官方 Playground 的 `/evaluate.json` 接口。该接口无需本地 Docker，但**需要访问外网**，且无法控制执行环境。

修改 `js/app.js` 中的 `runCode` 函数，将请求地址改为：

```js
const res = await fetch("https://play.rust-lang.org/evaluate.json", {
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
```

官方接口的请求体格式与本地后端一致，响应格式也兼容：

```json
{
  "result": "Hello\n",
  "error": ""
}
```

#### 适用场景

- 本地没有 Docker 环境
- 只需要快速演示前端功能
- 部署环境无法运行本地后端

#### 限制

- 必须能够访问外网（`https://play.rust-lang.org`）
- 无法自定义执行超时、内存限制等资源参数
- 依赖第三方服务，不适合生产环境或对稳定性要求高的场景

## 环境变量

| 变量 | 默认值 | 说明 |
|------|--------|------|
| `PORT` | `9001` | 后端监听端口 |
| `STATIC_DIR` | `../` | 前端静态文件目录（相对 backend 目录） |
| `CONCURRENCY` | `4` | 最大并发编译任务数 |
| `TIMEOUT_SECONDS` | `120` | 单次编译运行超时时间 |
| `MEMORY_LIMIT_MB` | `512` | Docker 容器内存限制 |
| `DOCKER_IMAGE` | `rust-learning-playground:1.86` | 编译运行使用的 Docker 镜像 |

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
2. 通过 `docker run` 启动隔离容器，使用 `cargo build --offline` 离线编译并运行代码
3. 容器限制：无网络、最小权限、512MB 内存、1 CPU、64 进程、120 秒超时
4. 返回 `result`（标准输出）和 `error`（编译/运行错误），前端分别用绿色和红色展示

### 离线编译说明

由于容器使用 `--network none`，无法访问 crates.io。因此后端会在临时项目中同时写入 `Cargo.toml` 和固定的 `Cargo.lock`（来自 `backend/src/Cargo.lock.playground`），并调用 `cargo build --offline`。只要自定义镜像 `rust-learning-playground:1.86` 中已缓存对应版本的 crate，编译就能在无网络环境下完成。

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

- 后端服务需要 Docker 守护进程正常运行，并能够运行 `rust-learning-playground:1.86` 镜像。如果镜像未构建，请参考上文「构建 Docker 编译镜像」步骤。
- 首次构建 `rust-learning-playground:1.86` 镜像时需要联网下载 crate；构建完成后，运行时容器无需联网。
- 示例代码均已控制在数秒内完成；过长或无限循环会被 Docker 超时机制终止。
- 移动端浏览器中，侧边栏可通过顶部菜单按钮展开/收起。
- 主题设置会保存在浏览器 `localStorage` 中，刷新页面后仍然生效。
- 修改 `codemirror-bundle.js` 后，建议同步更新 `js/app.js` 和 `index.html` 中的版本参数（`?v=N`），避免浏览器缓存旧 bundle。

## 自动部署

本项目已配置 Gitee CI，推送 `v*` 标签时自动部署前后端：

```bash
git tag v1.0.0
git push origin v1.0.0
```

### 前置条件

1. 在 Gitee 仓库 **设置 → CI/CD → 变量** 中配置：
   - `SSH_PRIVATE_KEY`：用于登录前后端服务器的 SSH 私钥
   - `FRONTEND_HOST`、`FRONTEND_USER`、`FRONTEND_WEB_ROOT`：前端服务器信息
   - `BACKEND_HOST`、`BACKEND_USER`、`BACKEND_DEPLOY_DIR`、`BACKEND_SERVICE`、`BACKEND_PORT`：后端服务器信息
   - `EVALUATE_URL`：生产环境 `/evaluate.json` 完整地址，例如 `http://api.rust-learning.example.com/evaluate.json`
2. 后端服务器安装 Rust、Cargo、Docker，并构建好 `rust-learning-playground:1.86` 镜像。
3. 后端服务器配置 systemd 服务：
   ```bash
   sudo cp systemd/rust-learning-backend.service /etc/systemd/system/
   sudo systemctl daemon-reload
   sudo systemctl enable rust-learning-backend
   ```
4. 把 CI 使用的 SSH 公钥添加到前后端服务器的 `~/.ssh/authorized_keys`。

### 部署流程

- **前端**：CI 通过 `scripts/deploy-frontend.sh` 把 `index.html`、`css/`、`js/`、`libs/`、`images/` 上传到 `FRONTEND_WEB_ROOT`，并自动把 `index.html` 中的 `evaluateUrl` 替换为 `EVALUATE_URL`。
- **后端**：CI 通过 `scripts/deploy-backend.sh` 把源码上传到 `BACKEND_DEPLOY_DIR`，在服务器上执行 `cargo build --release` 并重启 `BACKEND_SERVICE`。

## 许可证

MIT

Copyright (c) 2026 Rust 学习之旅贡献者
