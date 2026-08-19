---
title: "unsafe Rust"
module: "高级应用"
order: 1
code: |
  fn main() {
      let mut num = 5;

      // 创建裸指针：*const 只读、*mut 可写，创建本身不需要 unsafe
      let r1 = &num as *const i32;
      let r2 = &mut num as *mut i32;

      // 解引用裸指针、调用 unsafe 函数都必须放在 unsafe 块中，
      // 安全性由程序员自行保证
      unsafe {
          println!("r1 指向: {}", *r1);
          // 通过可变裸指针直接修改内存中的值
          *r2 += 1;
          println!("r2 指向: {}", *r2);
          dangerous();
      }

      // 任意地址的裸指针，仅作演示，不要解引用
      let address = 0x012345usize;
      let _r = address as *const i32;

      println!("unsafe 代码块结束");
  }

  // unsafe fn 表示该函数包含不安全操作，调用它也必须使用 unsafe 块
  unsafe fn dangerous() {
      println!("调用 unsafe 函数");
  }
hint: "unsafe 块应尽可能小，避免在块内进行过多操作。裸指针没有借用保证，需要程序员自己确保有效性。"
exercises:
  - title: "在 unsafe 块中解引用裸指针"
    description: "创建一个 *const i32 指向局部变量，并在 unsafe 块中打印它指向的值。"
    code_template: |
      fn main() {
          let x = 42;
          let r = &x as *const i32;
          unsafe {
              println!("{}", *r);
          }
      }
---

# 🛡️ unsafe Rust

Rust 的核心竞争力在于**编译期内存安全保证**，但在与操作系统、硬件驱动、C 库交互或实现零成本抽象时，编译器无法自动验证所有不变量。`unsafe` 关键字就是 Rust 为这种场景开的「安全天窗」——它允许程序员在**明确承担责任**的前提下绕过部分检查。

## 🎯 为什么需要 unsafe

想象一栋大楼：99% 的房间都有烟雾报警器和防火墙（借用检查器），但顶层天台需要一把特殊钥匙才能上。`unsafe` 就是这把钥匙——它不会拆掉整栋楼的消防系统，只是把特定区域的安全责任交给使用者。

`unsafe` 的典型出场场景：
- 与 C/C++ 代码互操作（FFI）
- 操作内存映射寄存器（嵌入式）
- 实现底层数据结构（如 BTreeMap、Vec）
- 对性能有极致要求的手动优化

## ⚠️ unsafe 能做什么

进入 `unsafe` 块或标记 `unsafe fn` 后，你可以：

- 解引用裸指针 `*const T` 和 `*mut T`
- 调用 `unsafe` 函数或方法
- 访问或修改可变静态变量 `static mut`
- 实现 `unsafe trait`
- 访问 `union` 的字段

## ✅ unsafe 不能做什么

这是最常见的误区！`unsafe` **不会**：

- 关闭借用检查器（常规引用仍受检查）
- 允许违反类型系统（不能随意转换类型）
- 取消整数溢出检查
- 让未定义行为变成合法

它只是把**一部分**安全保证从编译器转移到了程序员的肩膀上。

## 🦀 裸指针详解

裸指针是 `unsafe` 的灵魂角色：

| 特性 | `&T / &mut T` | `*const T / *mut T` |
|------|---------------|---------------------|
| 借用检查 | ✅ 有 | ❌ 无 |
| 生命周期 | ✅ 有 | ❌ 无 |
| 允许 null | ❌ 否 | ✅ 是 |
| 解引用 | 随处 | 仅在 unsafe 块 |

裸指针可以指向任意地址，包括无效内存。解引用前必须确认它确实有效，否则就是**未定义行为（UB）**。

![裸指针 vs 引用](/images/module2-raw-pointer.svg)

## 🔧 使用原则

1. **尽量保持 unsafe 块小且集中**——越小越容易审查
2. **用 safe API 包装 unsafe 代码**——让调用方看不到 unsafe
3. **在文档中清晰说明不变条件**——例如指针有效性、调用前提、线程安全要求
4. **优先使用成熟 crate**——如 `libc`、`winapi`、`bytemuck`

![safe API 包裹 unsafe](/images/module2-unsafe-boundary.svg)

## 🌉 FFI 与 extern

通过 `extern "C"` 声明 C 函数是最典型的 unsafe 场景。Rust 无法验证 C 代码的内存安全，因此调用必须在 `unsafe` 块中完成。建议总是写一个 safe 包装函数，在包装层检查参数、转换类型、处理错误。

## 💡 一句话总结

`unsafe` 不是 Rust 安全模型的对立面，而是**可控的边界**：把编译器无法证明安全的地方显式标注出来，让风险集中在最小范围内。

<RustPlayground />
