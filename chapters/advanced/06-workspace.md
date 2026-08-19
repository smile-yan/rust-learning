---
title: "项目工程化 workspace"
module: "高级应用"
order: 6
code: |
  // pub 使函数对 crate 外部可见，构成库的公共 API
  pub fn add(a: i32, b: i32) -> i32 {
      a + b
  }

  // 用 Result 显式返回错误，而不是 panic，调用方可以优雅处理
  pub fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
      if b == 0 {
          Err("除数不能为零")
      } else {
          Ok(a / b)
      }
  }

  // 文档注释（///）中 ``` 包裹的示例代码，
  // 会被 cargo test 作为文档测试执行
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
      println!("10 / 2 = {:?}", divide(10, 2));
      println!("10 / 0 = {:?}", divide(10, 0));
  }

  // #[cfg(test)] 标记的模块只在运行测试时编译，不进入发布产物
  #[cfg(test)]
  mod tests {
      // use super::* 把外层模块的项引入测试作用域
      use super::*;

      #[test]
      fn test_add() {
          assert_eq!(add(2, 3), 5);
      }

      #[test]
      fn test_divide_ok() {
          assert_eq!(divide(10, 2).unwrap(), 5);
      }

      #[test]
      fn test_divide_by_zero() {
          assert!(divide(10, 0).is_err());
      }

      #[test]
      fn test_greet() {
          assert_eq!(greet("Rust"), "Hello, Rust!");
      }
  }
hint: "在 Playground 中运行会自动执行测试。实际项目中使用 cargo test 运行所有测试，cargo clippy 做代码检查。"
exercises:
  - title: "给函数添加文档测试"
    description: "为一个 add 函数编写 /// 示例，并用 cargo test 验证。"
    code_template: |
      /// # Examples
      /// ```
      /// assert_eq!(rust_projects::add(2, 3), 5);
      /// ```
      pub fn add(a: i32, b: i32) -> i32 {
          a + b
      }

      fn main() {}
---

# 🏗️ 项目工程化 workspace

当代码从个人脚本成长为团队协作项目时，工程化能力就显得至关重要。Rust 通过 Cargo 工具链、模块系统、测试体系和 CI/CD 集成，提供了一整套成熟的工程化方案。

## 🎯 为什么工程化不可或缺

想象一个开源项目每天有几十个 PR：没有统一格式、没有自动化测试、没有文档检查，代码质量会迅速劣化。Rust 的工具链把「质量门禁」建在每次提交之前。

## 🔧 Cargo 工具链速览

- `cargo new`：创建项目
- `cargo build` / `cargo run`：编译与运行
- `cargo test`：运行测试
- `cargo check`：快速检查代码，不生成可执行文件（比 build 快得多）
- `cargo clippy`：更严格的静态检查，给出优化建议
- `cargo fmt`：统一代码格式
- `cargo doc --open`：生成并打开文档
- `cargo publish`：发布到 crates.io

## 🦀 模块系统

`mod`、`use`、`pub` 组织代码结构；`crate`、`super`、`self` 控制可见性。合理的模块划分让项目：
- 边界清晰
- 依赖可控
- 测试更易写

## ✅ 测试分类

- **单元测试**：与源码放在一起，验证单个函数或模块，使用 `#[cfg(test)] mod tests`
- **集成测试**：放在 `tests/` 目录，验证多个模块协作
- **文档测试**：写在文档注释的代码块中，保证示例代码始终可运行

## ⚙️ 持续集成

GitHub Actions 等 CI 工具通常会运行：
- `cargo test` 保证功能正确
- `cargo clippy` 捕获潜在问题
- `cargo fmt --check` 保证格式统一
- `cargo doc` 检查文档是否能正常生成

## 📦 工作空间 Workspace

当项目变大，可以使用 Cargo workspace 管理多个 crate：
- 共享 `Cargo.lock` 和 target 目录
- 拆分库、二进制、测试、工具
- 统一版本管理

典型结构：
```
my-project/
├── Cargo.toml      # workspace 根
├── crates/
│   ├── core/
│   ├── server/
│   └── cli/
```

## 🔒 依赖管理

- 使用 `cargo tree` 查看依赖树
- 使用 `cargo outdated` 检查过期依赖
- 使用 `cargo audit` 扫描已知安全漏洞
- 使用 `cargo-deny` 统一许可证和依赖策略

## 📝 文档与示例

- 文档注释 `///` 支持 Markdown
- 模块级文档 `//!`
- `examples/` 目录存放可运行示例
- 使用 `cargo doc` 生成 docs.rs 风格文档

## 📦 发布 crate

发布前要关注：
- `Cargo.toml` 元数据完整（name、version、authors、license、description）
- 语义化版本号
- README 和 CHANGELOG
- 必要的 feature gate 避免依赖膨胀
- 先用 `cargo publish --dry-run` 验证

## 💡 一句话总结

Rust 项目工程化 = Cargo 工具链守护质量 + Workspace 管理规模 + 模块系统划分边界 + 三层测试保证正确 + CI/CD 自动把关，让项目从小玩具成长为可维护的产品。

<RustPlayground />
