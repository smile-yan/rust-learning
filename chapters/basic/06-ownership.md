---
title: "所有权 Ownership"
module: "基础入门"
order: 6
code: |
  // ---- 所有权与借用：引用不拿走所有权 ----
  fn main() {
      // 不可变借用
      // &s1 传的是引用，所有权不转移，后面还能继续用 s1
      let s1 = String::from("hello");
      let len = calculate_length(&s1);
      println!("'{}' 的长度是 {}。", s1, len);

      // 可变借用
      // 同一作用域内可变借用最多一个，且不能和不可变借用共存
      let mut s2 = String::from("hello");
      change(&mut s2);
      println!("修改后: {}", s2);

      // 所有权转移与 clone
      // 若写 let s3 = s1 会移动所有权使 s1 失效；clone 是显式深拷贝
      let s3 = s1.clone();
      println!("s1 = {}, s3 = {}", s1, s3);

      // 切片不拥有数据
      // &text[..5] 等价于 &text[0..5]，只是对原字符串的借用
      let text = String::from("hello world");
      let first = &text[..5];
      let second = &text[6..];
      println!("{} | {}", first, second);
  }

  // 参数 &String 表示借用，函数用完不释放原数据
  fn calculate_length(s: &String) -> usize {
      s.len()
  }

  // &mut String 表示可变借用，允许在函数内修改内容
  fn change(s: &mut String) {
      s.push_str(", world");
  }
hint: "如果不使用 clone()，String 会被移动，之后不能再使用 s1。借用规则由编译器在编译期检查。"
exercises:
  - title: "借用计算长度"
    description: "定义 String s，编写一个函数接收 &String 返回长度，调用后仍可使用 s。"
    code_template: |
      fn len(s: &String) -> usize {
          s.len()
      }

      fn main() {
          let s = String::from("Rust");
          // 调用 len 并打印，s 之后还能使用
      }
  - title: "可变借用拼接字符串"
    description: "定义 mut String，用可变借用给它追加内容，最后打印。"
    code_template: |
      fn append(s: &mut String) {
          s.push_str("!");
      }

      fn main() {
          let mut s = String::from("Hello");
          // 调用 append
          println!("{}", s);
      }
---

# 所有权 Ownership

所有权是 Rust 最核心的特性，没有之一。它让 Rust 在没有垃圾回收器（GC）的情况下，保证内存安全。理解所有权，是掌握 Rust 的关键一步。几乎所有 Rust 编译错误，归根结底都和所有权、借用、生命周期有关。

## 引入：谁对这块内存负责？🤔

在传统语言中，内存管理常见两种方式：

- 手动管理（C/C++）：容易内存泄漏、双重释放、悬空指针。
- 垃圾回收（Java/Go/Python）：运行时开销，暂停不可控。

Rust 提出了一种新思路：**每个值都有且只有一个所有者**，所有者离开作用域，值就被释放。

![所有权三规则示意图](/images/module0-ownership.svg)

## 概念图解 💡

所有权三规则：

1. 🦀 每个值都有一个所有者。
2. 🦀 同一时刻只有一个所有者。
3. 🦀 所有者离开作用域，值被释放。

## 深入讲解

### 变量作用域

```rust
{
    let s = String::from("hello");  // s 在此有效
} // s 离开作用域，内存被释放
```

### 移动（Move）

```rust
let s1 = String::from("hello");
let s2 = s1;          // s1 的所有权移动到 s2
// println!("{}", s1); // 错误！s1 不再有效
```

String 存储在堆上，移动时只复制栈上的元数据，把所有权转移给新变量。这避免了双重释放。

### 克隆（Clone）

如果需要真正的深拷贝，显式调用 `.clone()`：

```rust
let s1 = String::from("hello");
let s2 = s1.clone();
```

### 拷贝（Copy）trait

栈上的简单类型（如 `i32`、`bool`、`char`）实现了 `Copy` trait，赋值时会自动复制，不会发生移动。

### 借用

借用让多个地方可以读取数据而不转移所有权：

```rust
let s = String::from("hello");
let r1 = &s;
let r2 = &s;
```

## 常见误区 ⚠️

- 以为 `let s2 = s1;` 后 `s1` 还能用：对于堆上类型，这是移动。
- 过度使用 `.clone()`：虽然安全，但会带来性能开销。
- 混淆 `Copy` 和 `Clone`：`Copy` 是隐式的栈复制，`Clone` 是显式的深拷贝。
- 同时拥有可变引用和不可变引用。

## 一句话总结 ✅

Rust 通过所有权系统在编译期保证内存安全：每个值有唯一所有者，所有权移动避免双重释放，借用机制让多访问成为可能。

<RustPlayground />
