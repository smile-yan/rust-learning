---
title: "显式优于隐式：错误处理与可变性"
module: "Rust 哲学"
order: 5
code: |
  use std::fs;

  // 用 Result 作为返回类型，强迫调用方显式处理失败，而不是抛异常
  fn read_file(path: &str) -> Result<String, String> {
      // ? 显式传播错误
      let content = fs::read_to_string(path)
          .map_err(|e| e.to_string())?;
      Ok(content)
  }

  // Option 显式表达「可能没有结果」，不存在静默的 null
  fn double_if_positive(n: i32) -> Option<i32> {
      if n > 0 {
          Some(n * 2)
      } else {
          None
      }
  }

  fn main() {
      // 可变变量必须显式声明
      let mut count = 0;
      count += 1;
      println!("count: {}", count);

      // 显式错误处理
      match read_file("/etc/hosts") {
          Ok(content) => println!("读取了 {} 字节", content.len()),
          Err(e) => println!("读取失败: {}", e),
      }

      // Option 也必须显式处理
      match double_if_positive(5) {
          Some(v) => println!("结果: {}", v),
          None => println!("输入不合法"),
      }
  }
hint: "显式错误处理让调用者清楚知道哪里可能失败。生产代码中优先使用 ? 或 match，而不是 unwrap。"
exercises:
  - title: "显式处理 Option"
    description: "对 Some(5) 和 None 分别用 match 处理。"
    code_template: |
      fn main() {
          let x: Option<i32> = Some(5);
          match x {
              Some(v) => println!("{}", v),
              None => println!("none"),
          }
      }
---

# 显式优于隐式：错误处理与可变性 🎭

Rust 喜欢把重要决策显式化。变量默认可变还是不可变？错误如何处理？这些在其他语言里可能隐式发生，但在 Rust 里必须明确写出来。

## 默认不可变

Rust 中 `let x = 5;` 默认不可变。需要修改时必须写 `let mut x = 5;`。

这背后的哲学是：
- 不可变数据更容易推理
- 不可变数据可以安全地并发共享
- `mut` 标记让代码中的变化点一目了然


![默认不可变，mut 显式开口](/images/module3-explicit-mut.svg)

## 显式错误处理

Rust 没有异常机制。函数用 `Result<T, E>` 或 `Option<T>` 返回可能失败的结果，调用者必须决定：
- 用 `match` / `if let` 处理
- 用 `?` 把错误传播给上层
- 用 `unwrap` / `expect` 在确定安全时快速解包

```
fn read_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}
```


![Result 的三种显式处理](/images/module3-explicit-error.svg)

## 没有隐式转换

Rust 也没有隐式类型转换。`u8` 不会自动变成 `u32`，整数不会自动变成浮点数。你需要显式写出 `as`、`into()` 或 `from()`。

## 为什么这样更好？

显式化减少了 surprises。读代码时，你看到 `mut` 就知道这里会变化；看到 `?` 就知道这里可能提前返回错误；看到类型转换就知道数据在变化。

## 一句话总结 ✅

Rust 相信「重要的决定应该被看见」：可变性、错误、类型转换都要显式表达，让代码意图更清晰。

<RustPlayground />
