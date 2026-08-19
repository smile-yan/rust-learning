---
title: "控制流 if / loop / for"
module: "基础入门"
order: 5
code: |
  fn main() {
      let number = 7;
      // if 的条件必须是 bool，Rust 不会把数字自动当作真值
      if number % 2 == 0 {
          println!("偶数");
      } else {
          println!("奇数");
      }

      // loop 可以返回值
      // break 后面跟的值就是整个 loop 表达式的结果
      let mut count = 0;
      let result = loop {
          count += 1;
          if count >= 3 {
              break count * 2;
          }
      };
      println!("loop 结果: {}", result);

      // while 循环
      let mut n = 3;
      while n > 0 {
          println!("while: {}", n);
          n -= 1;
      }

      // for 循环遍历范围
      // 1..4 不包含 4，即打印 1、2、3
      for i in 1..4 {
          println!("for: {}", i);
      }

      // for 循环遍历集合并获取索引
      // enumerate() 让每次迭代同时拿到 (索引, 值)
      let arr = [10, 20, 30];
      for (idx, val) in arr.iter().enumerate() {
          println!("arr[{}] = {}", idx, val);
      }

      // if let 简化模式匹配
      // 只关心 Some 这一种情况时，比写完整 match 更简洁
      let some_value = Some(5);
      if let Some(v) = some_value {
          println!("if let 匹配到: {}", v);
      }
  }
hint: "1..4 是左闭右开区间，包含 1、2、3。Rust 没有隐式类型转换，if 条件必须是 bool。"
exercises:
  - title: "判断闰年"
    description: "给定年份 year，判断是否为闰年（能被 4 整除且不能被 100 整除，或能被 400 整除）。"
    code_template: |
      fn main() {
          let year = 2024;
          // 使用 if/else 判断
      }
  - title: "1 到 100 求和"
    description: "用 for 循环计算 1 到 100 的累加和并打印。"
    code_template: |
      fn main() {
          let mut sum = 0;
          // for i in 1..=100 { ... }
          println!("{}", sum);
      }
---

# 控制流 if / loop / for

控制流决定程序的执行路径。Rust 提供了 `if`、`loop`、`while`、`for` 和 `match` 等控制结构，而且 `if` 和 `match` 都是**表达式**，可以直接赋值给变量。这种设计让 Rust 代码在表达分支逻辑时既安全又优雅。

## 引入：程序就像一条河 🌊

没有控制流的程序只会从上到下顺序执行。控制流让河水可以分流、循环、跳转，从而表达复杂的逻辑。Rust 的控制流强调显式和穷尽，避免隐藏的执行路径。

![控制流示意图](/images/module0-control-flow.svg)

## 概念图解 💡

```rust
let number = 6;
let result = if number % 2 == 0 {
    "even"
} else {
    "odd"
};
```

## 深入讲解

### if / else

`if` 的条件必须是 `bool` 类型，Rust 不会隐式转换数字为布尔值。这是 Rust 安全性的体现。

```rust
if x != 0 {  // 正确
    println!("x is not zero");
}
```

### loop

`loop` 是无限循环，可以用 `break value` 返回一个值：

```rust
let result = loop {
    break 10;
};
```

### while

`while` 在条件为真时持续循环，适合条件不明确的循环。

### for

`for` 配合迭代器使用，是 Rust 中最常用、最安全的循环方式：

```rust
for i in 1..=5 {
    println!("{}", i);
}
```

### match

`match` 是 Rust 模式匹配的入门，后面的章节会深入讲解。它的一个重要特点是**穷尽性**：必须覆盖所有可能的情况。

## 常见误区 ⚠️

- `if x { ... }`：Rust 要求条件必须是 `bool`。
- `if` 分支返回不同类型：作为表达式时，所有分支必须返回同一类型。
- 忘记 `match` 的穷尽性处理。
- 在 `for` 循环中修改被遍历的集合。

## 一句话总结 ✅

Rust 控制流丰富而安全；`if` 和 `match` 是表达式，循环可以返回值，善用 `for` 配合范围或迭代器。

<RustPlayground />
