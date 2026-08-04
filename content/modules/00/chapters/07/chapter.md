---
title: 结构体 struct
hint: '#[derive(Debug)] 让我们可以用 {:?} 打印结构体。impl 块可以定义多个方法和关联函数。'
---

# 结构体 struct

结构体（struct）是 Rust 中创建自定义复合类型的主要方式。它把多个相关的数据字段打包成一个有意义的整体，是面向数据编程的基石。在 Rust 中，结构体不仅可以携带数据，还可以为其定义方法。

## 引入：把零散信息组装成卡片 🪪

想象你要描述一个用户：有用户名、邮箱、是否激活。用单独变量会很难管理，结构体就像一张用户卡片，把所有字段整合在一起，并且可以通过方法定义卡片的行为。

![结构体示意图](images/module0-structs.svg)

## 概念图解 💡

```rust
struct User {
    username: String,
    email: String,
    active: bool,
}
```

## 深入讲解

### 定义结构体

使用 `struct` 关键字，后跟结构体名和花括号中的字段列表。每个字段都需要名字和类型。

### 实例化

```rust
let user = User {
    username: String::from("alice"),
    email: String::from("alice@example.com"),
    active: true,
};
```

### 访问字段

使用点号访问：`user.username`。

### 更新语法

```rust
let user2 = User {
    email: String::from("bob@example.com"),
    ..user  // 其余字段从 user 复制/移动
};
```

注意 `..user` 会移动那些没有实现 `Copy` 的字段，例如 `String`。如果 `user` 字段被移动走，`user` 将不能再使用。

### 元组结构体

当你想给元组一个名字，但又不需要命名字段时，使用元组结构体：

```rust
struct Color(i32, i32, i32);
let black = Color(0, 0, 0);
```

可以通过 `.0`、`.1`、`.2` 访问。

### 单元结构体

```rust
struct AlwaysEqual;
```

没有字段，常用于实现 trait 或作为标记类型。

### 为结构体定义方法

使用 `impl` 块：

```rust
impl User {
    fn can_login(&self) -> bool {
        self.active
    }
}
```

`self`、`&self`、`mut self` 分别表示获取所有权、不可变借用、可变借用。

## 常见误区 ⚠️

- 忘记字段类型标注。
- 使用 `..user` 时没注意所有权移动。
- 混淆普通结构体、元组结构体和单元结构体的使用场景。
- 在 `impl` 块外调用方法：方法必须通过结构体实例或类型本身调用。

## 一句话总结 ✅

结构体把相关字段打包成自定义类型；结合 `impl` 定义方法，普通结构体、元组结构体、单元结构体覆盖不同场景。

