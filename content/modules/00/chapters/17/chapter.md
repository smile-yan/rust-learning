---
title: 项目组织与 Cargo crate
hint: 文档注释中的代码示例默认会作为文档测试运行，是编写可测试文档的好方法。在 Playground 中 tests 模块也会被执行。
---

# 项目组织与 Cargo crate

Cargo 是 Rust 的构建系统和包管理器。它不仅能编译代码、管理依赖，还能运行测试、生成文档、发布 crate。学会 Cargo，才算真正进入 Rust 工程化开发。可以毫不夸张地说，Cargo 是 Rust 生态如此繁荣的重要功臣之一。

## 引入：Cargo 是你的项目经理 📋

没有 Cargo，你需要手动调用 `rustc`、管理依赖路径、记住各种编译选项。有了 Cargo，一句 `cargo run` 就能完成编译和运行，一句 `cargo test` 就能跑测试。它让 Rust 项目从“能跑”变成“好维护”。

## 概念图解 💡

```rust
my_project/
├── Cargo.toml    ← 项目配置：名称、版本、依赖
├── Cargo.lock    ← 依赖版本锁定
└── src/
    ├── main.rs   ← 二进制入口
    └── lib.rs    ← 库入口（可选）
```

## 深入讲解

### Cargo.toml

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0"
```

### 常用命令

- `cargo new project_name`：创建新项目
- `cargo build`：编译项目
- `cargo run`：编译并运行
- `cargo test`：运行测试
- `cargo check`：快速检查代码，不生成可执行文件
- `cargo doc --open`：生成并打开文档

### 依赖管理

Cargo 会自动从 crates.io 下载依赖，并缓存到本地。`Cargo.lock` 会锁定具体版本，保证团队协作时依赖一致。

### 工作空间（Workspace）

大型项目可以用 workspace 管理多个 crate：

```toml
[workspace]
members = ["crate-a", "crate-b"]
```

## 常见误区 ⚠️

- 直接修改 `Cargo.lock`：通常应该通过修改 `Cargo.toml` 来更新依赖。
- 忽略 `Cargo.toml` 中的 `edition`：不同 edition 的语法和特性可能不同。
- `cargo check` 和 `cargo build` 混淆：`check` 更快，但不生成产物。
- 把二进制和库 crate 的入口文件放错位置。

## 一句话总结 ✅

Cargo 是 Rust 的瑞士军刀，掌握 `cargo new/build/run/test/check/doc`，能大幅提升 Rust 开发效率和工程化能力。

