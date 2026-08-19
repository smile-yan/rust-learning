---
title: "命令行参数解析 clap"
module: "中等应用"
order: 11
code: |
  use clap::Parser;

  #[derive(Parser)]
  #[command(name = "greet")]
  struct Cli {
      #[arg(short, long, default_value = "World")]
      name: String,

      #[arg(short, long, default_value_t = 1)]
      count: u32,

      #[arg(short, long)]
      shout: bool,
  }

  fn main() {
      let args = Cli::parse();
      for _ in 0..args.count {
          let mut msg = format!("Hello, {}!", args.name);
          if args.shout { msg = msg.to_uppercase(); }
          println!("{}", msg);
      }
  }
hint: "clap derive 宏会自动生成 help、校验和错误提示。"
exercises:
  - title: "计算器 CLI"
    description: "用 clap 定义一个 calc 程序，接受 add <a> <b> 子命令并打印结果。"
    code_template: |
      use clap::{Parser, Subcommand};

      #[derive(Parser)]
      struct Cli {
          #[command(subcommand)]
          command: Commands,
      }

      #[derive(Subcommand)]
      enum Commands { Add { a: i32, b: i32 } }

      fn main() {}
---

# 命令行参数解析 clap 🦀

几乎所有实用的命令行工具都需要解析参数。`clap` 是 Rust 生态中最流行的命令行参数解析库，它把参数定义变成**类型安全的声明式结构**，并自动生成 `--help`、错误提示、子命令、环境变量绑定等能力。

## 从生活类比开始 💡

clap 就像咖啡店的 POS 机：你提前定义好菜单，它自动识别每笔订单。

![clap 参数流转](/images/module1-clap-args.svg)

## 深入讲解

### derive 宏

```rust
#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    name: String,
}
```

### 常用参数类型

- 标志位 `--flag`：`bool`
- 值参数 `--key VALUE`：`String`、`i32`
- 多值 `--files <PATH>`：`Vec<String>`
- 子命令 `add / remove`：嵌套 enum

## 一句话总结 🦀

> clap 用声明式定义把命令行参数变成类型安全的 Rust 代码，自动生成帮助文档并校验输入。

<RustPlayground />
