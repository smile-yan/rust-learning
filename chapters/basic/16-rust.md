---
title: "Rust 关键字速览"
module: "基础入门"
order: 16
code: |
  // 尽可能展示多个关键字的使用方式
  use std::fmt::Debug;

  // const 是编译期常量；static 是有固定内存地址的全局变量
  const GREETING: &str = "Rust";
  // static mut 是可变全局变量，读写都必须放在 unsafe 块中
  static mut COUNTER: usize = 0;

  // trait 定义一组能力（类似其他语言的接口）
  trait Speak {
      fn speak(&self) -> String;
  }

  // impl 某 trait for 某类型：为类型实现该能力
  struct Person;
  impl Speak for Person {
      fn speak(&self) -> String {
          String::from("Hello")
      }
  }

  // unsafe fn：调用者必须用 unsafe 块包裹，自行保证安全前提
  unsafe fn increment_counter() {
      COUNTER += 1;
  }

  fn main() {
      // loop 是无限循环，break 退出，continue 跳到下一轮
      let mut count = 0;
      loop {
          count += 1;
          if count == 2 {
              break;
          } else {
              continue;
          }
      }

      let person = Person;
      let speech = person.speak();

      // for / in / while
      let mut total = 0;
      for i in 0..5 {
          total += i;
      }
      while total > 0 {
          total -= 1;
      }

      // match / true / false
      let is_ready = true;
      match is_ready {
          true => println!("准备就绪"),
          false => println!("尚未就绪"),
      }

      // dyn trait 对象
      let speaker: &dyn Speak = &person;
      println!("{}", speaker.speak());

      // unsafe 调用
      unsafe {
          increment_counter();
          println!("COUNTER = {}", COUNTER);
      }

      println!("{}", GREETING);
  }
hint: "Rust 的关键字分为当前使用的和保留的。编写代码时尽量避免使用保留关键字作为标识符。"
exercises:
  - title: "使用 break 和 continue"
    description: "写一个 loop，当 count 等于 5 时用 break 退出，偶数时用 continue 跳过打印。"
    code_template: |
      fn main() {
          let mut count = 0;
          loop {
              count += 1;
              if count % 2 == 0 { continue; }
              if count > 5 { break; }
              println!("{}", count);
          }
      }
---

# Rust 关键字速览 📖

Rust 的关键字数量不多，但每一个都承担着重要的语义。理解它们是读懂 Rust 代码的基础。本文按用途把关键字分组，方便查阅和记忆。

## 🚦 控制流（10 个）

| 关键字 | 作用 |
|--------|------|
| `break` | 跳出循环或带标签的块 |
| `continue` | 跳过本次循环迭代 |
| `else` | `if` 的否定分支 |
| `for` | 遍历迭代器 |
| `if` | 条件分支 |
| `in` | `for` 循环的迭代绑定 |
| `loop` | 无限循环 |
| `match` | 模式匹配 |
| `while` | 条件循环 |
| `where` | 添加 trait bound 约束 |

## 🧬 类型与数据（15 个）

| 关键字 | 作用 |
|--------|------|
| `as` | 类型转换，或重命名导入 |
| `const` | 定义编译期常量 |
| `dyn` | 定义 trait 对象 `dyn Trait` |
| `enum` | 定义枚举类型 |
| `false` | 布尔值假 |
| `impl` | 实现方法或 trait |
| `let` | 变量绑定 |
| `mut` | 声明可变绑定或引用 |
| `ref` | 在模式匹配中按引用绑定 |
| `Self` | 指代实现所在的类型 |
| `self` | 指代当前实例 |
| `struct` | 定义结构体 |
| `trait` | 定义 trait（接口） |
| `true` | 布尔值真 |
| `type` | 类型别名或关联类型 |

## 📦 模块与可见性（6 个）

| 关键字 | 作用 |
|--------|------|
| `crate` | 指向当前 crate 根的路径 |
| `extern` | 声明外部函数或 crate |
| `mod` | 定义模块 |
| `pub` | 公开可见性 |
| `super` | 指向父模块的路径 |
| `use` | 引入路径到当前作用域 |

## ⚙️ 函数与生命周期（5 个）

| 关键字 | 作用 |
|--------|------|
| `fn` | 定义函数 |
| `move` | 转移所有权到闭包 |
| `return` | 从函数提前返回 |
| `static` | 静态变量或 `'static` 生命周期 |
| `unsafe` | 声明不安全代码块或函数 |

## ⚡ 异步编程（2 个）

| 关键字 | 作用 |
|--------|------|
| `async` | 定义异步函数或块 |
| `await` | 等待异步操作完成 |

## 🔒 保留关键字（未来可能使用）

以下关键字目前未被语言使用，但已被保留，不要用作标识符：

`abstract`, `become`, `box`, `do`, `final`, `macro`, `override`, `priv`, `try`, `typeof`, `unsized`, `virtual`, `yield`

## 一句话总结 ✅

Rust 的关键字体系简洁而严格，它们共同构成了所有权、借用、类型安全和并发安全的语法基础。

<RustPlayground />
