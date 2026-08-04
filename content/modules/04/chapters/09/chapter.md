---
title: 'Q9: Cargo.toml 是什么？'
hint: Cargo.toml 是 Rust 项目的核心配置文件。提交应用项目时通常也要提交 Cargo.lock。
---

# Q9: Cargo.toml 是什么？ 📦

`Cargo.toml` 是 Rust 项目的清单文件，相当于 Node.js 的 `package.json` 或 Python 的 `requirements.txt`。它告诉 Cargo 如何构建你的项目。

## 🧭 引入与类比

如果把 Rust 项目比作一道菜 🍲，`Cargo.toml` 就是菜谱：它列出了菜名、版本、作者，以及需要哪些食材（依赖）。

## 💡 核心概念图解

```toml
[package]
name = "my-project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
```

## 🔧 深入讲解

### 主要内容

- **`[package]`**：项目元数据，如名称、版本、作者、edition。
- **`[dependencies]`**：项目依赖的 crate。
- **`[dev-dependencies]`**：仅开发测试时使用的依赖。
- **`[features]`**：条件编译特性，用于按需开启功能。
- **`[workspace]`**：工作空间配置，管理多个相关 crate。

### Cargo.lock

首次构建后 Cargo 会生成 `Cargo.lock`，锁定依赖的具体版本，确保团队成员使用相同的依赖版本。对于应用项目，通常应该提交 `Cargo.lock`；对于库项目，一般不提交，因为库需要兼容更广泛的依赖版本。

### 常用命令

```bash
cargo new my_project      # 创建新项目
cargo build               # 编译
cargo run                 # 编译并运行
cargo test                # 运行测试
cargo add serde           # 添加依赖
```

## ⚠️ 常见误区

- **误区 1**：`Cargo.toml` 和 `Cargo.lock` 是一样的。
  - ✅ 正解：`Cargo.toml` 声明依赖范围，`Cargo.lock` 锁定精确版本。
- **误区 2**：库项目也应该提交 `Cargo.lock`。
  - ✅ 正解：库项目通常不提交 `Cargo.lock`，让下游用户自己选择兼容版本。
- **误区 3**：手动编辑 `Cargo.toml` 是添加依赖的唯一方式。
  - ✅ 正解：可以使用 `cargo add <crate>` 自动添加并格式化。

## 📝 一句话总结

`Cargo.toml` 是 Rust 项目的核心配置文件，定义了项目元数据和依赖；配合 `Cargo.lock` 可以确保构建结果可复现。

