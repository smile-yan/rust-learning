---
title: 数据库与序列化 serde / sqlx
hint: serde_json 是 Rust 生态中处理 JSON 的标准选择。derive 宏会自动生成序列化和反序列化代码。
---

# 🗄️ 数据库与序列化 serde / sqlx

任何真实应用都需要与外部系统交换数据，并把数据持久化。Rust 生态在这两个方向上都有非常成熟且类型安全的解决方案：serde 负责序列化，sqlx / SeaORM 负责数据库访问。

> ⚠️ **运行环境提示**
> 本章节代码使用了多线程 / 文件系统 / 外部 crate / FFI 等能力，**不能在前端 WASM Playground 运行**。当前 Playground 仅适合运行单线程、纯计算、标准库的示例。请复制代码到本地 IDE（VS Code + Rust Analyzer、CLion 或 cargo CLI）中运行。

## 🎯 为什么类型安全很重要

在动态语言中，JSON 字段拼写错误或类型不匹配通常要到运行时才会暴露。Rust 借助 serde 和强类型结构体，把这类错误**提前到编译期**发现。

## 🦀 serde：序列化的事实标准

`serde` 提供了一套统一的序列化/反序列化抽象。通过 `#[derive(Serialize, Deserialize)]`，结构体几乎可以无缝转换为：
- JSON（serde_json）
- YAML（serde_yaml）
- TOML（toml）
- MessagePack（rmp-serde）
- Bincode（bincode）

常用方法：
- `serde_json::to_string` / `to_string_pretty`
- `serde_json::from_str` / `from_slice`

## ⚙️ sqlx：编译期检查 SQL

`sqlx` 是 Rust 中极具特色的数据库库：
- 查询在编译时检查语法和参数类型
- 支持 PostgreSQL、MySQL、SQLite
- 原生 async/await 支持
- 零运行时开销（非 ORM，不隐藏 SQL）

对于喜欢写原始 SQL 的开发者，sqlx 提供了类型安全又不损失控制力的体验。

## 🏗️ ORM 选择

- **SeaORM**：基于 async、与 sqlx 驱动集成、活跃的社区
- **Diesel**：同步生态的成熟 ORM，编译期查询检查强，但 async 支持需额外适配

## ✅ 数据验证

结合 `validator` 等库，可以在反序列化或业务层自动验证字段：
- 邮箱格式
- 字符串长度
- 数值范围
- 自定义规则

## ⚠️ Playground 的限制

浏览器 Playground 无法连接真实数据库，因此示例通常只展示结构体定义和 serde 序列化。真实项目中需要配置连接池、迁移工具和错误处理。

## 💡 一句话总结

Rust 数据层 = serde 做类型安全的序列化 + sqlx/SeaORM 做类型安全的数据库访问，让「字段错误」在编译期就无处遁形。

