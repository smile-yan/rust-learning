---
title: "切片 slice"
module: "基础入门"
order: 12
code: |
  // 返回字符串切片 &str：借用输入字符串的一部分，不产生新的 String
  fn first_word(s: &str) -> &str {
      // as_bytes 把字符串按字节查看，方便逐字节查找空格
      let bytes = s.as_bytes();

      // enumerate() 同时给出下标和元素；&item 通过模式解构取出字节值
      for (i, &item) in bytes.iter().enumerate() {
          // b' ' 是字节字面量；遇到空格就返回开头到下标 i 的切片
          if item == b' ' {
              return &s[..i];
          }
      }

      // 没找到空格就返回整个字符串的切片
      &s[..]
  }

  // 参数 &[i32] 是切片：数组和 Vec 都能传入，比固定数组更通用
  fn sum_slice(nums: &[i32]) -> i32 {
      let mut total = 0;
      for n in nums {
          total += n;
      }
      total
  }

  fn main() {
      let text = String::from("hello world");
      // &String 会自动转换成 &str（deref 强制转换）
      let word = first_word(&text);
      println!("第一个单词: {}", word);

      let arr = [10, 20, 30, 40, 50];
      // arr[1..4] 取下标 1 到 3 的元素，含头不含尾
      println!("arr[1..4] 的和: {}", sum_slice(&arr[1..4]));
      // &arr 整个数组也能直接当切片传入
      println!("arr 全部元素的和: {}", sum_slice(&arr));

      // Vec 同样可以切片后传给同一个函数
      let v = vec![1, 2, 3, 4, 5];
      println!("Vec 前三个元素的和: {}", sum_slice(&v[..3]));

      // 字符串字面量本身就是 &str
      let literal: &str = "Rust 编程";
      println!("字符串切片: {}", literal);
  }
hint: "字符串切片按字节索引，中文字符每个占 3 个字节，切分时需要注意边界。&[T] 可以引用数组或 Vec。"
exercises:
  - title: "取前 N 个字符"
    description: "编写函数 first_n(s: &str, n: usize) -> &str，返回字符串前 n 个字符（按字节）。"
    code_template: |
      fn first_n(s: &str, n: usize) -> &str {
          &s[..n]
      }

      fn main() {
          println!("{}", first_n("Hello Rust", 5));
      }
  - title: "数组切片求和"
    description: "编写函数 sum_slice(nums: &[i32]) -> i32，并调用它计算数组子区间的和。"
    code_template: |
      fn sum_slice(nums: &[i32]) -> i32 {
          // 补全
      }

      fn main() {
          let arr = [1, 2, 3, 4, 5];
          println!("{}", sum_slice(&arr[1..4]));
      }
---

# 切片 slice

切片（slice）是对集合中一段连续元素的引用。它不拥有数据，只是提供了一种“视图”。切片是 Rust 中处理字符串和数组子集的核心工具，也是函数参数设计中常用的类型。

## 引入：看一本书的某一页 📖

如果你有一整本书（数组/Vec/String），切片就像其中连续的几页。你不会复制内容，只是标记从哪到哪。切片让你可以安全地借用一部分数据，而不用复制它。

![切片示意图](/images/module0-slices.svg)

## 概念图解 💡

```rust
let arr = [10, 20, 30, 40, 50];
let s = &arr[1..4];  // [20, 30, 40]
```

切片包含两个信息：指向首元素的指针和长度。

## 深入讲解

### 数组切片

```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
```

### 字符串切片

字符串切片 `&str` 是最常用的字符串类型之一：

```rust
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
```

很多函数接受 `&str` 而不是 `String`，因为 `&str` 更通用，可以传入字符串字面量或 `String` 的引用。

### 范围语法

- `[start..end]`：包含 start，不包含 end
- `[start..=end]`：包含 start 和 end
- `[..end]`：从开头到 end
- `[start..]`：从 start 到末尾
- `[..]`：整个集合的切片

### 切片与所有权

切片是引用，因此不会转移所有权。但切片的有效期不能超过原数据的有效期。Rust 编译器会检查这一点，防止悬空引用。

### 切片作为函数参数

使用切片参数让函数更通用：

```rust
fn first_word(s: &str) -> &str {
    // ...
}
```

这个函数既可以接收 `String`，也可以接收字符串字面量。

## 常见误区 ⚠️

- 切片越界：`&arr[0..10]` 会在运行时 panic。
- 字符串切片不在 UTF-8 字符边界：例如对 `"你好"` 切 `&s[0..1]` 会 panic。
- 误以为切片拥有数据：切片只是引用视图。
- 返回局部变量的切片：会导致悬空引用，编译器会报错。

## 一句话总结 ✅

切片是对连续数据的引用视图，不拥有数据；使用范围语法创建，字符串切片必须位于 UTF-8 边界，是编写通用函数的好帮手。

<RustPlayground />
