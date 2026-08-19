---
title: "环境变量与配置 dotenv / config"
module: "中等应用"
order: 13
code: |
  fn main() {
      dotenv::dotenv().ok();

      let host = std::env::var("HOST").unwrap_or("127.0.0.1".to_string());
      let port: u16 = std::env::var("PORT")
          .unwrap_or("8080".to_string())
          .parse()
          .expect("PORT 必须是数字");

      println!("服务启动: http://{}:{}", host, port);
  }
hint: "dotenv 仅用于开发，生产环境通过系统环境变量注入机密。"
exercises:
  - title: "读取配置"
    description: "创建 .env 文件，在 Rust 中读取 APP_NAME 并打印。"
    code_template: |
      fn main() {
          dotenv::dotenv().ok();
          // let app = std::env::var("APP_NAME").unwrap();
      }
---

# 环境变量与配置 dotenv / config 🦀

真实应用不会把数据库密码、API 密钥硬编码在代码里。Rust 通过 `std::env` 原生支持环境变量，再配合 `dotenv` 和 config crate 实现优雅的多环境配置管理。

## 从生活类比开始 💡

环境变量就像餐厅的「今日特价」小黑板：厨房（代码）不需要改动，只要换个黑板（环境变量）就能改变今天做什么菜。

![配置加载流程](/images/module1-dotenv-config.svg)

## 深入讲解

### std::env

```rust
let host = std::env::var("HOST").unwrap_or("127.0.0.1".to_string());
```

### dotenv

将 `.env` 文件中的键值对加载到进程环境变量中，优先级**低于**系统环境变量。

### config crate

支持从多种来源（文件、环境变量、命令行参数）加载配置，并自动合并。

### 分层策略

1. 默认值写在代码里
2. `.env` 覆盖开发环境
3. 系统环境变量覆盖生产环境
4. 命令行参数优先级最高

## 一句话总结 🦀

> 配置管理是应用启动的第一步；dotenv 隔离开发环境，config crate 管理多环境配置。

<RustPlayground />
