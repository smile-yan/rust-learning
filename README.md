# Rust 学习之旅

一个基于 CDN 的纯前端 Rust 交互式学习网站，包含理论与实践的多个章节。用户可以在浏览器中编辑 Rust 代码，点击「运行」按钮后通过 Rust Playground API 查看程序输出。

## 特性

- 📚 **三大模块，22 个章节**：
  - **基础入门**：Hello World、变量与可变性、数据类型、函数、控制流、所有权、结构体、枚举与模式匹配、错误处理、集合
  - **中等应用**：泛型、Trait、生命周期、闭包与迭代器、智能指针、并发基础
  - **高级应用**：unsafe Rust、宏编程、异步编程、Web 开发基础、数据库与序列化、项目工程化
- 📝 **在线编辑器**：基于 CodeMirror 6，支持语法高亮
- ▶️ **一键运行**：调用 Rust Playground 公开 API 真实编译执行
- 📱 **响应式布局**：适配桌面端与移动端，左侧边栏按模块分组
- 🚀 **零构建步骤**：直接打开 HTML 即可使用

## 快速开始

1. 进入项目目录：

```bash
cd /Users/yanshili/me/projects/rust-projects
```

2. 启动本地 HTTP 服务器（必须，因为浏览器 ES 模块需要 HTTP 协议）：

```bash
python3 -m http.server 8080
```

3. 在浏览器中打开：

```
http://localhost:8080
```

## 项目结构

```
.
├── index.html       # 页面骨架与 CDN 引入
├── css/
│   └── style.css    # 自定义样式
├── js/
│   ├── chapters.json  # 章节数据（JSON 格式）
│   └── app.js         # 交互逻辑与 Rust Playground API 调用
└── README.md
```

## CDN 依赖

- [Tailwind CSS](https://tailwindcss.com/)
- [CodeMirror 6](https://codemirror.net/)
- [Marked.js](https://marked.js.org/)

## 章节数据结构

`js/chapters.json` 是一个 JSON 文件，包含 `modules` 数组，每个模块包含若干章节：

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

## 运行原理

点击「运行」后，前端会将编辑器中的代码通过 `fetch` POST 到：

```
https://play.rust-lang.org/evaluate.json
```

请求体示例：

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

API 返回 `result`（标准输出）和 `error`（编译/运行错误），前端分别用绿色和红色展示。

## 可用的外部 crate

Rust Playground 预装了大量常用 crate，本教程中已经用到：

- `tokio`：异步运行时
- `hyper`：底层 HTTP 库
- `serde` / `serde_json`：序列化与反序列化

更多可用 crate 可查看 [Rust Playground Crates 列表](https://play.rust-lang.org/help#crates)。

## 注意事项

- 运行代码需要能够访问互联网（Rust Playground 服务器）。
- 示例代码均已控制在数秒内完成；过长或无限循环会被 API 超时拒绝。
- 移动端浏览器中，侧边栏可通过顶部菜单按钮展开/收起。
- 部分高级章节（如 Web 服务）受 Playground 环境限制无法真正监听端口，示例仅用于展示 API 和验证编译。

## 许可证

MIT
