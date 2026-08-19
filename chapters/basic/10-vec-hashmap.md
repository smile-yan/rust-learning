---
title: "集合 Vec / HashMap"
module: "基础入门"
order: 10
code: |
  // 引入标准库中的哈希表类型
  use std::collections::HashMap;

  fn main() {
      // Vec
      // vec! 宏创建动态数组；声明 mut 后才能 push
      let mut v = vec![1, 2, 3];
      v.push(4);
      println!("Vec: {:?}", v);

      // iter() 产生迭代器，map 对每个元素做变换，
      // collect 把结果收集成新的 Vec
      let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
      println!("翻倍: {:?}", doubled);

      // filter 按条件保留元素；闭包参数是 &&i32，用 *x 解引用再取余
      let evens: Vec<&i32> = v.iter()
          .filter(|x| *x % 2 == 0)
          .collect();
      println!("偶数: {:?}", evens);

      // String
      // String 可增长：push_str 追加字符串，push 追加单个字符
      let mut s = String::from("hello");
      s.push_str(", world");
      s.push('!');
      println!("String: {}", s);

      // HashMap
      // insert 插入键值对，键和值的类型由第一次插入推断
      let mut scores = HashMap::new();
      scores.insert("Alice", 95);
      scores.insert("Bob", 87);

      // 遍历 &scores 只是借用，不会夺走 HashMap 的所有权
      for (name, score) in &scores {
          println!("{}: {}", name, score);
      }

      // get 返回 Option：找到是 Some(分数)，找不到是 None，不会 panic
      match scores.get("Alice") {
          Some(score) => println!("Alice 的分数: {}", score),
          None => println!("未找到"),
      }

      // entry API
      // or_insert：键不存在时才插入默认值，常用于初始化或计数
      scores.entry("Charlie").or_insert(78);
      println!("Charlie: {}", scores["Charlie"]);
  }
hint: "HashMap 的键值对遍历顺序不保证固定。entry API 可以避免重复插入或实现「不存在才插入」的逻辑。"
exercises:
  - title: "过滤偶数"
    description: "给定 vec![1,2,3,4,5,6]，用 filter 和 collect 收集所有偶数到新 Vec。"
    code_template: |
      fn main() {
          let nums = vec![1, 2, 3, 4, 5, 6];
          // let evens: Vec<i32> = ...
          println!("{:?}", evens);
      }
  - title: "统计分数"
    description: "用 HashMap 记录 Alice=90, Bob=85，查询 Alice 的分数并打印。"
    code_template: |
      use std::collections::HashMap;

      fn main() {
          let mut scores = HashMap::new();
          // 插入并查询
      }
---

# 集合 Vec / HashMap

Rust 标准库提供了丰富的集合类型，最常用的包括 `Vec<T>`、`HashMap<K, V>` 和 `String`。它们都存储在堆上，可以根据需要动态增长，是处理运行时数据的核心工具。

## 引入：动态数据的容器 📦

数组长度固定，很多时候不够用。集合类型提供了动态增长、灵活访问的能力，是实际开发中处理数据的主力。例如，读取文件内容时行数未知，就需要 `Vec<String>`；统计词频时就需要 `HashMap<String, u32>`。

![集合示意图](/images/module0-collections.svg)

## 概念图解 💡

```rust
Vec<T>        HashMap<K, V>        String
动态数组       键值对映射            UTF-8 可变字符串
自动扩容       平均 O(1) 查找        拥有所有权
```

## 深入讲解

### Vec

`Vec<T>` 是动态数组，可以动态扩容：

```rust
let mut v = vec![1, 2, 3];
v.push(4);
println!("{:?}", v);
```

`Vec` 支持索引访问、遍历、排序、切片等操作。要注意索引访问会 panic，安全访问用 `get`。

### HashMap

`HashMap<K, V>` 存储键值对：

```rust
let mut scores = HashMap::new();
scores.insert("Alice", 10);
scores.insert("Bob", 20);
```

获取值时用 `get`：

```rust
if let Some(score) = scores.get("Alice") {
    println!("{}", score);
}
```

### String

`String` 是可增长的、拥有所有权的 UTF-8 字符串，与字符串切片 `&str` 不同：

```rust
let mut s = String::from("hello");
s.push_str(" world");
```

`String` 的内容存储在堆上，可以被修改和增长；`&str` 通常是对 `String` 或字符串字面量的借用。

### 所有权与集合

集合拥有其元素的所有权。当你把值插入集合时，值会被移动到集合中。如果元素实现了 `Copy`，则是复制。

```rust
let s = String::from("a");
let mut v = Vec::new();
v.push(s);
// s 不能再使用
```

### 性能特点

- `Vec`：追加元素平均 O(1)，索引 O(1)，插入/删除中间 O(n)
- `HashMap`：查找、插入、删除平均 O(1)
- `String`：追加通常 O(1)，但重新分配时会触发扩容

## 常见误区 ⚠️

- 在遍历 Vec 时同时修改它，导致借用冲突。
- 用整数索引访问 HashMap：HashMap 没有固定顺序。
- 混淆 `String` 和 `&str` 的所有权语义。
- 忽略 `HashMap` 键需要实现 `Eq` 和 `Hash` trait。

## 一句话总结 ✅

`Vec`、`HashMap`、`String` 是 Rust 最常用的集合；它们都位于堆上，理解其所有权、借用和性能特点，是正确使用的关键。

<RustPlayground />
