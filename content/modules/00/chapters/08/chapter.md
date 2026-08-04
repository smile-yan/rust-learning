---
title: 枚举与模式匹配 enum / match
hint: match 的分支必须穷举所有变体。使用 if let 可以简化只关心单一模式的场景。
---

# 枚举与模式匹配 enum / match

枚举（enum）让某个类型可以拥有若干**互斥的变体**，每个变体可以携带不同的数据。配合 `match` 模式匹配，Rust 可以优雅、安全地处理多分支逻辑。枚举和模式匹配是 Rust 类型系统表达能力的集中体现。

## 引入：一个变量，多种可能 🎭

假设你在做一个消息系统，消息可能是：退出、移动坐标、发送文字、改变颜色。用枚举可以把这些不同类型但语义相关的值统一成 `Message` 类型。

![枚举与模式匹配示意图](images/module0-enums.svg?v=2)

## 概念图解 💡

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

## 深入讲解

### 枚举定义

枚举的每个变体可以：

- 不带数据，如 `Quit`
- 带命名字段，如 `Move { x, y }`
- 带单个值，如 `Write(String)`
- 带多个值，如 `ChangeColor(r, g, b)`

### 模式匹配

```rust
match msg {
    Message::Quit => println!("退出"),
    Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
    Message::Write(text) => println!("文字: {}", text),
    Message::ChangeColor(r, g, b) => println!("颜色 RGB({}, {}, {})", r, g, b),
}
```

### 穷尽性检查

Rust 要求 `match` 覆盖枚举所有变体。如果你漏了某个变体，编译器会报错。这避免了很多遗漏分支的 bug。

### if let / while let

对于只关心一个变体的情况，可以用更简洁的语法：

```rust
if let Message::Write(text) = msg {
    println!("{}", text);
}
```

## 常见误区 ⚠️

- 忘记 `match` 的穷尽性，漏处理变体。
- 模式变量命名和外部变量冲突。
- 混淆枚举变体和结构体字段语法。
- 在 `match` 中忘记加 `=` 或 `=>`。

## 一句话总结 ✅

枚举让“一个类型的多种形态”在类型系统中显式表达，配合穷尽性 `match` 检查，能写出既灵活又安全的分支逻辑。

