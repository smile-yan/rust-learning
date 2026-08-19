---
title: "类型转换与类型系统进阶 From / Into"
module: "中等应用"
order: 7
code: |
  use std::convert::TryInto;

  // 元组结构体（newtype 模式）：用不同类型区分语义，避免米和千米混用
  #[derive(Debug)]
  struct Meters(u32);
  #[derive(Debug)]
  struct Kilometers(u32);

  // 实现了 From，编译器会自动赠送反向调用的 Into
  impl From<Kilometers> for Meters {
      fn from(k: Kilometers) -> Self {
          Meters(k.0 * 1000)
      }
  }

  // 类型别名只是起名字，不创建新类型，UserId 与 u64 可以互换
  type UserId = u64;

  fn process_id(id: UserId) {
      println!("处理用户 ID: {}", id);
  }

  // 返回类型 !（never 类型）表示函数永不正常返回，
  // 常见于 panic 或死循环
  fn always_fail() -> ! {
      panic!("这个函数永远不会正常返回");
  }

  fn main() {
      // as 转换
      let x = 42_i32;
      // as 做基础类型间的显式强转，可能静默截断，需要谨慎
      let y = x as f64;
      println!("{} -> {}", x, y);

      // From / Into
      let k = Kilometers(5);
      // into 的目标类型由变量标注决定
      let m: Meters = k.into();
      println!("{:?}", m);

      // TryInto
      let big: i64 = 300;
      // 可能失败的转换返回 Result：300 超出 u8 上限，会得到 Err
      let small: Result<u8, _> = big.try_into();
      println!("转换结果: {:?}", small);

      // 类型别名
      let id: UserId = 12345;
      process_id(id);

      // never 类型不会执行
      // always_fail();
  }
hint: "优先使用 From/TryFrom 而不是 as，因为它们更安全、更可扩展。dyn Trait 提供动态分发能力。"
exercises:
  - title: "实现 From<&str> for String"
    description: "使用 to_string 实现 From<&str> 转换（String 已内置，练习手写自定义 From）。"
    code_template: |
      #[derive(Debug)]
      struct Name(String);

      impl From<&str> for Name {
          fn from(s: &str) -> Self {
              Name(s.to_string())
          }
      }

      fn main() {
          let n: Name = "Rust".into();
          println!("{:?}", n);
      }
---

# 类型转换与类型系统进阶 From / Into 🦀

Rust 的类型系统强大而严格，掌握类型转换和高级类型特性能让你写出更灵活、更安全的代码。

## 从生活类比开始 💡

类型转换就像单位换算：你可以把 5 公里换算成 5000 米，但不能把“苹果”换算成“公里”。Rust 的类型系统既允许安全的转换，也会拒绝不合理的转换。

## 深入讲解

### 类型转换

- **`as`**：用于基本类型之间的显式转换，如 `i32 as f64`。简单直接，但可能丢失信息。
- **`From` / `Into`**：用于类型之间的安全、可扩展转换。优先使用，更符合 Rust 习惯。
- **`TryFrom` / `TryInto`**：可能失败的转换，返回 `Result`，避免静默截断。

### 类型别名

```rust
type UserId = u64;
```

类型别名只是现有类型的同义词，不会创建新类型。它让代码意图更清晰。

### Never 类型 `!`

`!` 表示一个函数永远不会正常返回，例如 `panic!` 或无限循环。它在类型推断中非常有用。

### 动态分发 `dyn Trait`

```rust
fn draw(item: &dyn Drawable) { ... }
```

动态分发牺牲了一点运行时性能，换取了更灵活的多态。

### 类型转换选择指南 💡

| 场景 | 推荐方式 | 原因 |
|------|----------|------|
| 基本数值转换 | `as` | 简单、直接 |
| 自定义类型安全转换 | `From` / `Into` | 可扩展、符合 Rust 习惯 |
| 可能失败的转换 | `TryFrom` / `TryInto` | 避免静默截断 |
| 运行时多态 | `dyn Trait` | 灵活，但有一次间接开销 |

### 新类型模式（Newtype Pattern）

```rust
struct Meters(u32);
struct Kilometers(u32);
```

新类型模式比类型别名更安全：它创建了全新的类型，编译器会阻止无意识的混用。

## 常见误区 ⚠️

- **误区 1**：`as` 是最安全的转换方式。  
  ✅ 正解：`as` 可能导致数据截断，优先使用 `From`/`TryFrom`。
- **误区 2**：类型别名会创建新类型。  
  ✅ 正解：类型别名只是语法糖，不会增加类型安全。
- **误区 3**：`dyn Trait` 和 `impl Trait` 没有区别。  
  ✅ 正解：`impl Trait` 是编译期单态化，`dyn Trait` 是运行时动态分发。

## 一句话总结 🦀

> 善用 Rust 的类型转换工具，让类型安全从“编译通过”变成一种设计优势。

<RustPlayground />
