# Rust 哲学模块配图设计

日期：2026-07-16
状态：已获用户批准（方案 A）

## 目标

为「Rust 哲学」模块（`js/chapters.json` 中 `data[3]`）的 7 篇文章补充 SVG 矢量示意图，每篇约 2 张，共 14 张；其中 2 张用于替换文章中现有的 ASCII 字符画。

## 范围

- 仅涉及 `data[3].chapters[0..6]` 这 7 章的 `theory` 字段与 `images/` 目录新增文件。
- 不改动任何代码示例（`code` 字段）、不改动其他模块。
- 同步 bump 缓存版本：`js/app.js` 的 `chapters.json?v=10` → `v=11`，`index.html` 的 `app.js?v=17` → `v=18`。

## 风格约定（沿用全站现有 SVG 惯例）

- viewBox 宽 720，高度按内容 300–400；`rx` 圆角卡片。
- 配色：底 `#fff8f0`、白卡片描边 `#e0d6cc`；蓝框 `#e8f4ff`/`#3399ff`、绿框 `#f0ffe8`/`#66bb33`、粉框 `#fff0f5`/`#dd5599`、橙高亮 `#ffeacc`/`#ffaa55`。
- 字体：标题 bold 22px system-ui，正文 15px，标注 13px，代码 14px Menlo 等宽。
- 箭头统一用 `marker-end` 箭头标记；正负语义用 ✅/❌ 或绿色/红色区分。
- 图内不使用占位标题（如 `module0-structs` 这类无意义标题不再出现）。

## 每章配图清单

### 3/0 安全优先：编译期保证
1. `images/module3-compile-time-defense.svg` —— 编译期五道防线图：五个横条（所有权与借用检查→use-after-free、类型系统→类型不匹配、模式匹配→分支遗漏、生命周期→悬垂引用、Send/Sync→数据竞争），左为机制右为防住的错误。**替换「## Rust 的编译期防线」下的 ASCII 代码块。**
2. `images/module3-compile-vs-runtime.svg` —— 上下两条时间线（写代码→编译→运行→线上）：编译期检查在「编译」节点拦截错误（✅ 零运行时开销），运行时检查到「线上」才暴露（❌ 已有损失 + 持续开销）。插入「## 为什么编译期安全更重要？」小节。

### 3/1 所有权：内存管理的哲学
1. `images/module3-ownership-move.svg` —— 移动语义：左 `s1` 右 `s2` 两个栈框与一块堆内存 `"hello"`；赋值后所有权箭头从 s1 移到 s2，s1 标 ❌ 失效。插入「## 移动而非复制」小节。
2. `images/module3-ownership-borrow.svg` —— 借用规则三格：多个 `&T` 共存 ✅、单个 `&mut T` 独占 ✅、`&T` 与 `&mut T` 共存 ❌。插入「## 借用：不转移所有权」小节。

### 3/2 fearless 并发：共享状态的安全哲学
1. `images/module3-data-race.svg` —— 数据竞争三条件（多线程同访、至少一写、无同步）→ 借用规则逐条拦截。插入「## 数据竞争的本质」小节。
2. `images/module3-channel-ownership.svg` —— 线程 A → channel → 线程 B，消息携带所有权转移，两端各自独占。**替换「## 通道 vs 共享内存」下的 ASCII 代码块。**

### 3/3 组合优于继承：设计哲学的选择
1. `images/module3-inheritance-vs-composition.svg` —— 左：深层继承树（基类标红「脆弱基类」，改动波及全部子类）；右：扁平组合积木块（自由拼装、低耦合）。插入「## 组合 vs 继承」小节。
2. `images/module3-trait-contract.svg` —— trait 契约：中心 `Flyable` trait，`Bird`/`Plane` 等类型实现它，`&dyn Flyable` 统一调用 `fly()`。插入「## 多态依然存在」小节。

### 3/4 显式优于隐式：错误处理与可变性
1. `images/module3-explicit-mut.svg` —— `let x = 5`（默认不可变，大道）vs `let mut x = 5`（显式开口，变化点一目了然）。插入「## 默认不可变」小节。
2. `images/module3-explicit-error.svg` —— `Result<T, E>` 决策流：`Ok`/`Err` 分出三个处理出口（`match` 就地处理、`?` 向上传播、`unwrap` 确定安全时解包）。插入「## 显式错误处理」小节。

### 3/5 实用主义：unsafe 的边界与责任
1. `images/module3-unsafe-boundary.svg` —— 边界围墙图：大的 safe 区域（编译器守护）内嵌小的 unsafe 块（程序员担保），safe API 封装作为唯一城门。插入「## 边界与责任」小节。
2. `images/module3-unsafe-scope.svg` —— 双列清单：unsafe 关闭的检查（解引用裸指针、调用 unsafe fn、可变静态、unsafe trait、union 字段）vs 仍然生效的检查（借用、生命周期、类型）。插入「## unsafe 不是关闭所有检查」小节。

### 3/6 大模型时代下的 Rust 机遇
1. `images/module3-ai-infra-map.svg` —— AI 基础设施生态地图：中心 Rust，周围六个场景块（推理 candle/tract、向量库 Qdrant、分词 tokenizers、数据管道 Polars/DataFusion、Agent 网关 tokio/axum、边缘 wasm-bindgen）。插入「## 已经在发生的 Rust 机遇」小节。
2. `images/module3-ai-four-strengths.svg` —— 四大优势（性能/并发/安全/部署）各自映射到 AI 场景（推理引擎、模型服务、基础设施、容器与边缘）。插入「## Rust 在 AI 领域的独特优势」小节。

## 插入方式

- 每张图以 `![alt](images/module3-xxx.svg)` 插入对应小节首段文字之后，与全站其他章节格式一致。
- 两张替换图：删除原 ``` 围起来的 ASCII 代码块，原位插入图片引用。
- 修改 `js/chapters.json` 采用「JSON 片段 + Node `.cjs` 脚本」方式，避免大文件手工改写出错。

## 验证

- 每张 SVG 画完后用 Playwright 本地渲染截图，确认无文字越界、元素不重叠。
- `js/chapters.json` 修改后用 `JSON.parse` 校验结构，抽查 2–3 章 theory 渲染正常。
- 提交并打 tag（下一个版本 v0.1.19）推送，`gh run watch` 观察部署流水线。
- 部署完成后 Playwright 访问线上 7 个章节页，确认 14 张图全部可见且显示正常。

## 非目标

- 不重写章节正文文字（除删除两处 ASCII 图外）。
- 不为其他模块新增插图。
