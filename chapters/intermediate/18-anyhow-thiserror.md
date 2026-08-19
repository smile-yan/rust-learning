---
title: "错误处理进阶 anyhow / thiserror"
module: "中等应用"
order: 18
code: |
  use thiserror::Error;
  use anyhow::{Context, Result};

  #[derive(Error, Debug)]
  enum DataError {
      #[error("字段缺失: {0}")]
      MissingField(String),
      #[error("类型不匹配")]
      TypeMismatch,
  }

  fn parse_age(data: &str) -> Result<u32> {
      let age = data
          .parse::<u32>()
          .context("age 不是有效数字")?;
      Ok(age)
  }

  fn main() -> Result<()> {
      let content = std::fs::read_to_string("user.json")
          .context("无法打开 user.json")?;
      let age = parse_age(&content)?;
      println!("年龄: {}", age);
      Ok(())
  }
hint: "库用 thiserror，应用用 anyhow。.context() 是最常用的追加上下文方法。"
exercises:
  - title: "用 anyhow 传播错误"
    description: "写一个返回 anyhow::Result 的函数，使用 ? 和 .context()。"
    code_template: |
      use anyhow::Result;

      fn read_config(path: &str) -> Result<String> {
          std::fs::read_to_string(path)
              .context("读取配置失败")
      }

      fn main() -> Result<()> {
          let s = read_config("config.toml")?;
          println!("{}", s.len());
          Ok(())
      }
---

# 错误处理进阶 anyhow / thiserror 🦀

Rust 内置的 `Result` 和 `Option` 已经提供了强大的错误表达能力。但在真实项目中，你还需要统一的错误类型、易读的错误链和格式化输出。`thiserror` 和 `anyhow` 是 Rust 生态中处理错误的两大神器。

## 从生活类比开始 💡

错误处理就像快递的「异常登记簿」：`thiserror` 是物流公司的标准异常标签（精确分类），`anyhow` 是你的客服统一答复（给用户看，并附原始标签）。

![错误处理分层](/images/module1-error-handling-anyhow.svg)

## 深入讲解

### thiserror：库的视角

在库（library）中，你需要定义**结构化的错误类型**，让调用者可以根据错误类型做精确处理：
```rust
#[derive(thiserror::Error, Debug)]
enum MyError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
```

### anyhow：应用的视角

在应用（application）中，你关心的是**快速传播和渲染**，不需要区分每个错误细节：
```rust
fn main() -> anyhow::Result<()> {
    let content = std::fs::read_to_string("file.txt")?;
    Ok(())
}
```

## 分层选择

- **库代码**：用 `thiserror` 定义结构化错误，精确到变体
- **应用代码**：用 `anyhow::Result` 统一错误类型，`.context()` 追加上下文

## 一句话总结 🦀

> thiserror 在库中定义结构化错误，anyhow 在应用中统一处理并追加上下文；两者结合构成 Rust 错误处理的最佳实践。

<RustPlayground />
