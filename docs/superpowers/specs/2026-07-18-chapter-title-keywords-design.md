# 章节标题添加英文关键字设计

日期：2026-07-18
状态：方案与映射表已获用户批准（方案 A，按表执行）

## 目标

为教程各章节标题补充简单的英文关键字，帮助初学者把中文概念与代码/文档中的英文术语对应起来。示例（用户给出）：`变量与可变性` → `常量与变量 let / let mut`。

## 范围与格式（方案 A）

- **模块 0–2（基础入门/中等应用/高级应用）**：逐章添加，个别保持不变的除外。
- **模块 3（Rust 哲学）、模块 4（Q & A）**：不加。标题为理念句/疑问句，且多数已含英文术语（unsafe、fearless、unwrap、null、RAII、free 等），硬加冗余。
- **格式**：`中文标题 + 空格 + 关键字`，多个关键字用 ` / ` 分隔。
- **位置**：`title` 字段（侧边栏目录）与 `theory` 首行 H1 同步修改；H1 原有前缀/后缀 emoji 保留，关键字紧跟中文标题之后。

## 标题映射表

### 模块 0 基础入门

| 原标题 | 新标题 |
| --- | --- |
| Hello World | Hello World println! |
| 变量与可变性 | 常量与变量 let / let mut |
| 数据类型 | 数据类型 i32 / f64 / bool |
| 函数 | 函数 fn |
| 控制流 | 控制流 if / loop / for |
| 所有权 | 所有权 Ownership |
| 结构体 | 结构体 struct |
| 枚举与模式匹配 | 枚举与模式匹配 enum / match |
| 错误处理 | 错误处理 Result / panic! |
| 集合 | 集合 Vec / HashMap |
| 注释与运算符 | 注释与运算符 // |
| 切片 | 切片 slice |
| 迭代器 | 迭代器 iter |
| 枚举与 Option | 枚举与 Option Some / None |
| 模块与包管理 | 模块与包管理 mod / use |
| Rust 关键字速览 | （不变，主题即关键字） |
| 项目组织与 Cargo | 项目组织与 Cargo crate |

### 模块 1 中等应用

| 原标题 | 新标题 |
| --- | --- |
| 泛型 | 泛型 \<T\> |
| Trait | Trait impl |
| 生命周期 | 生命周期 'a |
| 闭包与迭代器 | 闭包与迭代器 closure |
| 智能指针 | 智能指针 Box / Rc / RefCell |
| 并发基础 | 并发基础 thread |
| 类型转换与类型系统进阶 | 类型转换与类型系统进阶 From / Into |
| 测试与错误处理最佳实践 | 测试与错误处理最佳实践 #[test] |
| 文件 I/O 与路径处理 | 文件 I/O 与路径处理 fs / Path |

### 模块 2 高级应用

| 原标题 | 新标题 |
| --- | --- |
| unsafe Rust | （不变，已含英文） |
| 宏编程 | 宏编程 macro_rules! |
| 异步编程 | 异步编程 async / await |
| Web 开发基础 | Web 开发基础 axum |
| 数据库与序列化 | 数据库与序列化 serde / sqlx |
| 项目工程化 | 项目工程化 workspace |
| FFI 与 C 互操作 | FFI 与 C 互操作 extern |
| WASM 与 WebAssembly | （不变，已含英文） |
| 嵌入式 Rust 入门 | 嵌入式 Rust 入门 no_std |
| 性能优化与 Benchmark | 性能优化与 Benchmark criterion |
| 并发模式与设计 | 并发模式与设计 channel |

## 实现要点

- 用 Python 脚本修改 `js/chapters.json`（保持 `ensure_ascii=False, indent=2` + 末尾换行的既有字节格式），逐条 assert 旧标题存在再替换。
- **H1 转义特例**：`泛型 <T>` 在正文 H1 中必须写成 `泛型 \<T\>`——marked 会把裸 `<T>` 当 HTML 标签吞掉；侧边栏 `title` 经 Vue 文本插值渲染，无需转义。
- H1 三种现状格式均已确认：模块 0 为 `# 标题`（ch15 带后缀 📖）；模块 1 为 `# 标题 🦀`；模块 2 为 `# emoji 标题`。替换时只动标题文本，不动 emoji。
- 已扫描全库：其他章节 theory 中无对「变量与可变性」的交叉引用，改名安全。
- 章节 hash 路由基于索引（`#chapter/m/c`），改名不影响路由。

## 验证

1. `python3 scripts/validate_chapters.py`：JSON 合法（存量 enums `?v=2` 告警与本次无关）。
2. Playwright 抽查渲染：侧边栏目录显示新标题；正文 H1 正确（重点检查 `泛型 <T>` 的小于号/大于号可见、模块 2 的 emoji 前缀保留）。
3. 缓存号：`chapters.json?v=16→17`、`app.js?v=27→28`。
