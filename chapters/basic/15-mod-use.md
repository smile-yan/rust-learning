---
title: "模块与包管理 mod / use"
module: "基础入门"
order: 15
code: |
  // 这个文件演示了模块系统的核心语法。
  // 在真实项目中，这些模块通常分别放在
  // my_module.rs 或 my_module/mod.rs 中。

  // mod 定义模块；模块内的项默认私有，加 pub 才对外可见
  mod my_module {
      pub fn public_fn() {
          println!("这是公共函数");
      }

      // 没有 pub，只能在 my_module 内部调用
      fn _private_fn() {
          println!("这是私有函数");
      }

      // 模块可以嵌套，内层模块同样用 pub 控制可见性
      pub mod nested {
          pub fn greet() {
              println!("来自嵌套模块的问候");
          }
      }
  }

  mod utils {
      pub fn add(a: i32, b: i32) -> i32 {
          a + b
      }
  }

  // use 把深层路径引入当前作用域，之后可直接写 greet()
  use my_module::nested::greet;

  fn main() {
      my_module::public_fn();
      greet();

      let result = utils::add(2, 3);
      println!("utils::add(2, 3) = {}", result);

      // self 指当前模块，crate 指当前 crate 根
      let _local = self::utils::add(1, 1);
      println!("通过 self 调用: {}", _local);
  }
hint: "默认情况下模块内的项都是私有的，需要用 pub 才能被外部访问。子模块可以访问父模块的私有项。"
exercises:
  - title: "创建简单模块"
    description: "定义 math 模块，包含 pub fn add 和 pub fn sub，在 main 中调用。"
    code_template: |
      mod math {
          pub fn add(a: i32, b: i32) -> i32 { a + b }
          pub fn sub(a: i32, b: i32) -> i32 { a - b }
      }

      fn main() {
          println!("{}", math::add(2, 3));
      }
---

# 模块与包管理 mod / use

随着项目变大，把所有代码放在一个文件里会变得难以维护。Rust 提供了模块系统，让你可以把代码拆分成多个文件、多个模块，并精确控制可见性。模块系统是 Rust 工程化开发的基石。

## 引入：整理你的工具箱 🧰

一个木匠不会把所有工具都堆在地上，而是分门别类放进抽屉。Rust 的模块系统就是你的代码抽屉。

![模块与包管理示意图](/images/module0-modules.svg)

## 概念图解 💡

```rust
crate 根
├── mod front_of_house;
│   ├── mod hosting;
│   │   └── fn add_to_waitlist()
│   └── mod serving;
└── mod back_of_house;
```

## 深入讲解

### crate

crate 是 Rust 编译的最小单元：

- 二进制 crate：有 `main.rs`，编译成可执行文件
- 库 crate：有 `lib.rs`，编译成库，供其他代码使用

### 模块

使用 `mod` 关键字声明模块：

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}
```

### 可见性

- 默认私有：模块内的项默认只能在本模块及子模块访问
- `pub`：公开给外部使用
- `pub(crate)`：只在当前 crate 内公开
- `pub(super)`：只在父模块公开

### use 关键字

`use` 创建快捷方式，避免写长路径：

```rust
use crate::front_of_house::hosting;
hosting::add_to_waitlist();
```

### 文件组织

模块可以内联在文件中，也可以放到单独文件：

```rust
// src/front_of_house.rs
pub mod hosting;
```

## 常见误区 ⚠️

- 以为模块声明 `mod xxx;` 和 `use xxx;` 是一回事。
- 忘记加 `pub` 导致外部无法访问。
- 文件路径和模块路径不一致，导致编译错误。
- 过度公开内部实现，破坏封装。

## 一句话总结 ✅

Rust 模块系统通过 crate、module、`pub` 和 `use` 组织代码；默认私有、显式公开，既保护实现细节，又提供清晰接口。

<RustPlayground />
