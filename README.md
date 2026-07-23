# Rust 学习之旅

一个纯前端的 Rust 交互式学习网站。使用 **Vue 3** 管理状态，基于 **CodeMirror 6** 的在线编辑器，所有依赖（Vue、CodeMirror、Tailwind CSS、Marked.js）全部本地化，无需访问外部 CDN。

![Vue](https://img.shields.io/badge/Vue-3-4FC08D?logo=vue.js&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-blue.svg)

<img alt="image" src="https://github.com/user-attachments/assets/f4c41da8-9a57-40ce-99b1-4ed493f110a9" />

欢迎访问 rust 学习 playground: [https://rust.smileyan.cn/](https://rust.smileyan.cn/)


## 特性

- 📚 **四大模块，64 个章节**：基础入门、中等应用、高级应用、Q & A 常见问题
- 📝 **在线编辑器**：基于 CodeMirror 6，支持 Rust 语法高亮、行号、括号匹配、自动补全
- 🌙 **深色/浅色主题**：自动跟随系统偏好，支持手动切换并持久化
- 📱 **响应式布局**：适配桌面端与移动端，左侧边栏按模块分组
- ⌨️ **快捷键支持**：`Ctrl/Cmd + Enter` 快速运行代码（需配合后端接口）
- 🧩 **章节练习**：每个章节均配有 1–3 道简单编程题，点击即可加载到编辑器中练习
- 🚀 **无外部 CDN 依赖**：Vue、CodeMirror、Tailwind CSS、Marked.js 全部本地化
- ⚡ **Vue 3 Composition API**：响应式状态管理与模板渲染

## 项目结构

```
.
├── index.html                # Vue 3 应用挂载点
├── css/
│   └── style.css             # 自定义样式与主题变量
├── js/
│   ├── chapters.json         # 章节数据（JSON 格式）
│   └── app.js                # Vue 3 应用逻辑
├── libs/
│   ├── codemirror-bundle.js  # 本地 esbuild 打包的 CodeMirror 6 bundle
│   ├── tailwindcss.js        # 本地缓存的 Tailwind CSS Play CDN
│   ├── vue.esm-browser.prod.js  # Vue 3 生产构建
│   └── marked.min.js         # Marked.js 本地副本
├── codemirror-entry.js       # CodeMirror 打包入口（构建用）
├── package.json              # 仅用于安装 CodeMirror 构建依赖
└── README.md
```

## 本地预览

前端是纯静态资源，可直接用任意静态文件服务器预览：

```bash
# 在项目根目录运行
python3 -m http.server 8080
```

然后访问 `http://localhost:8080`。

## 运行 Rust 代码

编辑器中的「运行」按钮默认调用 `/evaluate.json` 接口执行 Rust 代码。**本仓库只包含前端页面**，如果需要在线执行代码，需要搭配后端服务：

- 推荐后端（与本项目配套）：[smile-yan/rust-playground-backend](https://github.com/smile-yan/rust-playground-backend)
- 或自行部署后端，并将 `js/app.js` 中的 `evaluateUrl` 修改为可用的后端地址

部署时也可通过 `index.html` 底部的 `window.RUST_PLAYGROUND.evaluateUrl` 配置后端接口地址。

## 部署

将项目根目录下的 `index.html`、`css/`、`js/`、`libs/`、`images/` 上传到任意静态托管服务即可，例如 Nginx、Caddy、GitHub Pages、Vercel、Netlify 或对象存储 CDN。

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

推送 `v*` 标签时，GitHub Actions 会自动通过 `scripts/deploy-frontend.sh` 将前端静态文件部署到服务器：

```bash
git tag v0.1.0
git push origin v0.1.0
```

#### 前置条件

1. 在 GitHub 仓库 **Settings → Secrets and variables → Actions** 中配置 Secrets：
   - `SSH_PRIVATE_KEY`：用于登录前端服务器的 SSH 私钥
   - `FRONTEND_HOST`、`FRONTEND_USER`、`FRONTEND_WEB_ROOT`：前端服务器信息
   - `EVALUATE_URL`：生产环境 `/evaluate.json` 完整地址
2. 把 CI 使用的 SSH 公钥添加到前端服务器的 `~/.ssh/authorized_keys`。

## 重新打包 CodeMirror

如果你修改了 `codemirror-entry.js` 或需要更新 CodeMirror 版本：

```bash
npm install
npm run build:codemirror
```

生成的 `libs/codemirror-bundle.js` 已包含完整的 CodeMirror 6 运行时、Rust 语言支持、oneDark 主题，以及自定义高亮所需的 `syntaxHighlighting`、`HighlightStyle`、`tags`。

## 技术栈

| 名称 | 来源 | 用途 |
|------|------|------|
| [Vue 3](https://cn.vuejs.org/) | 本地 `libs/vue.esm-browser.prod.js` | 响应式 UI 与状态管理（生产构建） |
| [Tailwind CSS](https://www.tailwindcss.cn/) | 本地 `libs/tailwindcss.js` | 原子化 CSS 样式 |
| [CodeMirror 6](https://codemirror.net/) | 本地 `libs/codemirror-bundle.js` | Rust 代码在线编辑器 |
| [Marked.js](https://marked.js.org/) | 本地 `marked.min.js` | Markdown 理论内容渲染 |

## 许可证

本项目采用 [MIT 许可证](LICENSE) 开源。

Copyright (c) 2026 Rust 学习之旅贡献者
