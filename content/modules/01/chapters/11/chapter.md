---
title: 命令行参数解析 clap
hint: clap derive 宏会自动生成 help、校验和错误提示。
---

# 命令行参数解析 clap 🦀

几乎所有实用的命令行工具都需要解析参数。`clap` 是 Rust 生态中最流行的命令行参数解析库，它把参数定义变成**类型安全的声明式结构**，并自动生成 `--help`、错误提示、子命令、环境变量绑定等能力。

## 从生活类比开始 💡

clap 就像咖啡店的 POS 机：你提前定义好菜单，它自动识别每笔订单。

![clap 参数流转](images/module1-clap-args.svg)

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

