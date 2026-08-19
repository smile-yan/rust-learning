---
title: "嵌入式 Rust 入门 no_std"
module: "高级应用"
order: 9
code: |
  // 这是一个 no_std 风格的伪代码示例，展示嵌入式 Rust 的核心思想
  // 实际项目需要目标板和对应 crate

  // #![no_std] 不链接标准库，#![no_main] 没有常规 main，
  // 入口由 #[entry] 指定
  // #![no_std]
  // #![no_main]

  // use cortex_m_rt::entry;

  // #[entry]
  // fn main() -> ! {
  //     // 初始化硬件
  //     // 配置 GPIO
  //     // 主循环
  //     loop {
  //         // 闪烁 LED
  //     }
  // }

  fn main() {
      println!("嵌入式 Rust 示例");
      println!("在真实嵌入式项目中，你会使用 #![no_std] 和硬件抽象层 crate。");
      println!("Rust 的类型系统可以帮助你在编译期发现硬件配置错误。");

      // 模拟主循环
      for i in 0..3 {
          println!("循环 {}: LED 状态切换", i);
      }
  }
hint: "嵌入式 Rust 使用 #![no_std]，依赖 embedded-hal 等生态。需要实际的开发板才能运行真实代码。"
exercises:
  - title: "no_std 风格计数循环"
    description: "写一个普通的 for 循环，模拟嵌入式主循环执行 3 次。"
    code_template: |
      fn main() {
          for i in 0..3 {
              println!("tick {}", i);
          }
      }
---

# 🔌 嵌入式 Rust 入门 no_std

嵌入式设备通常资源极其有限：没有操作系统、内存以 KB 计、CPU 以 MHz 计。Rust 凭借其内存安全、零成本抽象和现代工具链，正在成为嵌入式和裸机开发的有力竞争者。

## 🎯 为什么挑战 C 的统治地位

C 在嵌入式领域长期占主导地位，但空指针、缓冲区溢出、数据竞争等问题在资源受限设备上同样致命，且更难调试。Rust 的类型系统能在编译期排除大量此类缺陷。

## 🦀 #![no_std]

标准库依赖文件系统、堆分配、线程等操作系统能力。嵌入式环境通常没有这些，因此使用 `#![no_std]` 禁用标准库，仅依赖 `core` crate。`core` 提供：
- 基础类型和 trait
- 切片、迭代器
- 选项、结果
- 原子类型

## ⚙️ 常用生态

- `embedded-hal`：硬件抽象层 trait，统一 GPIO、I2C、SPI、UART 等接口
- `cortex-m` / `cortex-m-rt`：ARM Cortex-M 启动与中断支持
- `probe-rs`：调试和烧录工具
- `defmt`：高效的帧式日志，大幅减少体积

## 🚨 Panic Handler 与启动

`#![no_std]` 下没有默认的 panic 处理，需要自己实现：

```rust
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
```

入口函数使用 `#[entry]` 宏，并且返回 `!`（永不返回），因为嵌入式程序通常永远运行在主循环中。

## 🧱 内存布局与链接脚本

通过 `memory.x` 描述 Flash 和 RAM 的地址与大小，链接器据此安排代码和数据。理解 `.text`、`.rodata`、`.data`、`.bss` 段对嵌入式调优至关重要。

## ✅ 关键概念

- **内存映射寄存器（MMIO）**：通过特定地址直接读写硬件寄存器
- **中断处理**：响应外部事件，需要仔细管理临界区
- **实时性**：满足硬实时或软实时要求
- **无堆或谨慎用堆**：堆分配可能失败或碎片化

## 🔧 入门开发板

STM32、nRF52、ESP32、Raspberry Pi Pico 等都有活跃的 Rust 社区支持。通常流程是：
1. 安装目标平台工具链（如 `rustup target add thumbv7em-none-eabihf`）
2. 选择 BSP（板级支持包）
3. 配置 `memory.x` 链接脚本
4. 使用 `cargo embed` 或 `probe-rs` 烧录

## ⚠️ 调试难度

嵌入式 Rust 调试依赖硬件调试器、OpenOCD 或 probe-rs。Playground 无法模拟真实硬件，因此示例多为伪代码或概念演示。

## 💡 一句话总结

嵌入式 Rust = `#![no_std]` 减去标准库依赖 + `embedded-hal` 统一硬件抽象 + 类型系统提前捕获硬件配置错误，让裸机开发也能享受现代语言的安全与效率。

<RustPlayground />
