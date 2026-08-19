---
title: "枚举与 Option Some / None"
module: "基础入门"
order: 14
code: |
  // 枚举的每个变体可以携带不同类型、不同数量的数据
  enum Message {
      Quit,
      Move { x: i32, y: i32 },
      Write(String),
      ChangeColor(u8, u8, u8),
  }

  fn main() {
      let msg = Message::Move { x: 10, y: 20 };

      // match 必须覆盖所有变体；每个分支可以解构出变体携带的数据
      match msg {
          Message::Quit => println!("退出"),
          Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
          Message::Write(text) => println!("写入: {}", text),
          Message::ChangeColor(r, g, b) => {
              println!("颜色: RGB({}, {}, {})", r, g, b);
          },
      }

      // ---- Option<T>：表示"可能有值" ----
      // 只有 Some(值) 和 None 两种

      // if let 是只关心某一种匹配时的简洁写法
      let some_number = Some(5);
      if let Some(n) = some_number {
          println!("值是: {}", n);
      }

      // None 表示没有值；类型标注不可少，编译器无法推断
      let absent: Option<i32> = None;
      println!("absent: {:?}", absent);

      // if n > 5 是匹配守卫，为分支附加额外条件
      let val = Some(7);
      match val {
          Some(n) if n > 5 => println!("大于 5: {}", n),
          Some(_) => println!("不大于 5"),
          None => println!("没有值"),
      }

      // unwrap_or：有值则取出，是 None 时返回给定的默认值
      let score: Option<u32> = Some(85);
      let final_score = score.unwrap_or(60);
      println!("最终分数: {}", final_score);
  }
hint: "match 的分支必须穷举所有变体；使用 if let 可以简化只关心单一模式的场景。Option 强制处理 None 情况。"
exercises:
  - title: "用 unwrap_or 给默认值"
    description: "定义 score: Option<u32> = None，用 unwrap_or 得到默认值 60。"
    code_template: |
      fn main() {
          let score: Option<u32> = None;
          // let final_score = ...
          println!("{}", final_score);
      }
  - title: "Option 的 map 练习"
    description: "对 Some(5) 使用 map 使其值翻倍。"
    code_template: |
      fn main() {
          let x = Some(5);
          // let y = x.map(...);
          println!("{:?}", y);
      }
---

# 枚举与 Option Some / None

`Option<T>` 是 Rust 标准库中最重要的枚举之一，它用显式的方式表达“一个值可能存在，也可能不存在”。这是 Rust 避免 null 指针问题的核心设计，也是编写安全代码的 daily tool。

## 引入：没有 null 的世界 🚫

很多语言用 `null` 表示“没有值”，但 null 引用被称为“十亿美元错误”。Rust 选择用 `Option<T>` 强制你处理“无值”的情况，把潜在的空指针错误消灭在编译期。

## 概念图解 💡

```rust
enum Option<T> {
    Some(T),
    None,
}
```

## 深入讲解

### 为什么需要 Option

```rust
let x: Option<i32> = Some(5);
let y: Option<i32> = None;
```

`Option<i32>` 和 `i32` 是不同类型，不能直接把 `Option<i32>` 当 `i32` 用。这就迫使你显式处理 None 的情况，而不是在运行时突然发现空值。

### 常用方法

- `is_some()` / `is_none()`：判断是否有值
- `unwrap()`：有值返回值，None 则 panic
- `expect()`：类似 unwrap，可自定义信息
- `map()`：对 Some 中的值进行转换
- `and_then()` / `or()` / `unwrap_or()`：组合 Option

```rust
let maybe_num = Some(5);
let doubled = maybe_num.map(|n| n * 2);  // Some(10)
```

### match Option

```rust
match some_option {
    Some(value) => println!("{}", value),
    None => println!("没有值"),
}
```

### if let / while let

对于只关心 Some 的情况，可以用更简洁的语法：

```rust
if let Some(v) = some_option {
    println!("{}", v);
}
```

### Option 与 Result 的区别

- `Option<T>`：值可能有也可能没有
- `Result<T, E>`：操作可能成功也可能失败

很多标准库方法同时返回 `Option` 和 `Result` 版本，例如 `parse` 返回 `Result`，而 `find` 返回 `Option`。

## 常见误区 ⚠️

- 滥用 `unwrap()` 导致 panic。
- 把 `Option<T>` 直接当 `T` 用：需要显式解包。
- 忘记 `Option` 和 `Result` 虽然相似，但语义不同。
- 没有处理 `None` 分支，编译器会提醒你。

## 一句话总结 ✅

`Option<T>` 用类型系统消灭 null，强制你处理有值/无值两种情况，是 Rust 安全性的重要基石。

<RustPlayground />
