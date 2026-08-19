---
title: "WASM 与 WebAssembly"
module: "高级应用"
order: 8
code: |
  // 这是一个 wasm-bindgen 风格示例，需要在 Cargo.toml 中添加依赖
  // [dependencies]
  // wasm-bindgen = "0.2"

  // 下面是真实项目中的导出写法（此处注释掉，使示例无需依赖即可运行）
  // use wasm_bindgen::prelude::*;

  // #[wasm_bindgen]
  // pub fn add(a: i32, b: i32) -> i32 {
  //     a + b
  // }

  fn main() {
      println!("WASM 示例代码");
      println!("在浏览器中，这段 Rust 代码可以近乎原生速度运行。");

      // 模拟 WASM 导出的函数逻辑
      fn add(a: i32, b: i32) -> i32 {
          a + b
      }

      println!("add(2, 3) = {}", add(2, 3));
      println!("使用 wasm-pack build 可以生成可在浏览器中加载的 .wasm 文件。");
  }
hint: "WASM 让 Rust 代码运行在浏览器中。实际开发需要 wasm-pack 和 wasm-bindgen 工具链。"
exercises:
  - title: "模拟 WASM 导出的 add"
    description: "写一个普通 add 函数，模拟 #[wasm_bindgen] 导出的行为。"
    code_template: |
      fn add(a: i32, b: i32) -> i32 {
          a + b
      }

      fn main() {
          println!("{}", add(2, 3));
      }
---

# 🚀 WASM 与 WebAssembly

WebAssembly（WASM）是一种可移植、体积紧凑、执行高效的二进制指令格式，能在现代浏览器中以接近原生的速度运行。Rust 因其高性能、小体积和强类型，成为编写 WASM 模块的热门语言。

## 🎯 为什么浏览器需要 WASM

JavaScript 是动态类型、解释执行，在计算密集型任务（图像处理、加密、游戏物理引擎）上存在瓶颈。WASM 提供了一种沙盒化的低级执行目标，让 C/C++/Rust 等语言编写的代码也能在 Web 上高效运行。

## 🦀 为什么选 Rust 写 WASM

- **高性能**：接近原生速度
- **小体积**：编译产物紧凑，加载快
- **安全性**：Rust 的内存安全保证延伸到浏览器沙盒
- **工具链成熟**：wasm-pack 简化了构建与发布

## ⚙️ 核心工具链

- `wasm-pack`：构建、测试、发布 Rust 生成的 WASM 包
- `wasm-bindgen`：自动生成 Rust 与 JavaScript 之间的绑定
- `web-sys`：访问浏览器 DOM、Canvas、Fetch、WebSocket 等 API
- `js-sys`：访问 JavaScript 全局对象和内置类型

## 🧠 内存模型

WASM 使用线性内存（Linear Memory），是一块连续的 byte 数组。Rust 编译后的代码在这块内存中管理栈和堆。JavaScript 可以通过 `WebAssembly.Memory` 读写这块内存，因此字符串等数据需要序列化/反序列化后传递。

## 🔗 JS 与 Rust 互操作

`wasm-bindgen` 负责处理类型映射：
- 基本数字类型直接传递
- 字符串通过拷贝转换
- 数组、对象需要显式处理
- 回调函数需要 `Closure::wrap`

```rust
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

## ✅ 典型应用场景

- 图像/视频/音频处理
- 游戏引擎和模拟器
- 加密与科学计算
- 将现有 Rust 库移植到 Web
- 前后端共享业务逻辑

## 📦 构建流程

1. 安装 wasm-pack
2. 创建 `lib` 类型的 wasm 项目
3. 用 `#[wasm_bindgen]` 导出公开函数
4. 运行 `wasm-pack build --target web`
5. 在 JavaScript 中导入生成的 npm 包或直接加载 .wasm

## ⚠️ 常见误区

- WASM 不是替代 JavaScript，而是与 JS 协作
- WASM 没有直接操作 DOM 的能力，需要通过 JS 桥接
- 跨语言调用有开销，不要把 WASM 用于过于细粒度的频繁交互
- 调试 WASM 需要 source map 和 `wasm-pack test`

## 💡 一句话总结

WASM 让 Rust 突破服务端边界进入浏览器；借助 wasm-bindgen 和 web-sys，你可以用 Rust 的安全与性能构建下一代 Web 应用。

<RustPlayground />
