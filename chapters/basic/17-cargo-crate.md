---
title: "项目组织与 Cargo crate"
module: "基础入门"
order: 17
code: |
  // pub 表示对外公开，这些函数构成库对外的接口
  pub fn add(a: i32, b: i32) -> i32 {
      a + b
  }

  // Result<T, E> 表示可能失败的操作：Ok(结果) 或 Err(错误)
  pub fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
      if b == 0 {
          Err("除数不能为零")
      } else {
          Ok(a / b)
      }
  }

  /// 返回一个问候语
  ///
  /// # Examples
  ///
  /// ```
  /// assert_eq!(rust_projects::greet("Rust"), "Hello, Rust!");
  /// ```
  pub fn greet(name: &str) -> String {
      format!("Hello, {}!", name)
  }

  fn main() {
      println!("{}", greet("Rust"));
      println!("2 + 3 = {}", add(2, 3));
      // {:?} 以调试格式打印，可以直接输出 Result
      println!("10 / 2 = {:?}", divide(10, 2));
      println!("10 / 0 = {:?}", divide(10, 0));
  }

  // #[cfg(test)] 表示这个模块只在 cargo test 时才编译
  #[cfg(test)]
  mod tests {
      // 引入外层模块的全部内容，测试里才能直接调用 add 等函数
      use super::*;

      // #[test] 把一个函数标记为测试用例
      #[test]
      fn test_add() {
          assert_eq!(add(2, 3), 5);
      }

      // unwrap 在遇到 Err 时会 panic，从而让测试失败
      #[test]
      fn test_divide_ok() {
          assert_eq!(divide(10, 2).unwrap(), 5);
      }

      // is_err() 判断结果是否为 Err 变体
      #[test]
      fn test_divide_by_zero() {
          assert!(divide(10, 0).is_err());
      }

      #[test]
      fn test_greet() {
          assert_eq!(greet("Rust"), "Hello, Rust!");
      }
  }
hint: "文档注释中的代码示例默认会作为文档测试运行，是编写可测试文档的好方法。在 Playground 中 tests 模块也会被执行。"
exercises:
  - title: "给函数写文档注释"
    description: "为 add 函数编写 /// 文档注释，并在注释中写一个可运行的示例代码块。"
    code_template: |
      /// 计算两数之和
      ///
      /// # Examples
      /// ```
      /// assert_eq!(add(2, 3), 5);
      /// ```
      fn add(a: i32, b: i32) -> i32 {
          a + b
      }

      fn main() {
          println!("{}", add(2, 3));
      }
---

# 项目组织与 Cargo crate

Cargo 是 Rust 的构建系统和包管理器。它不仅能编译代码、管理依赖，还能运行测试、生成文档、发布 crate。学会 Cargo，才算真正进入 Rust 工程化开发。可以毫不夸张地说，Cargo 是 Rust 生态如此繁荣的重要功臣之一。

## 引入：Cargo 是你的项目经理 📋

没有 Cargo，你需要手动调用 `rustc`、管理依赖路径、记住各种编译选项。有了 Cargo，一句 `cargo run` 就能完成编译和运行，一句 `cargo test` 就能跑测试。它让 Rust 项目从“能跑”变成“好维护”。

## 概念图解 💡

```rust
my_project/
├── Cargo.toml    ← 项目配置：名称、版本、依赖
├── Cargo.lock    ← 依赖版本锁定
└── src/
    ├── main.rs   ← 二进制入口
    └── lib.rs    ← 库入口（可选）
```

## 深入讲解

### Cargo.toml

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0"
```

### 常用命令

- `cargo new project_name`：创建新项目
- `cargo build`：编译项目
- `cargo run`：编译并运行
- `cargo test`：运行测试
- `cargo check`：快速检查代码，不生成可执行文件
- `cargo doc --open`：生成并打开文档

### 依赖管理

Cargo 会自动从 crates.io 下载依赖，并缓存到本地。`Cargo.lock` 会锁定具体版本，保证团队协作时依赖一致。

### 工作空间（Workspace）

大型项目可以用 workspace 管理多个 crate：

```toml
[workspace]
members = ["crate-a", "crate-b"]
```

## 常见误区 ⚠️

- 直接修改 `Cargo.lock`：通常应该通过修改 `Cargo.toml` 来更新依赖。
- 忽略 `Cargo.toml` 中的 `edition`：不同 edition 的语法和特性可能不同。
- `cargo check` 和 `cargo build` 混淆：`check` 更快，但不生成产物。
- 把二进制和库 crate 的入口文件放错位置。

## 一句话总结 ✅

Cargo 是 Rust 的瑞士军刀，掌握 `cargo new/build/run/test/check/doc`，能大幅提升 Rust 开发效率和工程化能力。

<RustPlayground />
