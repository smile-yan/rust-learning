---
title: 项目工程化 workspace
hint: 在 Playground 中运行会自动执行测试。实际项目中使用 cargo test 运行所有测试，cargo clippy 做代码检查。
---

# 🏗️ 项目工程化 workspace

当代码从个人脚本成长为团队协作项目时，工程化能力就显得至关重要。Rust 通过 Cargo 工具链、模块系统、测试体系和 CI/CD 集成，提供了一整套成熟的工程化方案。

## 🎯 为什么工程化不可或缺

想象一个开源项目每天有几十个 PR：没有统一格式、没有自动化测试、没有文档检查，代码质量会迅速劣化。Rust 的工具链把「质量门禁」建在每次提交之前。

## 🔧 Cargo 工具链速览

- `cargo new`：创建项目
- `cargo build` / `cargo run`：编译与运行
- `cargo test`：运行测试
- `cargo check`：快速检查代码，不生成可执行文件（比 build 快得多）
- `cargo clippy`：更严格的静态检查，给出优化建议
- `cargo fmt`：统一代码格式
- `cargo doc --open`：生成并打开文档
- `cargo publish`：发布到 crates.io

## 🦀 模块系统

`mod`、`use`、`pub` 组织代码结构；`crate`、`super`、`self` 控制可见性。合理的模块划分让项目：
- 边界清晰
- 依赖可控
- 测试更易写

## ✅ 测试分类

- **单元测试**：与源码放在一起，验证单个函数或模块，使用 `#[cfg(test)] mod tests`
- **集成测试**：放在 `tests/` 目录，验证多个模块协作
- **文档测试**：写在文档注释的代码块中，保证示例代码始终可运行

## ⚙️ 持续集成

GitHub Actions 等 CI 工具通常会运行：
- `cargo test` 保证功能正确
- `cargo clippy` 捕获潜在问题
- `cargo fmt --check` 保证格式统一
- `cargo doc` 检查文档是否能正常生成

## 📦 工作空间 Workspace

当项目变大，可以使用 Cargo workspace 管理多个 crate：
- 共享 `Cargo.lock` 和 target 目录
- 拆分库、二进制、测试、工具
- 统一版本管理

典型结构：
```
my-project/
├── Cargo.toml      # workspace 根
├── crates/
│   ├── core/
│   ├── server/
│   └── cli/
```

## 🔒 依赖管理

- 使用 `cargo tree` 查看依赖树
- 使用 `cargo outdated` 检查过期依赖
- 使用 `cargo audit` 扫描已知安全漏洞
- 使用 `cargo-deny` 统一许可证和依赖策略

## 📝 文档与示例

- 文档注释 `///` 支持 Markdown
- 模块级文档 `//!`
- `examples/` 目录存放可运行示例
- 使用 `cargo doc` 生成 docs.rs 风格文档

## 📦 发布 crate

发布前要关注：
- `Cargo.toml` 元数据完整（name、version、authors、license、description）
- 语义化版本号
- README 和 CHANGELOG
- 必要的 feature gate 避免依赖膨胀
- 先用 `cargo publish --dry-run` 验证

## 💡 一句话总结

Rust 项目工程化 = Cargo 工具链守护质量 + Workspace 管理规模 + 模块系统划分边界 + 三层测试保证正确 + CI/CD 自动把关，让项目从小玩具成长为可维护的产品。

