---
title: "常量与变量 let / let mut"
module: "基础入门"
order: 2
code: |
  fn main() {
      // let 声明的变量默认不可变，这是 Rust 的安全设计
      let x = 5;
      println!("x = {}", x);
      // x = 6; // 取消注释会报错，因为 x 不可变

      // 加上 mut 关键字，变量才能被重新赋值
      let mut y = 10;
      println!("修改前 y = {}", y);
      y = 20;
      println!("修改后 y = {}", y);

      // const 声明常量：必须标注类型，命名习惯全大写
      // 100_000 中的下划线只是可读性分隔符，不影响数值
      const MAX_POINTS: u32 = 100_000;
      println!("常量 MAX_POINTS = {}", MAX_POINTS);

      // 隐藏：用同名新变量覆盖旧变量
      // 新变量允许换类型，这是它与 mut 重新赋值的关键区别
      let spaces = "   ";
      let spaces = spaces.len();
      println!("隐藏后的 spaces = {}", spaces);

      // parse 方法需要显式类型标注
      // expect 在解析失败时直接让程序 panic 并显示这条提示
      let guess: u32 = "42".parse().expect("不是数字");
      println!("解析结果: {}", guess);
  }
hint: "注意不可变变量与可变变量的区别，以及 const 必须标注类型。隐藏与 mut 不同，它创建的是新变量。"
exercises:
  - title: "累加器"
    description: "声明一个可变的 counter，循环 5 次每次加 1，最后打印结果。"
    code_template: |
      fn main() {
          let mut counter = 0;
          // 使用 for 循环累加
          println!("{}", counter);
      }
  - title: "常量计算圆面积"
    description: "定义 const PI: f64 = 3.14，给定半径 r = 5，打印圆面积。"
    code_template: |
      fn main() {
          const PI: f64 = 3.14;
          let r = 5.0;
          // area = PI * r * r
      }
---

# 常量与变量 let / let mut

Rust 最重要的设计理念之一是 **默认不可变性**。变量一旦绑定到某个值，默认情况下就不能再指向其他值。这让代码更安全、更易于推理，也是 Rust 能在编译期发现大量并发 bug 的基础。很多人第一次写 Rust 时会被这个设计“绊倒”，但当你习惯后，会发现它让代码意图变得非常清晰。

## 引入：保险箱的比喻 🔐

想象变量是一个保险箱：

- 默认情况下，保险箱贴上了封条，里面的东西不能换（不可变）。
- 如果你明确贴上 `mut` 标签，就代表这个保险箱可以随时打开更换内容（可变）。
- 常量 `const` 则是刻在金属板上的数值，永远不可更改，且所有使用它的地方都在编译期替换。

## 概念图解 💡

```rust
let x = 5;        →  🔒 封条，不可再赋值
let mut y = 10;   →  🔑 可打开，可修改
const PI = 3.14;  →  ⚙️ 编译期固定，全局可用
```

## 深入讲解

### 不可变变量

`let x = 5;` 声明的变量 `x` 默认不可变，重新赋值会导致编译错误。Rust 编译器会强制你思考：这个值真的需要改吗？

### 可变变量

如果需要修改变量，必须显式使用 `mut` 关键字：`let mut y = 10;`。这种显式声明让代码读者一眼就能看出哪里可能发生变化。

### 常量

`const MAX_POINTS: u32 = 100_000;` 声明常量，必须标注类型，值在编译期确定，且始终不可变。常量命名习惯使用全大写加下划线。下划线 `_` 在数字中只起分隔作用，不影响数值。

### 隐藏（Shadowing）

可以使用相同的名字声明新变量，新变量会 **隐藏** 旧变量。与 `mut` 不同，隐藏可以修改变量的类型。

例如可以先让 `spaces` 是字符串，然后让同名变量 `spaces` 变成数字：

```rust
let spaces = "   ";
let spaces = spaces.len();
```

## 常见误区 ⚠️

- 认为 `let x = 5; x = 6;` 可以编译：Rust 默认不可变。
- 混淆 `mut` 和 shadowing：`mut` 是修改同一个变量；shadowing 是创建一个新变量。
- 常量不写类型：`const` 必须显式标注类型。
- 在循环内部 shadowing 外部变量时，误以为修改了外部变量。

## 一句话总结 ✅

Rust 默认不可变，用 `mut` 显式开启可变；常量编译期确定且必须标注类型；shadowing 允许同名变量改变类型。

<RustPlayground />
