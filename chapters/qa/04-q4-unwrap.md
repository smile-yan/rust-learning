---
title: "Q4: unwrap 和 ? 有什么区别？"
module: "Q & A"
order: 4
code: |
  // 返回 Result 表示可能失败：Ok 装成功值，Err 装错误信息
  fn may_fail(input: i32) -> Result<i32, String> {
      if input > 0 {
          Ok(input * 2)
      } else {
          Err(String::from("输入必须大于 0"))
      }
  }

  // 只有返回 Result 或 Option 的函数内部才能使用 ?
  fn double_positive(input: i32) -> Result<i32, String> {
      // ? 会在 Err 时提前返回错误
      let value = may_fail(input)?;
      Ok(value + 1)
  }

  fn main() {
      // unwrap：成功则取值，失败直接 panic，仅在确信不会失败时使用
      let good = may_fail(5).unwrap();
      println!("unwrap 结果: {}", good);

      // ? 示例：传播错误，传入 -1 时 may_fail 返回 Err，
      // double_positive 随之提前返回
      match double_positive(-1) {
          Ok(n) => println!("成功: {}", n),
          Err(e) => println!("错误: {}", e),
      }

      match double_positive(5) {
          Ok(n) => println!("成功: {}", n),
          Err(e) => println!("错误: {}", e),
      }
  }
hint: "unwrap 会让程序在失败时崩溃。生产代码中优先使用 ? 传播错误，或用 match 显式处理。"
exercises:
  - title: "用 ? 传播错误"
    description: "实现 parse_number 并用 ? 在 compute 中传播。"
    code_template: |
      fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
          s.parse()
      }

      fn compute(s: &str) -> Result<i32, std::num::ParseIntError> {
          let n = parse_number(s)?;
          Ok(n * 2)
      }

      fn main() {
          println!("{:?}", compute("10"));
      }
---

# Q4: unwrap 和 ? 有什么区别？ ⚠️

Rust 的错误处理非常强大，但新手常常被 `unwrap` 和 `?` 搞得晕头转向。两者都用于处理 `Result` 或 `Option`，但安全性和适用场景天差地别。

## 🧭 引入与类比

想象你有一家快递店 📦：

- `unwrap` 就像直接拆开包裹，假设里面一定是你想要的东西。如果包裹是空的或者碎的，整个店就倒闭（panic）。
- `?` 就像把有问题的包裹退给上级仓库，让更上层决定怎么处理，自己不用当场崩溃。

## 💡 核心概念图解

![unwrap 与 ? 的行为差异](/images/module4-unwrap-question.svg)

## 🔧 深入讲解

### unwrap

- 如果值是 `Ok`/`Some`，返回内部值。
- 如果值是 `Err`/`None`，程序会 **panic**。
- 适合快速原型、测试，或者你能 100% 确信不会失败的场景。

```rust
let config = std::fs::read_to_string("/etc/hosts").unwrap();
```

### ? 操作符

- 如果值是 `Ok`/`Some`，解包并继续。
- 如果值是 `Err`/`None`，立即从当前函数返回这个错误。
- 只能在返回 `Result` 或 `Option` 的函数中使用。

```rust
fn read_username() -> Result<String, std::io::Error> {
    let file = std::fs::File::open("user.txt")?;
    // ...
    Ok(String::from("alice"))
}
```

### 选择建议

| 场景 | 推荐方式 |
|------|----------|
| 快速原型 / 测试 | `unwrap` / `expect` |
| 库代码 | `?` |
| 用户输入 / 网络请求 | `?` 或 `match` |
| 程序启动时读取必要配置 | `expect("配置文件缺失")` |

## ⚠️ 常见误区

- **误区 1**：`unwrap` 和 `?` 完全等价，只是写法不同。
  - ✅ 正解：`unwrap` 会在失败时 panic，而 `?` 会把错误传播给调用者。
- **误区 2**：生产代码里可以随便用 unwrap。
  - ✅ 正解：生产代码应尽量减少 unwrap，否则程序可能因未处理的错误而崩溃。
- **误区 3**：`?` 只能在 `main` 函数里用。
  - ✅ 正解：`?` 可以在任何返回 `Result` 或 `Option` 的函数中使用；`main` 也可以返回 `Result`。

## 📝 一句话总结

`unwrap` 是「我赌它不会失败，失败就 panic」的懒人工具；`?` 是「把错误优雅地交给上层处理」的生产级写法。

<RustPlayground />
