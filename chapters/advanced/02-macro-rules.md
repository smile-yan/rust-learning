---
title: "宏编程 macro_rules!"
module: "高级应用"
order: 2
code: |
  // macro_rules! 定义声明宏：按模式匹配输入，再展开为代码
  macro_rules! say_hello {
      // 无参数模式：匹配 say_hello!() 这种空调用
      () => {
          println!("Hello!");
      };
  }

  // $func_name:ident 捕获一个标识符（这里是函数名）
  macro_rules! create_function {
      ($func_name:ident) => {
          // 宏展开后生成一个名为 $func_name 的函数
          fn $func_name() {
              // stringify! 把捕获的代码原样转成字符串
              println!("调用了 {:?}", stringify!($func_name));
          }
      };
  }

  // $expression:expr 捕获任意一个表达式
  macro_rules! print_result {
      ($expression:expr) => {
          println!(
              "{:?} = {:?}",
              stringify!($expression),
              $expression
          );
      };
  }

  // $(...),* 重复匹配：零个或多个表达式，以逗号分隔
  macro_rules! vec_of_strings {
      ($($x:expr),*) => {
          {
              let mut temp_vec = Vec::new();
              // 对每个捕获到的表达式重复执行一次 push
              $(
                  temp_vec.push($x.to_string());
              )*
              // 末尾表达式作为宏展开后的结果值
              temp_vec
          }
      };
  }

  // 调用宏在编译期展开，实际生成 foo 和 bar 两个函数
  create_function!(foo);
  create_function!(bar);

  fn main() {
      say_hello!();
      foo();
      bar();
      print_result!(1u32 + 1);
      print_result!({
          let x = 2u32;
          x * x
      });

      let strings = vec_of_strings!["a", "b", "c"];
      println!("{:?}", strings);
  }
hint: "声明宏在调用处展开，注意避免命名冲突。重复模式 $()* 可以匹配零个或多个重复内容。"
exercises:
  - title: "定义 say! 宏"
    description: "定义一个 say! 宏，接收一个表达式并打印它及其结果。"
    code_template: |
      macro_rules! say {
          ($e:expr) => {
              println!("{} = {}", stringify!($e), $e);
          };
      }

      fn main() {
          say!(1 + 2);
      }
---

# 🔧 宏编程 macro_rules!

Rust 的宏系统让代码在编译期「自我复制和自我组装」，是减少样板代码、构建领域特定语言（DSL）的利器。与 C 的文本替换宏不同，Rust 宏操作的是**语法树（AST）**，因此更安全、更强大。

## 🎯 类比：宏是代码的 3D 打印机

普通函数像注塑模具——运行时接收参数、执行逻辑；宏像 3D 打印机——编译期根据设计图纸生成定制零件，最终产物和普通代码一起被编译。

## 🦀 声明宏 macro_rules!

使用 `macro_rules!` 定义，基于**模式匹配**展开代码。声明宏只能在其定义的 crate 中使用，适合：

- 批量生成相似函数/结构体
- 封装重复的控制流
- 创建轻量级 DSL

常用元变量：

- `$x:expr` — 表达式
- `$x:ident` — 标识符
- `$x:ty` — 类型
- `$x:pat` — 模式
- `$x:block` — 代码块
- `$x:stmt` — 语句

重复模式：
- `$(...)*` — 零个或多个
- `$(...)+` — 一个或多个
- `$(...),*` — 逗号分隔的零个或多个

## ⚙️ 过程宏 Procedural Macros

当声明宏不够用时，过程宏登场。它直接操作 Token Stream，功能更强大，但必须在单独 crate 中实现。过程宏分三类：

1. **自定义 derive**：`#[derive(MyTrait)]` 自动实现 trait
2. **属性宏**：`#[my_attribute]` 修饰函数、结构体等
3. **函数式宏**：`my_macro!()` 类似声明宏，但可做任何语法变换

## ✅ 常见使用场景

标准库里到处是宏：`vec!`、`println!`、`format!`、`assert!`、`vec!`、`thread_local!`、`lazy_static!` 等。

## ⚠️ 常见误区

- 宏不是函数，不能在运行时被调用或作为值传递
- 宏展开后可能产生难以理解的编译错误
- 声明宏的作用域和普通 item 不同，需要 `#[macro_use]` 跨模块导出
- 过程宏调试较复杂，需要 `cargo expand` 查看展开结果

## 💡 一句话总结

宏让 Rust 在保持类型安全的同时拥有极强的表达能力；声明宏适合常见模式匹配，过程宏适合复杂代码生成，两者共同支撑起 Rust 生态的「零成本抽象」。

<RustPlayground />
