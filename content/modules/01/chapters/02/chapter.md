---
title: Trait impl
hint: '&impl Summary 是 Trait Bound 的简写形式。默认实现可以减少重复代码，实现者可以选择覆盖。'
---

# Trait impl 🦀

Trait 定义了类型之间**共享的行为契约**。你可以把它理解为其他语言中的“接口”：它规定了一组方法，任何实现该 trait 的类型都必须提供这些方法。

## 从生活类比开始 💡

想象一个“可打印”的能力：无论是 PDF、图片还是网页，只要实现了“可打印” trait，打印机就能统一处理。Trait 让 Rust 能够以统一的方式操作不同的类型。

## 概念图解

![Trait 定义共享行为](images/module1-trait-interface.svg)

图中 `Summary` trait 定义了 `summarize()` 方法，`NewsArticle` 和 `Tweet` 都实现了它。这样，`notify(&impl Summary)` 就能接受任何实现 `Summary` 的类型。

![Trait Bounds 组合能力](images/module1-trait-bounds.svg)

通过 `T: Summary + Display`，我们可以要求类型同时满足多个 trait 的能力，实现精细的接口组合。

## 深入讲解

### 定义与实现

```rust
trait Summary {
    fn summarize(&self) -> String;
}
```

为具体类型实现 trait 时，需要实现所有没有默认实现的方法。

### 默认实现

Trait 方法可以提供默认实现，实现者可以选择覆盖：

```rust
fn summarize(&self) -> String {
    format!("(阅读更多...)")
}
```

### Trait Bound

```rust
fn notify<T: Summary>(item: &T) { ... }
fn notify(item: &impl Summary) { ... } // 语法糖
```

### 高级用法

- **Trait Object**：`Box<dyn Summary>` 实现运行时多态。
- **关联类型**：在 trait 中定义 `type Item;`，实现者指定具体类型。
- **Supertrait**：`trait Printable: Summary {}` 要求实现 `Printable` 必须先实现 `Summary`。

## 常见误区 ⚠️

- **误区 1**：`&impl Trait` 和 `dyn Trait` 是一样的。  
  ✅ 正解：`&impl Trait` 是编译期单态化，`dyn Trait` 是运行时动态分发。
- **误区 2**：可以为任何类型实现任何 trait。  
  ✅ 正解：必须满足“孤儿规则”——trait 或类型至少有一个来自当前 crate。
- **误区 3**：trait 可以有字段。  
  ✅ 正解：Rust trait 只能定义方法和关联类型，不能包含字段。

## 一句话总结 🦀

> Trait 是 Rust 多态的基石：它定义“能做什么”，而具体类型决定“怎么做”。

