---
title: "Trait impl"
module: "中等应用"
order: 2
code: |
  // trait 定义一组共享行为：summarize_author 是没有默认体的必需方法
  pub trait Summary {
      fn summarize_author(&self) -> String;

      // 带默认实现的方法：实现者可直接复用，也可以覆盖
      fn summarize(&self) -> String {
          format!(
              "(阅读更多来自 {} 的内容...)",
              self.summarize_author()
          )
      }
  }

  pub struct NewsArticle {
      pub headline: String,
      pub location: String,
      pub author: String,
  }

  // 为 NewsArticle 实现 trait，并覆盖了默认的 summarize
  impl Summary for NewsArticle {
      fn summarize_author(&self) -> String {
          self.author.clone()
      }

      fn summarize(&self) -> String {
          format!(
              "{} - {} (作者: {})",
              self.headline,
              self.location,
              self.author
          )
      }
  }

  pub struct Tweet {
      pub username: String,
      pub content: String,
  }

  // Tweet 只实现必需方法，summarize 沿用 trait 的默认实现
  impl Summary for Tweet {
      fn summarize_author(&self) -> String {
          format!("@{}", self.username)
      }
  }

  // impl Trait 作参数：接受任何实现了 Summary 的类型，
  // 是「泛型 + trait 约束」的语法糖
  fn notify(item: &impl Summary) {
      println!("突发新闻! {}", item.summarize());
  }

  fn main() {
      let article = NewsArticle {
          headline: String::from("Rust 1.80 发布"),
          location: String::from("全球"),
          author: String::from("Rust 团队"),
      };
      // 这里调用的是覆盖后的 summarize
      notify(&article);

      let tweet = Tweet {
          username: String::from("rustlang"),
          content: String::from("Rust 越来越好用了！"),
      };
      // 这里调用的是 trait 默认实现的 summarize
      notify(&tweet);
  }
hint: "&impl Summary 是 Trait Bound 的简写形式。默认实现可以减少重复代码，实现者可以选择覆盖。"
exercises:
  - title: "为自定义类型实现 Display"
    description: "为 struct Person { name: String } 实现 std::fmt::Display。"
    code_template: |
      use std::fmt;

      struct Person {
          name: String,
      }

      impl fmt::Display for Person {
          fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
              // write!(f, "...")
          }
      }

      fn main() {
          let p = Person { name: String::from("Alice") };
          println!("{}", p);
      }
  - title: "Greetable trait"
    description: "定义 Greetable trait 含 greet 方法，为 Person 实现。"
    code_template: |
      trait Greetable {
          fn greet(&self);
      }

      struct Person { name: String }

      impl Greetable for Person {
          fn greet(&self) {
              println!("Hello, {}!", self.name);
          }
      }

      fn main() {
          Person { name: String::from("Bob") }.greet();
      }
---

# Trait impl 🦀

Trait 定义了类型之间**共享的行为契约**。你可以把它理解为其他语言中的“接口”：它规定了一组方法，任何实现该 trait 的类型都必须提供这些方法。

## 从生活类比开始 💡

想象一个“可打印”的能力：无论是 PDF、图片还是网页，只要实现了“可打印” trait，打印机就能统一处理。Trait 让 Rust 能够以统一的方式操作不同的类型。

## 概念图解

![Trait 定义共享行为](/images/module1-trait-interface.svg)

图中 `Summary` trait 定义了 `summarize()` 方法，`NewsArticle` 和 `Tweet` 都实现了它。这样，`notify(&impl Summary)` 就能接受任何实现 `Summary` 的类型。

![Trait Bounds 组合能力](/images/module1-trait-bounds.svg)

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

<RustPlayground />
