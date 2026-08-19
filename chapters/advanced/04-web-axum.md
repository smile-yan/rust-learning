---
title: "Web 开发基础 axum"
module: "高级应用"
order: 4
code: |
  use axum::{
      routing::get,
      Router,
      http::StatusCode,
  };

  // handler：返回 &'static str 会自动作为 200 OK 的响应体
  async fn hello() -> &'static str {
      "Hello, Rust Web!"
  }

  // 返回 (StatusCode, String) 元组可同时指定状态码和响应体
  async fn users() -> (StatusCode, String) {
      (StatusCode::OK, String::from("[\"Alice\", \"Bob\"]"))
  }

  // Router 把路径与 handler 关联，get(...) 限定只响应 GET 请求
  fn app() -> Router {
      Router::new()
          .route("/", get(hello))
          .route("/users", get(users))
  }

  fn main() {
      // Playground 无法真正启动 HTTP 服务监听端口，
      // 这里演示路由定义与 handler 可以编译通过。
      let _router = app();
      println!("Axum 路由定义成功");
      println!("GET / -> hello");
      println!("GET /users -> users");
  }
hint: "Playground 无法暴露网络端口，此示例仅用于展示 Axum 框架 API 和验证编译。真实项目需要 tokio runtime 启动服务。"
exercises:
  - title: "定义 Axum 路由"
    description: "创建一个 Router，把 GET /hello 映射到返回字符串的 handler。"
    code_template: |
      use axum::{routing::get, Router};

      async fn hello() -> &'static str {
          "Hello, Axum!"
      }

      fn main() {
          let app = Router::new().route("/hello", get(hello));
          println!("路由创建成功");
      }
---

# 🌐 Web 开发基础 axum

Rust 在 Web 后端领域已经形成了成熟的框架生态。凭借零成本抽象、内存安全和出色的并发性能，Axum、Actix-web、Rocket 等框架正在构建越来越多高性能、高可靠的 Web 服务。

> ⚠️ **运行环境提示**
> 本章节代码使用了多线程 / 文件系统 / 外部 crate / FFI 等能力，**不能在前端 WASM Playground 运行**。当前 Playground 仅适合运行单线程、纯计算、标准库的示例。请复制代码到本地 IDE（VS Code + Rust Analyzer、CLion 或 cargo CLI）中运行。

## 🎯 为什么是 Rust 做 Web

传统 Web 后端语言在并发和安全性上总要二选一：要么写起来简单但性能受限，要么性能强劲但容易内存出错。Rust 借助所有权和异步运行时，让**高性能与类型安全兼得**。

## 🦀 Axum：基于 tokio 与 Tower

Axum 是官方推荐的新一代 Web 框架，设计理念是「处理器就是函数」：
- 基于 tokio 异步运行时
- 基于 hyper HTTP 实现
- 基于 Tower 服务抽象与中间件
- 类型安全的提取器（Extractor）

## ⚙️ 路由与 Handler

路由把 URL 模式映射到处理函数，Handler 接收请求、处理逻辑并返回响应。Axum 支持：
- 路径参数：`/:id`
- 查询参数：`Query<T>`
- 请求体提取：`Json<T>`、表单等
- 状态共享：`State`

![Axum 请求处理流程](/images/module2-axum-request-flow.svg)

## 🔧 中间件

通过 Tower 的中间件机制，可以像叠汉堡一样叠加能力：
- 日志记录（trace）
- 跨域处理（CORS）
- 认证与授权
- 限流（rate limit）
- 超时控制

![Tower 中间件栈](/images/module2-middleware-stack.svg)

## ✅ 响应类型

Handler 返回类型非常灵活：
- `&'static str`、String
- `Json<T>`
- `Result<T, E>`
- 自定义 `Response`

Axum 会自动调用对应的 `IntoResponse` trait 完成转换。

## ⚠️ Playground 的限制

在浏览器 Playground 中无法真正监听网络端口，因此示例通常只验证路由定义和 Handler 能够编译通过。真实项目需要配合 `tokio::net::TcpListener` 启动服务，生产环境往往还要前置 Nginx 等反向代理。

## 💡 一句话总结

Rust Web 开发的核心公式是：Axum（路由+Handler）+ Tower（中间件）+ tokio（异步运行时）= 类型安全、高性能的现代 Web 服务。

<RustPlayground />
