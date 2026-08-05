# Rust 学习之旅

一个纯前端的 Rust 交互式学习网站。使用 **Vue 3** 管理状态，基于 **CodeMirror 6** 的在线编辑器，由 **Vite** 构建打包；Tailwind CSS 与 Prism 等少量依赖本地化，无需访问外部 CDN。

![Vue](https://img.shields.io/badge/Vue-3-4FC08D?logo=vue.js&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-blue.svg)

<img alt="image" src="https://github.com/user-attachments/assets/f4c41da8-9a57-40ce-99b1-4ed493f110a9" />

欢迎访问 rust 学习 playground: [https://rust.smileyan.cn/](https://rust.smileyan.cn/)


## 特性

- 📚 **五大模块，73 个章节**：基础入门、中等应用、高级应用、Rust 哲学、Q & A 常见问题
- 🗺️ **章节矢量插图**：各章节配有 SVG 概念图解，首页附学习路线图
- 📝 **在线编辑器**：基于 CodeMirror 6，支持 Rust 语法高亮、行号、括号匹配、自动补全
- 🌙 **深色/浅色主题**：自动跟随系统偏好，支持手动切换并持久化
- 📱 **响应式布局**：适配桌面端与移动端，左侧边栏按模块分组
- ⌨️ **快捷键支持**：`Ctrl/Cmd + Enter` 快速运行代码（需配合后端接口）
- 🧩 **章节练习**：每个章节均配有 1–3 道简单编程题，点击即可加载到编辑器中练习
- 🚀 **无外部 CDN 依赖**：Vue、CodeMirror、Marked 由 Vite 打包，Tailwind CSS、Prism 本地化
- ⚡ **Vue 3 Composition API**：响应式状态管理与模板渲染

## 项目结构

```
.
├── index.html                # 封面页（Vite 入口之一）
├── app.html                  # 学习应用页面（Vue 3 挂载点，Vite 入口之一）
├── src/
│   ├── app.js                # Vue 3 应用逻辑
│   └── style.css             # 自定义样式与主题变量
├── js/
│   └── chapters.json         # 章节数据（JSON 格式，构建时打包进 JS）
├── public/                   # 静态资源（Vite 原样拷贝到 dist/）
│   ├── css/                  # Prism 主题样式
│   ├── libs/                 # 本地化的 Tailwind CSS Play CDN 与 Prism
│   └── images/               # 章节矢量插图（SVG）
├── scripts/
│   ├── deploy-frontend.sh    # 推送标签时触发的部署脚本（CI 调用）
│   └── validate_chapters.py  # 章节数据校验脚本
├── vite.config.js            # Vite 配置（多页入口、产物目录 dist/）
├── package.json
└── README.md
```

## 本地预览

项目使用 Vite 构建，先安装依赖再启动开发服务器：

```bash
npm install
npm run dev
```

然后访问终端输出的地址（默认 `http://localhost:5173`）。

也可以构建后预览产物：

```bash
npm run build
npm run preview        # 或：python3 -m http.server 8080 --directory dist
```

修改章节数据后可用校验脚本检查：

```bash
python3 scripts/validate_chapters.py
```

## 运行 Rust 代码

编辑器中的「运行」按钮默认调用 `app.html` 底部 `window.RUST_PLAYGROUND.evaluateUrl` 配置的接口执行 Rust 代码（默认为 `http://localhost:9001/evaluate.json`）。**本仓库只包含前端页面**，如果需要在线执行代码，需要搭配后端服务：

- 推荐后端（与本项目配套）：[smile-yan/rust-playground-backend](https://github.com/smile-yan/rust-playground-backend)
- 或自行部署后端，并修改 `app.html` 中的 `evaluateUrl` 为可用的后端地址；也可以配置为 Rust 官方 Playground 的 `https://play.rust-lang.org/evaluate.json`

## 部署

先执行 `npm run build`，然后将 `dist/` 目录的全部内容上传到任意静态托管服务即可，例如 Nginx、Caddy、GitHub Pages、Vercel、Netlify 或对象存储 CDN。访问根目录 `/` 将看到封面页，点击「开始学习」进入 `app.html` 学习应用。

### Nginx 示例

```nginx
server {
    listen 80;
    server_name rust-learning.example.com;
    root /var/www/rust-learning;
    index index.html;

    location / {
        try_files $uri $uri/ /index.html;
    }
}
```

### 自动部署

推送 `v*` 标签时，GitHub Actions 会自动执行 `scripts/deploy-frontend.sh`：安装依赖、构建、注入版本号与生产环境 `evaluateUrl`，再把 `dist/` 上传到服务器：

```bash
git tag v0.2.18
git push origin v0.2.18
```

#### 前置条件

1. 在 GitHub 仓库 **Settings → Secrets and variables → Actions** 中配置 Secrets：
   - `SSH_PRIVATE_KEY`：用于登录前端服务器的 SSH 私钥
   - `FRONTEND_HOST`、`FRONTEND_USER`、`FRONTEND_WEB_ROOT`：前端服务器信息
   - `EVALUATE_URL`：生产环境 `/evaluate.json` 完整地址
2. 把 CI 使用的 SSH 公钥添加到前端服务器的 `~/.ssh/authorized_keys`。

## 技术栈

| 名称 | 来源 | 用途 |
|------|------|------|
| [Vue 3](https://cn.vuejs.org/) | npm 依赖，Vite 打包 | 响应式 UI 与状态管理 |
| [CodeMirror 6](https://codemirror.net/) | npm 依赖，Vite 打包 | Rust 代码在线编辑器（含 oneDark 主题） |
| [Marked.js](https://marked.js.org/) | npm 依赖，Vite 打包 | Markdown 理论内容渲染 |
| [Tailwind CSS](https://www.tailwindcss.cn/) | 本地 `public/libs/tailwindcss.js` | 原子化 CSS 样式 |
| [Prism](https://prismjs.com/) | 本地 `public/libs/prism*.js` | 理论内容中的代码块高亮 |
| [Vite](https://vitejs.dev/) | devDependency | 开发服务器与生产构建 |

## 许可证

本项目采用 [MIT 许可证](LICENSE) 开源。

Copyright (c) 2026 Rust 学习之旅贡献者
