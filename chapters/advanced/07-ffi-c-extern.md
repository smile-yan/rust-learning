---
title: "FFI 与 C 互操作 extern"
module: "高级应用"
order: 7
code: |
  use std::os::raw::{c_int, c_char};

  // 声明 C 标准库函数
  extern "C" {
      fn abs(input: c_int) -> c_int;
      fn strlen(s: *const c_char) -> usize;
  }

  // 导出 Rust 函数供 C 调用
  // #[no_mangle] 禁止符号名修饰，extern "C" 使用 C 的调用约定
  #[no_mangle]
  pub extern "C" fn rust_add(a: c_int, b: c_int) -> c_int {
      a + b
  }

  fn main() {
      // 调用 C 函数必须在 unsafe 块中：编译器无法检查外部函数的安全性
      unsafe {
          let result = abs(-42);
          println!("C abs(-42) = {}", result);

          // CString 保证内容以 \0 结尾，
          // as_ptr() 得到可传给 C 的裸指针
          let s = std::ffi::CString::new("Hello").unwrap();
          let len = strlen(s.as_ptr());
          println!("字符串长度: {}", len);
      }

      // 导出的 extern "C" 函数在 Rust 内部也可以像普通函数一样调用
      println!("rust_add(3, 5) = {}", rust_add(3, 5));
  }
hint: "FFI 调用在 unsafe 块中进行。用 safe 包装函数隐藏 unsafe，避免 unsafe 在业务代码中扩散。"
exercises:
  - title: "声明并调用 C 的 abs"
    description: "用 extern \"C\" 声明 abs，在 unsafe 块中调用。"
    code_template: |
      extern "C" {
          fn abs(input: i32) -> i32;
      }

      fn main() {
          unsafe {
              println!("{}", abs(-10));
          }
      }
---

# 🌉 FFI 与 C 互操作 extern

FFI（Foreign Function Interface）让 Rust 能够与 C、C++ 乃至其他语言互相调用。作为一门系统编程语言，Rust 必须具备这种能力——无论是调用操作系统 API、复用现有 C 库，还是把 Rust 代码编译成库供其他语言使用。

> ⚠️ **运行环境提示**
> 本章节代码使用了多线程 / 文件系统 / 外部 crate / FFI 等能力，**不能在前端 WASM Playground 运行**。当前 Playground 仅适合运行单线程、纯计算、标准库的示例。请复制代码到本地 IDE（VS Code + Rust Analyzer、CLion 或 cargo CLI）中运行。

## 🎯 为什么 FFI 是 unsafe

Rust 编译器无法验证 C 代码是否遵守 Rust 的内存规则：C 可能返回空指针、可能越界、可能多线程不安全。因此 FFI 调用天然需要 `unsafe`，并通过 safe 包装层把风险隔离。

## 🦀 从 Rust 调用 C

使用 `extern "C"` 声明 C 函数原型，然后在 `unsafe` 块中调用：

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}
```

实际项目中通常用 `libc` crate 提供的类型定义，避免自己写错 `c_int`、`c_void` 等映射。

## ⚙️ 从 C 调用 Rust

要让 C 看到 Rust 函数，需要：
- `#[no_mangle]`：禁止编译器修饰函数名，保持符号稳定
- `pub extern "C"`：使用 C 调用约定导出
- 参数和返回类型使用 C 兼容类型

## ✅ 类型映射

| C 类型 | Rust 对应 |
|--------|-----------|
| int | c_int / i32 |
| char | c_char / i8 或 u8 |
| void* | *mut c_void |
| struct | #[repr(C)] struct |
| 字符串 | *const c_char + CString |

## 🧱 #[repr(C)] 与内存布局

Rust 默认不保证结构体内存布局与 C 兼容。跨越 FFI 边界的结构体必须标注 `#[repr(C)]`，让编译器按 C 规则排布字段。不透明指针（opaque pointer）常用于隐藏实现细节：C 侧只看到 `typedef struct Foo Foo;`，Rust 侧用 `#[repr(C)] pub struct Foo { _private: [u8; 0] }` 表示。

## 🔧 构建集成

- 使用 `build.rs` + `cc` crate 编译 C 源码
- 使用 `pkg-config` 或 `vcpkg` 查找系统库
- 使用 `bindgen` 自动生成 C 头文件对应的 Rust 绑定
- 使用 `cbindgen` 从 Rust 生成 C 头文件

## ⚠️ 安全边界

- 所有 FFI 调用放在 unsafe 块
- 用 safe 包装函数做参数校验和错误处理
- 明确文档说明调用前提和线程安全要求
- 注意字符串编码、生命周期和资源释放
- 跨语言 ABI 必须严格匹配

## 💡 一句话总结

FFI 是 Rust 与外部世界之间的桥梁，桥的一端是 C 的自由，另一端是 Rust 的安全；用 `unsafe` 过桥，用 safe API 守门。

<RustPlayground />
