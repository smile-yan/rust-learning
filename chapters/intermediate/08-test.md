---
title: "测试与错误处理最佳实践 #[test]"
module: "中等应用"
order: 8
code: |
  use std::fs::File;
  use std::io::{self, Read};

  // ? 运算符：出错时提前返回并把错误交给调用者，成功时取出其中的值
  fn read_username_from_file(
      path: &str,
  ) -> Result<String, io::Error> {
      let mut file = File::open(path)?;
      let mut username = String::new();
      file.read_to_string(&mut username)?;
      Ok(username)
  }

  // 用 Result 显式表达「可能失败」，这里用 String 作错误类型
  fn divide(a: f64, b: f64) -> Result<f64, String> {
      if b == 0.0 {
          Err(String::from("除数不能为零"))
      } else {
          Ok(a / b)
      }
  }

  // #[cfg(test)] 标注的模块只在 cargo test 时编译
  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn divide_normal() {
          assert_eq!(divide(10.0, 2.0).unwrap(), 5.0);
      }

      // 验证错误路径：除零应当返回 Err
      #[test]
      fn divide_by_zero() {
          assert!(divide(10.0, 0.0).is_err());
      }
  }

  fn main() {
      // 用 match 穷尽处理 Ok / Err 两种情况
      match divide(10.0, 2.0) {
          Ok(result) => println!("结果: {}", result),
          Err(e) => println!("错误: {}", e),
      }

      // 这个文件可能不存在，会返回错误
      match read_username_from_file("nonexistent.txt") {
          Ok(name) => println!("用户名: {}", name),
          Err(e) => println!("读取失败: {}", e),
      }
  }
hint: "库代码优先返回 Result，应用代码可以用 anyhow 简化错误处理。测试覆盖率是项目质量的重要保障。"
exercises:
  - title: "为 add 写单元测试"
    description: "在 #[cfg(test)] mod tests 中为 add 函数写 assert_eq! 测试。"
    code_template: |
      fn add(a: i32, b: i32) -> i32 { a + b }

      #[cfg(test)]
      mod tests {
          use super::*;

          #[test]
          fn it_adds() {
              assert_eq!(add(2, 3), 5);
          }
      }

      fn main() {}
---

# 测试与错误处理最佳实践 #[test] 🦀

随着项目规模增长，良好的测试和错误处理策略变得至关重要。Rust 提供了内置的测试框架和强大的类型驱动错误处理机制。

## 从生活类比开始 💡

测试就像汽车的安全气囊和刹车系统：它们不直接让你开得更快，但能在危险发生时保护你。错误处理则像是导航仪，遇到岔路时告诉你该转弯还是掉头。

## 深入讲解

### 测试组织

- **单元测试**：与源码放一起，使用 `#[cfg(test)]`。
- **集成测试**：放在 `tests/` 目录，测试公共 API。
- **文档测试**：在文档注释中写可运行示例，代码和文档同步。
- **自定义 fixture**：使用 `rstest` 或手动构建辅助函数。

### 错误处理库

- **`anyhow`**：简化应用错误处理，自动转换错误类型。
- **`thiserror`**：为库定义结构化错误类型，便于调用者匹配。

### 最佳实践

- 库代码优先返回 `Result`，让调用者决定如何处理。
- 应用代码可以用 `?` + `anyhow` 快速传播错误。
- 不要把 `unwrap()` 用于可能失败的生产代码。

### 自定义错误类型

使用枚举和 `thiserror` 可以定义语义清晰的错误类型：

```rust
#[derive(thiserror::Error, Debug)]
enum MyError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
```

## 常见误区 ⚠️

- **误区 1**：测试越多越好。  
  ✅ 正解：测试要覆盖关键路径和边界条件，避免无意义的重复测试。
- **误区 2**：`unwrap()` 在测试中没关系。  
  ✅ 正解：测试中也应使用 `?` 或 `expect`，让失败信息更清晰。
- **误区 3**：所有函数都应该返回 `Result`。  
  ✅ 正解：只在可能失败的地方返回 `Result`，简单函数可以直接返回值。

## 一句话总结 🦀

> 好的测试让重构 fearless，好的错误处理让用户体验稳健。

<RustPlayground />
