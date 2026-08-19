---
title: "结构体 struct"
module: "基础入门"
order: 7
code: |
  // derive 宏让编译器自动实现常用 trait：
  // Debug 用于 {:?} 打印，Clone 用于复制，PartialEq 用于比较
  #[derive(Debug, Clone, PartialEq)]
  struct Rectangle {
      width: u32,
      height: u32,
  }

  // impl 块中为结构体定义方法和关联函数
  impl Rectangle {
      // 关联函数，常用于构造
      // 没有 self 参数，调用时用 类型名::函数名
      fn square(size: u32) -> Self {
          Self { width: size, height: size }
      }

      // 方法
      // 第一个参数 &self 表示只读借用调用者
      fn area(&self) -> u32 {
          self.width * self.height
      }

      fn can_hold(&self, other: &Rectangle) -> bool {
          self.width > other.width && self.height > other.height
      }

      // 可变方法示例
      // &mut self 才能修改字段，且要求调用者用 mut 声明
      fn scale(&mut self, factor: u32) {
          self.width *= factor;
          self.height *= factor;
      }
  }

  fn main() {
      let mut rect1 = Rectangle { width: 30, height: 50 };
      // {:?} 能打印全靠上面 derive 出的 Debug
      println!("矩形: {:?}", rect1);
      println!("矩形面积: {}", rect1.area());

      rect1.scale(2);
      println!("放大后: {:?}, 面积: {}", rect1, rect1.area());

      let rect2 = Rectangle { width: 10, height: 40 };
      println!("rect1 能容纳 rect2? {}", rect1.can_hold(&rect2));

      // :: 语法调用关联函数，相当于其他语言的"静态工厂方法"
      let sq = Rectangle::square(20);
      println!("正方形: {:?}, 面积: {}", sq, sq.area());
  }
hint: "#[derive(Debug)] 让我们可以用 {:?} 打印结构体。impl 块可以定义多个方法和关联函数。"
exercises:
  - title: "定义矩形并计算面积"
    description: "定义 Rectangle { width: u32, height: u32 }，并实现 area 方法。"
    code_template: |
      struct Rectangle {
          width: u32,
          height: u32,
      }

      impl Rectangle {
          fn area(&self) -> u32 {
              // 补全
          }
      }

      fn main() {
          let r = Rectangle { width: 10, height: 20 };
          println!("{}", r.area());
      }
  - title: "关联函数构造正方形"
    description: "为 Rectangle 添加 square(size: u32) 关联函数，返回正方形。"
    code_template: |
      impl Rectangle {
          fn square(size: u32) -> Self {
              // 补全
          }
      }

      fn main() {
          let s = Rectangle::square(5);
          println!("{}", s.area());
      }
---

# 结构体 struct

结构体（struct）是 Rust 中创建自定义复合类型的主要方式。它把多个相关的数据字段打包成一个有意义的整体，是面向数据编程的基石。在 Rust 中，结构体不仅可以携带数据，还可以为其定义方法。

## 引入：把零散信息组装成卡片 🪪

想象你要描述一个用户：有用户名、邮箱、是否激活。用单独变量会很难管理，结构体就像一张用户卡片，把所有字段整合在一起，并且可以通过方法定义卡片的行为。

![结构体示意图](/images/module0-structs.svg)

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

<RustPlayground />
