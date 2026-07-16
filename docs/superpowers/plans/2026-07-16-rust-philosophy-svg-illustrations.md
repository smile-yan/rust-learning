# Rust 哲学模块 SVG 插图 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 为「Rust 哲学」模块（js/chapters.json 中 data[3]）7 章各配 2 张 SVG 插图（共 14 张，含 2 张替换 ASCII 字符画），并以 v0.1.19 发布上线。

**Architecture:** Tasks 1–7 每章一个任务，产出 `images/module3-*.svg` 两张（含 Playwright 渲染自查循环）；Task 8 用 Node 脚本把 14 张图插入 `data[3].chapters[*].theory`（两张为替换 ASCII 围栏块），bump 缓存版本，打 tag 发布并线上验证。

**Tech Stack:** 手写 SVG（无依赖）、Node `.cjs` 脚本、Playwright（项目 node_modules 已装）、GitHub Actions + `gh` CLI。

## Global Constraints

- 规格来源：`docs/superpowers/specs/2026-07-16-rust-philosophy-svg-illustrations-design.md`（已获用户批准）。
- 只新增 `images/module3-*.svg`，只修改 `data[3].chapters[*].theory`；**严禁**改动任何 `code` 字段与其他模块。
- 每个 SVG 文件完整使用以下样板（替换 `HEIGHT` 与 `TITLE`，内容放在注释 `<!-- CONTENT -->` 处）：

```xml
<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="720" height="HEIGHT" viewBox="0 0 720 HEIGHT">
  <defs>
    <style>
      .bg { fill: #fff8f0; }
      .card { fill: #ffffff; stroke: #e0d6cc; stroke-width: 2; }
      .title { font: bold 22px system-ui, -apple-system, sans-serif; fill: #1a1a1a; }
      .text { font: 15px system-ui, -apple-system, sans-serif; fill: #2a2a2a; }
      .code { font: 14px "SF Mono", Menlo, monospace; fill: #3a2a1a; }
      .label { font: 13px system-ui, -apple-system, sans-serif; fill: #5a5a5a; }
      .arrow { stroke: #7a6a5a; stroke-width: 2; fill: none; marker-end: url(#arrowhead); }
      .highlight { fill: #ffeacc; stroke: #ffaa55; stroke-width: 2; }
      .box { fill: #e8f4ff; stroke: #3399ff; stroke-width: 2; }
      .box2 { fill: #f0ffe8; stroke: #66bb33; stroke-width: 2; }
      .box3 { fill: #fff0f5; stroke: #dd5599; stroke-width: 2; }
    </style>
    <marker id="arrowhead" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
      <polygon points="0 0, 10 3.5, 0 7" fill="#7a6a5a"/>
    </marker>
  </defs>
  <rect width="720" height="HEIGHT" class="bg"/>
  <rect x="16" y="16" width="688" height="HEIGHT-32" rx="12" class="card"/>
  <text x="360" y="50" text-anchor="middle" class="title">TITLE</text>
  <!-- CONTENT -->
</svg>
```

- XML 转义：文本中的 `&` 写 `&amp;`，`<` 写 `&lt;`，`>` 写 `&gt;`（如 `&mut T` → `&amp;mut T`，`Result<...>` → `Result&lt;...&gt;`）。
- 彩色文字用内联样式覆盖 class 的 fill：`<text class="label" style="fill:#cc3333">`；✅/❌ 用 `<text font-size="18">` 不带 class。
- 项目根 `package.json` 是 `"type": "module"`，一次性脚本一律用 `.cjs` 后缀，且不要用 `module` 当变量名。
- Playwright 脚本从项目根运行（`require("playwright")` 可解析）。
- 发布约定：仓库 `smile-yan/rust-learning`，站点 `https://rust.smileyan.cn/`；本次 bump `js/app.js` 中 `chapters.json?v=10` → `v=11`、`index.html` 中 `js/app.js?v=17` → `v=18`；tag `v0.1.19`。

---

### Task 1: 3/0 安全优先 — 防线图 + 对比时间线

**Files:**
- Create: `render-check.cjs`（Tasks 1–7 共用的渲染自查脚本）
- Create: `images/module3-compile-time-defense.svg`
- Create: `images/module3-compile-vs-runtime.svg`

**Interfaces:**
- Produces: `render-check.cjs`（用法 `node render-check.cjs <svg文件...>`，在项目根生成同名 `.png`）；上述 2 个 SVG 文件名供 Task 8 使用。

- [ ] **Step 1: 写 render-check.cjs**

```js
const { chromium } = require("playwright");
(async () => {
  const files = process.argv.slice(2);
  const browser = await chromium.launch();
  for (const f of files) {
    const page = await browser.newPage({ viewport: { width: 760, height: 520 } });
    await page.goto("file://" + process.cwd() + "/" + f);
    await page.waitForTimeout(300);
    const out = f.replace("images/", "").replace(".svg", "") + ".png";
    await page.screenshot({ path: out });
    console.log("rendered", out);
    await page.close();
  }
  await browser.close();
})();
```

- [ ] **Step 2: 写 images/module3-compile-time-defense.svg**（HEIGHT=380，TITLE=`Rust 编译期的五道安全防线`）

内容（五行，行顶 y 依次为 86、138、190、242、294，每行结构相同，仅 y 与文字不同）：

```xml
  <!-- 行模板：左框机制 → 箭头 → 右框防住的错误；text y = 行顶+26，line y = 行顶+20 -->
  <rect x="70" y="86" width="220" height="40" rx="8" class="box"/>
  <text x="180" y="112" text-anchor="middle" class="text">所有权与借用检查</text>
  <line x1="300" y1="106" x2="340" y2="106" class="arrow"/>
  <rect x="350" y="86" width="300" height="40" rx="8" class="box3"/>
  <text x="500" y="112" text-anchor="middle" class="text">防止 use-after-free</text>
  <!-- 行2 y=138: 类型系统 → 防止类型不匹配 -->
  <!-- 行3 y=190: 模式匹配 → 强制处理所有分支 -->
  <!-- 行4 y=242: 生命周期 → 防止悬垂引用 -->
  <!-- 行5 y=294: Send / Sync → 防止数据竞争 -->
  <text x="360" y="356" text-anchor="middle" class="label">五道防线全部在编译期生效，不生成任何运行时代码</text>
```

按行模板把 5 行全部写出（不允许省略）。

- [ ] **Step 3: 写 images/module3-compile-vs-runtime.svg**（HEIGHT=350，TITLE=`错误发现得越早，代价越小`）

```xml
  <text x="130" y="95" text-anchor="middle" class="label">写代码</text>
  <text x="290" y="95" text-anchor="middle" class="label">编译</text>
  <text x="450" y="95" text-anchor="middle" class="label">运行</text>
  <text x="610" y="95" text-anchor="middle" class="label">线上</text>
  <rect x="45" y="140" width="110" height="40" rx="8" class="box3"/>
  <text x="100" y="166" text-anchor="middle" class="text">运行时检查</text>
  <line x1="170" y1="160" x2="650" y2="160" class="arrow"/>
  <text x="620" y="166" font-size="18">❌</text>
  <text x="170" y="195" class="text">❌ 发现太晚：bug 可能已在线上造成损失，每次检查还消耗 CPU</text>
  <rect x="45" y="230" width="110" height="40" rx="8" class="box2"/>
  <text x="100" y="256" text-anchor="middle" class="text">编译期检查</text>
  <line x1="170" y1="250" x2="285" y2="250" class="arrow"/>
  <text x="295" y="257" font-size="18">✅</text>
  <line x1="325" y1="250" x2="650" y2="250" stroke="#c9c0b8" stroke-width="2" stroke-dasharray="6 4"/>
  <text x="420" y="240" class="label">无需经历后续阶段</text>
  <text x="170" y="285" class="text">✅ 编译即拦截：精确错误位置 + 修复建议，零运行时开销</text>
  <text x="360" y="325" text-anchor="middle" class="label">拦截越早，损失越小，运行时越轻快</text>
```

- [ ] **Step 4: 语法与渲染自查**

Run: `xmllint --noout images/module3-compile-time-defense.svg images/module3-compile-vs-runtime.svg && node render-check.cjs images/module3-compile-time-defense.svg images/module3-compile-vs-runtime.svg`
Expected: xmllint 无输出；生成 2 个 png。用 ReadMediaFile 查看两张图，确认：所有文字在框内、无重叠、无越界、箭头正常。有问题改坐标后重渲，直到干净。

- [ ] **Step 5: 清理并提交**

Run: `rm -f module3-compile-time-defense.png module3-compile-vs-runtime.png && git add render-check.cjs images/module3-compile-time-defense.svg images/module3-compile-vs-runtime.svg && git commit -m "feat(images): add safety-philosophy diagrams for chapter 3/0"`

---

### Task 2: 3/1 所有权 — 移动图 + 借用规则图

**Files:**
- Create: `images/module3-ownership-move.svg`
- Create: `images/module3-ownership-borrow.svg`

**Interfaces:**
- Consumes: `render-check.cjs`（Task 1）
- Produces: 2 个 SVG 文件名供 Task 8 使用。

- [ ] **Step 1: 写 images/module3-ownership-move.svg**（HEIGHT=330，TITLE=`移动语义：所有权从 s1 转移到 s2`）

```xml
  <text x="140" y="100" text-anchor="middle" class="text">栈</text>
  <text x="560" y="100" text-anchor="middle" class="text">堆</text>
  <rect x="70" y="115" width="140" height="50" rx="8" class="box3"/>
  <text x="90" y="137" class="code">s1</text>
  <text x="90" y="158" class="label">已失效 ❌</text>
  <rect x="70" y="205" width="140" height="50" rx="8" class="box2"/>
  <text x="90" y="227" class="code">s2</text>
  <text x="90" y="248" class="label">新主人 ✅</text>
  <rect x="460" y="140" width="200" height="70" rx="8" class="highlight"/>
  <text x="480" y="182" class="code">String "hello"</text>
  <line x1="210" y1="230" x2="455" y2="180" class="arrow"/>
  <text x="280" y="195" class="label">所有权转移</text>
  <line x1="210" y1="140" x2="455" y2="165" stroke="#cc3333" stroke-width="2" stroke-dasharray="6 4"/>
  <text x="320" y="145" font-size="18">❌</text>
  <text x="360" y="300" text-anchor="middle" class="label">let s2 = s1; 之后 s1 即失效（编译错误）</text>
```

- [ ] **Step 2: 写 images/module3-ownership-borrow.svg**（HEIGHT=320，TITLE=`借用规则：读可共享，写必独占`）

```xml
  <text x="140" y="100" text-anchor="middle" class="text">不可变借用 &amp;T：多个共存 ✅</text>
  <text x="360" y="100" text-anchor="middle" class="text">可变借用 &amp;mut T：独占 ✅</text>
  <text x="580" y="100" text-anchor="middle" class="text">&amp;T 与 &amp;mut T 共存 ❌</text>
  <rect x="100" y="150" width="80" height="44" rx="8" class="box"/>
  <text x="140" y="178" text-anchor="middle" class="text">值</text>
  <text x="140" y="142" text-anchor="middle" font-size="18">✅</text>
  <text x="95" y="265" text-anchor="middle" class="code">&amp;a</text>
  <text x="140" y="265" text-anchor="middle" class="code">&amp;b</text>
  <text x="185" y="265" text-anchor="middle" class="code">&amp;c</text>
  <line x1="95" y1="252" x2="112" y2="196" class="arrow"/>
  <line x1="140" y1="252" x2="140" y2="196" class="arrow"/>
  <line x1="185" y1="252" x2="168" y2="196" class="arrow"/>
  <rect x="320" y="150" width="80" height="44" rx="8" class="highlight"/>
  <text x="360" y="178" text-anchor="middle" class="text">值</text>
  <text x="360" y="142" text-anchor="middle" font-size="18">✅</text>
  <text x="360" y="265" text-anchor="middle" class="code">&amp;mut x</text>
  <line x1="360" y1="252" x2="360" y2="196" class="arrow"/>
  <rect x="540" y="150" width="80" height="44" rx="8" class="box3"/>
  <text x="580" y="178" text-anchor="middle" class="text">值</text>
  <text x="580" y="142" text-anchor="middle" font-size="18">❌</text>
  <text x="552" y="265" text-anchor="middle" class="code">&amp;a</text>
  <text x="608" y="265" text-anchor="middle" class="code">&amp;mut b</text>
  <line x1="552" y1="252" x2="566" y2="196" class="arrow"/>
  <line x1="608" y1="252" x2="594" y2="196" class="arrow"/>
  <text x="360" y="300" text-anchor="middle" class="label">规则不满足 → 编译直接报错，而不是运行时崩溃</text>
```

- [ ] **Step 3: 渲染自查**

Run: `xmllint --noout images/module3-ownership-move.svg images/module3-ownership-borrow.svg && node render-check.cjs images/module3-ownership-move.svg images/module3-ownership-borrow.svg`
ReadMediaFile 两张 png，确认文字不越界、不重叠；必要时修坐标重渲。

- [ ] **Step 4: 清理并提交**

Run: `rm -f module3-ownership-move.png module3-ownership-borrow.png && git add images/module3-ownership-move.svg images/module3-ownership-borrow.svg && git commit -m "feat(images): add ownership diagrams for chapter 3/1"`

---

### Task 3: 3/2 fearless 并发 — 数据竞争拦截图 + channel 图

**Files:**
- Create: `images/module3-data-race.svg`
- Create: `images/module3-channel-ownership.svg`

**Interfaces:**
- Consumes: `render-check.cjs`（Task 1）
- Produces: 2 个 SVG 文件名供 Task 8 使用。

- [ ] **Step 1: 写 images/module3-data-race.svg**（HEIGHT=360，TITLE=`数据竞争是如何被编译期拦截的`）

```xml
  <rect x="40" y="85" width="300" height="220" rx="10" class="box3"/>
  <text x="55" y="118" class="text">数据竞争三条件</text>
  <text x="55" y="150" class="label">① 多线程同时访问同一数据</text>
  <text x="55" y="185" class="label">② 至少有一个是写操作</text>
  <text x="55" y="220" class="label">③ 没有同步机制</text>
  <text x="55" y="262" class="label" style="fill:#cc3333">组合起来 = 数据竞争（未定义行为）</text>
  <text x="360" y="195" text-anchor="middle" font-size="26">🛡️</text>
  <rect x="380" y="85" width="300" height="220" rx="10" class="box2"/>
  <text x="395" y="118" class="text">Rust 的拦截规则</text>
  <text x="395" y="150" class="label">① 同一时刻只有一个 &amp;mut T</text>
  <text x="395" y="185" class="label">② &amp;T 与 &amp;mut T 不能共存</text>
  <text x="395" y="220" class="label">③ 不满足 Send / Sync 无法跨线程</text>
  <text x="395" y="262" class="label">④ 违反规则 → 编译直接报错</text>
  <text x="360" y="340" text-anchor="middle" class="label">数据竞争在编译期就变成错误，而不是线上崩溃</text>
```

- [ ] **Step 2: 写 images/module3-channel-ownership.svg**（HEIGHT=360，TITLE=`消息传递：所有权随消息转移`）

```xml
  <text x="237" y="120" text-anchor="middle" class="code">tx.send(data)</text>
  <text x="477" y="120" text-anchor="middle" class="code">rx.recv()</text>
  <rect x="60" y="130" width="140" height="80" rx="8" class="box"/>
  <text x="130" y="168" text-anchor="middle" class="text">线程 A</text>
  <text x="130" y="192" text-anchor="middle" class="label">拥有数据</text>
  <rect x="280" y="155" width="160" height="40" rx="8" class="highlight"/>
  <text x="360" y="181" text-anchor="middle" class="code">channel</text>
  <rect x="520" y="130" width="140" height="80" rx="8" class="box2"/>
  <text x="590" y="168" text-anchor="middle" class="text">线程 B</text>
  <text x="590" y="192" text-anchor="middle" class="label">独占处理</text>
  <line x1="200" y1="175" x2="275" y2="175" class="arrow"/>
  <line x1="440" y1="175" x2="515" y2="175" class="arrow"/>
  <text x="130" y="240" text-anchor="middle" class="label">发送后不再持有 ❌</text>
  <text x="590" y="240" text-anchor="middle" class="label">接收后独占 ✅</text>
  <text x="360" y="320" text-anchor="middle" class="label">线程之间不共享数据，只通过 channel 转移所有权</text>
```

- [ ] **Step 3: 渲染自查**

Run: `xmllint --noout images/module3-data-race.svg images/module3-channel-ownership.svg && node render-check.cjs images/module3-data-race.svg images/module3-channel-ownership.svg`
ReadMediaFile 两张 png，确认无越界/重叠；必要时修坐标重渲。

- [ ] **Step 4: 清理并提交**

Run: `rm -f module3-data-race.png module3-channel-ownership.png && git add images/module3-data-race.svg images/module3-channel-ownership.svg && git commit -m "feat(images): add concurrency diagrams for chapter 3/2"`

---

### Task 4: 3/3 组合优于继承 — 对比图 + trait 契约图

**Files:**
- Create: `images/module3-inheritance-vs-composition.svg`
- Create: `images/module3-trait-contract.svg`

**Interfaces:**
- Consumes: `render-check.cjs`（Task 1）
- Produces: 2 个 SVG 文件名供 Task 8 使用。

- [ ] **Step 1: 写 images/module3-inheritance-vs-composition.svg**（HEIGHT=350，TITLE=`继承树 vs 组合积木`）

```xml
  <text x="185" y="90" text-anchor="middle" class="text">继承：深层耦合</text>
  <rect x="115" y="105" width="140" height="36" rx="8" class="box3"/>
  <text x="185" y="129" text-anchor="middle" class="text">Animal（基类）</text>
  <text x="265" y="129" class="label" style="fill:#cc3333">脆弱基类!</text>
  <rect x="60" y="185" width="110" height="36" rx="8" class="box"/>
  <text x="115" y="209" text-anchor="middle" class="text">Mammal</text>
  <rect x="200" y="185" width="110" height="36" rx="8" class="box"/>
  <text x="255" y="209" text-anchor="middle" class="text">Bird</text>
  <rect x="60" y="265" width="110" height="36" rx="8" class="box"/>
  <text x="115" y="289" text-anchor="middle" class="text">Dog</text>
  <line x1="170" y1="141" x2="120" y2="185" class="arrow"/>
  <line x1="200" y1="141" x2="250" y2="185" class="arrow"/>
  <line x1="115" y1="221" x2="115" y2="265" class="arrow"/>
  <text x="185" y="330" text-anchor="middle" class="label">❌ 改基类，全部子类遭殃</text>
  <text x="535" y="90" text-anchor="middle" class="text">组合：按需拼装</text>
  <rect x="420" y="100" width="85" height="26" rx="13" class="highlight"/>
  <text x="462" y="117" text-anchor="middle" class="label">Flyable</text>
  <rect x="520" y="100" width="85" height="26" rx="13" class="highlight"/>
  <text x="562" y="117" text-anchor="middle" class="label">Drivable</text>
  <line x1="462" y1="126" x2="480" y2="140" class="arrow"/>
  <line x1="562" y1="126" x2="540" y2="140" class="arrow"/>
  <rect x="420" y="140" width="230" height="150" rx="10" class="box2"/>
  <text x="445" y="165" class="code">Robot</text>
  <rect x="445" y="180" width="60" height="30" rx="6" class="box"/>
  <text x="475" y="200" text-anchor="middle" class="label">引擎</text>
  <rect x="520" y="180" width="60" height="30" rx="6" class="box"/>
  <text x="550" y="200" text-anchor="middle" class="label">轮子</text>
  <rect x="445" y="230" width="90" height="30" rx="6" class="box"/>
  <text x="490" y="250" text-anchor="middle" class="label">传感器</text>
  <rect x="550" y="230" width="80" height="30" rx="6" class="box"/>
  <text x="590" y="250" text-anchor="middle" class="label">电池</text>
  <text x="535" y="330" text-anchor="middle" class="label">✅ 能力像积木一样插拔</text>
```

- [ ] **Step 2: 写 images/module3-trait-contract.svg**（HEIGHT=380，TITLE=`trait 契约：实现即获得能力`）

```xml
  <rect x="290" y="80" width="140" height="56" rx="8" class="box"/>
  <text x="360" y="106" text-anchor="middle" class="code">trait Flyable</text>
  <text x="360" y="126" text-anchor="middle" class="label">fn fly(&amp;self)</text>
  <rect x="100" y="200" width="110" height="40" rx="8" class="box2"/>
  <text x="155" y="225" text-anchor="middle" class="text">Bird</text>
  <rect x="510" y="200" width="110" height="40" rx="8" class="box2"/>
  <text x="565" y="225" text-anchor="middle" class="text">Plane</text>
  <rect x="220" y="290" width="280" height="44" rx="8" class="highlight"/>
  <text x="360" y="317" text-anchor="middle" class="code">fn let_it_fly(f: &amp;dyn Flyable)</text>
  <line x1="180" y1="200" x2="310" y2="140" class="arrow"/>
  <text x="225" y="165" class="label">impl</text>
  <line x1="540" y1="200" x2="410" y2="140" class="arrow"/>
  <text x="480" y="165" class="label">impl</text>
  <line x1="155" y1="240" x2="280" y2="290" class="arrow"/>
  <text x="165" y="275" class="label">&amp;dyn 传入</text>
  <line x1="565" y1="240" x2="440" y2="290" class="arrow"/>
  <text x="470" y="275" class="label">&amp;dyn 传入</text>
  <text x="360" y="362" text-anchor="middle" class="label">新类型只需 impl trait，调用方代码无需改动</text>
```

- [ ] **Step 3: 渲染自查**

Run: `xmllint --noout images/module3-inheritance-vs-composition.svg images/module3-trait-contract.svg && node render-check.cjs images/module3-inheritance-vs-composition.svg images/module3-trait-contract.svg`
ReadMediaFile 两张 png，确认无越界/重叠；必要时修坐标重渲。

- [ ] **Step 4: 清理并提交**

Run: `rm -f module3-inheritance-vs-composition.png module3-trait-contract.png && git add images/module3-inheritance-vs-composition.svg images/module3-trait-contract.svg && git commit -m "feat(images): add composition diagrams for chapter 3/3"`

---

### Task 5: 3/4 显式优于隐式 — mut 图 + Result 决策图

**Files:**
- Create: `images/module3-explicit-mut.svg`
- Create: `images/module3-explicit-error.svg`

**Interfaces:**
- Consumes: `render-check.cjs`（Task 1）
- Produces: 2 个 SVG 文件名供 Task 8 使用。

- [ ] **Step 1: 写 images/module3-explicit-mut.svg**（HEIGHT=340，TITLE=`默认不可变，mut 显式开口`）

```xml
  <rect x="60" y="95" width="240" height="50" rx="8" class="box2"/>
  <text x="80" y="126" class="code">let x = 5;</text>
  <text x="330" y="120" class="code">x = 6;</text>
  <text x="400" y="122" font-size="18">❌</text>
  <text x="428" y="122" class="text">编译错误：不能二次赋值</text>
  <rect x="60" y="185" width="240" height="50" rx="8" class="highlight"/>
  <text x="80" y="216" class="code">let mut x = 5;</text>
  <text x="330" y="210" class="code">x = 6;</text>
  <text x="400" y="212" font-size="18">✅</text>
  <text x="428" y="212" class="text">允许修改，mut 让变化点显式可见</text>
  <text x="360" y="300" text-anchor="middle" class="label">不可变是默认，可变是显式选择 —— 读代码时一眼定位变化点</text>
```

- [ ] **Step 2: 写 images/module3-explicit-error.svg**（HEIGHT=350，TITLE=`Result 的三种显式处理方式`）

```xml
  <rect x="170" y="80" width="380" height="46" rx="8" class="box"/>
  <text x="360" y="109" text-anchor="middle" class="code">fn read() -&gt; Result&lt;String, io::Error&gt;</text>
  <rect x="35" y="200" width="190" height="80" rx="8" class="box2"/>
  <text x="130" y="230" text-anchor="middle" class="code">match</text>
  <text x="130" y="256" text-anchor="middle" class="label">就地处理，恢复或降级</text>
  <rect x="265" y="200" width="190" height="80" rx="8" class="box2"/>
  <text x="360" y="230" text-anchor="middle" class="code">?</text>
  <text x="360" y="256" text-anchor="middle" class="label">向上传播给调用方</text>
  <rect x="495" y="200" width="190" height="80" rx="8" class="box2"/>
  <text x="590" y="230" text-anchor="middle" class="code">unwrap</text>
  <text x="590" y="256" text-anchor="middle" class="label">确定安全才解包（会 panic）</text>
  <line x1="360" y1="126" x2="130" y2="196" class="arrow"/>
  <line x1="360" y1="126" x2="360" y2="196" class="arrow"/>
  <line x1="360" y1="126" x2="590" y2="196" class="arrow"/>
  <text x="360" y="322" text-anchor="middle" class="label">没有隐式异常，每条错误路径都必须显式写出</text>
```

- [ ] **Step 3: 渲染自查**

Run: `xmllint --noout images/module3-explicit-mut.svg images/module3-explicit-error.svg && node render-check.cjs images/module3-explicit-mut.svg images/module3-explicit-error.svg`
ReadMediaFile 两张 png，确认无越界/重叠；必要时修坐标重渲。

- [ ] **Step 4: 清理并提交**

Run: `rm -f module3-explicit-mut.png module3-explicit-error.png && git add images/module3-explicit-mut.svg images/module3-explicit-error.svg && git commit -m "feat(images): add explicitness diagrams for chapter 3/4"`

---

### Task 6: 3/5 实用主义 — unsafe 边界图 + 双列清单图

**Files:**
- Create: `images/module3-unsafe-boundary.svg`
- Create: `images/module3-unsafe-scope.svg`

**Interfaces:**
- Consumes: `render-check.cjs`（Task 1）
- Produces: 2 个 SVG 文件名供 Task 8 使用。

- [ ] **Step 1: 写 images/module3-unsafe-boundary.svg**（HEIGHT=360，TITLE=`unsafe：被围起来的责任区`）

```xml
  <rect x="45" y="80" width="630" height="230" rx="12" class="box2"/>
  <text x="65" y="108" class="text">Safe Rust 区域：借用 / 生命周期 / 类型检查全部生效</text>
  <rect x="70" y="170" width="110" height="44" rx="8" class="box"/>
  <text x="125" y="198" text-anchor="middle" class="text">调用方</text>
  <line x1="180" y1="192" x2="235" y2="192" class="arrow"/>
  <rect x="240" y="170" width="150" height="44" rx="8" class="highlight"/>
  <text x="315" y="198" text-anchor="middle" class="text">safe API 封装</text>
  <line x1="390" y1="192" x2="445" y2="192" class="arrow"/>
  <rect x="450" y="150" width="190" height="84" rx="8" class="box3"/>
  <text x="545" y="184" text-anchor="middle" class="code">unsafe { … }</text>
  <text x="545" y="210" text-anchor="middle" class="label">程序员手动担保</text>
  <text x="450" y="262" class="label">担保：指针有效、无数据竞争</text>
  <text x="360" y="340" text-anchor="middle" class="label">把 unsafe 压缩到最小，并用 safe 抽象包裹它</text>
```

- [ ] **Step 2: 写 images/module3-unsafe-scope.svg**（HEIGHT=370，TITLE=`unsafe 关闭了什么，没关什么`）

```xml
  <rect x="40" y="85" width="310" height="230" rx="10" class="box3"/>
  <text x="60" y="115" class="text">可以做的事（检查被放宽）</text>
  <text x="60" y="150" class="label">· 解引用裸指针</text>
  <text x="60" y="183" class="label">· 调用 unsafe 函数</text>
  <text x="60" y="216" class="label">· 访问 / 修改可变静态变量</text>
  <text x="60" y="249" class="label">· 实现 unsafe trait</text>
  <text x="60" y="282" class="label">· 读写 union 的字段</text>
  <rect x="370" y="85" width="310" height="230" rx="10" class="box2"/>
  <text x="390" y="115" class="text">仍然生效的检查</text>
  <text x="390" y="150" class="label">· 借用检查 ✅</text>
  <text x="390" y="183" class="label">· 生命周期检查 ✅</text>
  <text x="390" y="216" class="label">· 类型检查 ✅</text>
  <text x="390" y="249" class="label">· 其他所有安全规则 ✅</text>
  <text x="360" y="345" text-anchor="middle" class="label">unsafe 不是关掉编译器，只是申请了几项特许</text>
```

- [ ] **Step 3: 渲染自查**

Run: `xmllint --noout images/module3-unsafe-boundary.svg images/module3-unsafe-scope.svg && node render-check.cjs images/module3-unsafe-boundary.svg images/module3-unsafe-scope.svg`
ReadMediaFile 两张 png，确认无越界/重叠；必要时修坐标重渲。

- [ ] **Step 4: 清理并提交**

Run: `rm -f module3-unsafe-boundary.png module3-unsafe-scope.png && git add images/module3-unsafe-boundary.svg images/module3-unsafe-scope.svg && git commit -m "feat(images): add unsafe-philosophy diagrams for chapter 3/5"`

---

### Task 7: 3/6 大模型时代 — 生态地图 + 四大优势图

**Files:**
- Create: `images/module3-ai-infra-map.svg`
- Create: `images/module3-ai-four-strengths.svg`

**Interfaces:**
- Consumes: `render-check.cjs`（Task 1）
- Produces: 2 个 SVG 文件名供 Task 8 使用。

- [ ] **Step 1: 写 images/module3-ai-infra-map.svg**（HEIGHT=380，TITLE=`Rust 在 AI 基础设施中的位置`）

```xml
  <rect x="45" y="95" width="180" height="48" rx="8" class="box"/>
  <text x="60" y="116" class="text">模型推理</text>
  <text x="60" y="136" class="label">candle · tract</text>
  <rect x="45" y="175" width="180" height="48" rx="8" class="box"/>
  <text x="60" y="196" class="text">向量数据库</text>
  <text x="60" y="216" class="label">Qdrant · tantivy</text>
  <rect x="45" y="255" width="180" height="48" rx="8" class="box"/>
  <text x="60" y="276" class="text">分词器</text>
  <text x="60" y="296" class="label">tokenizers</text>
  <rect x="495" y="95" width="180" height="48" rx="8" class="box2"/>
  <text x="510" y="116" class="text">数据管道</text>
  <text x="510" y="136" class="label">Polars · DataFusion</text>
  <rect x="495" y="175" width="180" height="48" rx="8" class="box2"/>
  <text x="510" y="196" class="text">Agent 网关</text>
  <text x="510" y="216" class="label">tokio · axum</text>
  <rect x="495" y="255" width="180" height="48" rx="8" class="box2"/>
  <text x="510" y="276" class="text">边缘与浏览器</text>
  <text x="510" y="296" class="label">wasm-bindgen</text>
  <rect x="300" y="165" width="120" height="60" rx="30" class="highlight"/>
  <text x="360" y="202" text-anchor="middle" class="text">Rust 🦀</text>
  <line x1="225" y1="119" x2="305" y2="180" class="arrow"/>
  <line x1="225" y1="199" x2="295" y2="195" class="arrow"/>
  <line x1="225" y1="279" x2="305" y2="210" class="arrow"/>
  <line x1="495" y1="119" x2="415" y2="180" class="arrow"/>
  <line x1="495" y1="199" x2="425" y2="195" class="arrow"/>
  <line x1="495" y1="279" x2="415" y2="210" class="arrow"/>
  <text x="360" y="350" text-anchor="middle" class="label">从推理、检索到部署，Rust 正在覆盖 AI 链路的每个环节</text>
```

- [ ] **Step 2: 写 images/module3-ai-four-strengths.svg**（HEIGHT=350，TITLE=`四大优势 × AI 场景`）

四行结构相同，行顶 y 依次为 85、145、205、265（text y = 行顶+28，line y = 行顶+22）：

```xml
  <!-- 行模板 -->
  <rect x="60" y="85" width="170" height="44" rx="8" class="box"/>
  <text x="145" y="113" text-anchor="middle" class="text">极致性能</text>
  <line x1="230" y1="107" x2="330" y2="107" class="arrow"/>
  <rect x="340" y="85" width="330" height="44" rx="8" class="box2"/>
  <text x="505" y="113" text-anchor="middle" class="text">推理引擎、张量运算、KV Cache 管理</text>
  <!-- 行2 y=145: 无数据竞争并发 → 高并发模型服务、批量请求处理 -->
  <!-- 行3 y=205: 编译期安全 → AI 基础设施稳定运行，更少崩溃 -->
  <!-- 行4 y=265: 单文件部署 → 容器化、WASM、边缘与 IoT 设备 -->
  <text x="360" y="328" text-anchor="middle" class="label">大模型时代拼的不只是 GPU，还有 CPU 侧基础设施的效率</text>
```

按行模板把 4 行全部写出（不允许省略）。

- [ ] **Step 3: 渲染自查**

Run: `xmllint --noout images/module3-ai-infra-map.svg images/module3-ai-four-strengths.svg && node render-check.cjs images/module3-ai-infra-map.svg images/module3-ai-four-strengths.svg`
ReadMediaFile 两张 png，确认无越界/重叠；必要时修坐标重渲。

- [ ] **Step 4: 清理并提交**

Run: `rm -f module3-ai-infra-map.png module3-ai-four-strengths.png && git add images/module3-ai-infra-map.svg images/module3-ai-four-strengths.svg && git commit -m "feat(images): add ai-era diagrams for chapter 3/6"`

---

### Task 8: 插图注入 chapters.json + 发布 v0.1.19

**Files:**
- Create: `insert-philosophy-images.cjs`（用后删除）
- Modify: `js/chapters.json`（仅 `data[3].chapters[*].theory`）
- Modify: `js/app.js`（第 229 行附近 `chapters.json?v=10` → `v=11`）
- Modify: `index.html`（`js/app.js?v=17` → `v=18`）

**Interfaces:**
- Consumes: Tasks 1–7 产出的 14 个 `images/module3-*.svg`。

- [ ] **Step 1: 写 insert-philosophy-images.cjs**

```js
const fs = require("fs");
const FILE = "js/chapters.json";
const data = JSON.parse(fs.readFileSync(FILE, "utf8"));
const ph = data[3].chapters;

// [chapterIdx, "insert"|"replace", anchor, imageMarkdown]
// insert: 插图放到 anchor 小节末尾（下一个 "## " 之前）；replace: 用图替换匹配到的 ASCII 围栏块
const OPS = [
  [0, "replace", /```\n┌[\s\S]*?```/, "![Rust 编译期的五道安全防线](images/module3-compile-time-defense.svg)"],
  [0, "insert", "## 为什么编译期安全更重要？", "![编译期 vs 运行时检查](images/module3-compile-vs-runtime.svg)"],
  [1, "insert", "## 移动而非复制", "![移动：所有权从 s1 转移到 s2](images/module3-ownership-move.svg)"],
  [1, "insert", "## 借用：不转移所有权", "![借用规则：读可共享，写必独占](images/module3-ownership-borrow.svg)"],
  [2, "insert", "## 数据竞争的本质", "![借用规则拦截数据竞争](images/module3-data-race.svg)"],
  [2, "replace", /```\n线程 A[\s\S]*?```/, "![消息传递：所有权随消息转移](images/module3-channel-ownership.svg)"],
  [3, "insert", "## 组合 vs 继承", "![继承树 vs 组合积木](images/module3-inheritance-vs-composition.svg)"],
  [3, "insert", "## 多态依然存在", "![trait 契约：实现即获得能力](images/module3-trait-contract.svg)"],
  [4, "insert", "## 默认不可变", "![默认不可变，mut 显式开口](images/module3-explicit-mut.svg)"],
  [4, "insert", "## 显式错误处理", "![Result 的三种显式处理](images/module3-explicit-error.svg)"],
  [5, "insert", "## 边界与责任", "![unsafe：被围起来的责任区](images/module3-unsafe-boundary.svg)"],
  [5, "insert", "## unsafe 不是关闭所有检查", "![unsafe 关闭了什么，没关什么](images/module3-unsafe-scope.svg)"],
  [6, "insert", "## 已经在发生的 Rust 机遇", "![Rust 在 AI 基础设施中的位置](images/module3-ai-infra-map.svg)"],
  [6, "insert", "## Rust 在 AI 领域的独特优势", "![四大优势 × AI 场景](images/module3-ai-four-strengths.svg)"],
];

for (const [ci, type, anchor, img] of OPS) {
  const ch = ph[ci];
  if (ch.theory.includes(img)) { console.log("[skip] 3/" + ci + " 已存在"); continue; }
  if (type === "replace") {
    if (!anchor.test(ch.theory)) throw new Error("3/" + ci + " 未找到待替换的 ASCII 图");
    ch.theory = ch.theory.replace(anchor, img);
    console.log("[replace] 3/" + ci);
  } else {
    const hIdx = ch.theory.indexOf(anchor);
    if (hIdx === -1) throw new Error("3/" + ci + " 未找到小节: " + anchor);
    const next = ch.theory.indexOf("\n## ", hIdx + anchor.length);
    ch.theory = next === -1
      ? ch.theory + "\n\n" + img + "\n"
      : ch.theory.slice(0, next) + "\n\n" + img + "\n" + ch.theory.slice(next);
    console.log("[insert] 3/" + ci + " @ " + anchor);
  }
}

fs.writeFileSync(FILE, JSON.stringify(data, null, 2) + "\n");
console.log("done");
```

- [ ] **Step 2: 运行并校验**

Run: `node insert-philosophy-images.cjs`
Expected: 输出 12 行 `[insert]` + 2 行 `[replace]` + `done`。

再执行结构校验：

Run: `node -e "const d=require('./js/chapters.json'); d[3].chapters.forEach((c,i)=>{const n=(c.theory.match(/images\/module3-[a-z-]+\.svg/g)||[]).length; console.log('3/'+i, c.title, n+' 张'); if(n!==2) process.exitCode=1;});"`
Expected: 每章输出 `2 张`，退出码 0。

- [ ] **Step 3: bump 缓存版本**

Edit `js/app.js`：`fetch("./js/chapters.json?v=10")` → `fetch("./js/chapters.json?v=11")`。
Edit `index.html`：`js/app.js?v=17` → `js/app.js?v=18`。

- [ ] **Step 4: 清理临时脚本并提交、打 tag、推送**

Run: `rm -f insert-philosophy-images.cjs render-check.cjs && git add js/chapters.json js/app.js index.html && git commit -m "docs(content): illustrate Rust philosophy chapters with 14 SVGs" && git tag v0.1.19 && git push origin main && git push origin v0.1.19`
Expected: main 与 tag 均推送成功。

- [ ] **Step 5: 观察流水线**

Run: `gh run list -R smile-yan/rust-learning --branch v0.1.19 --limit 1 --json databaseId --jq '.[0].databaseId'` 拿 run id，再 `gh run watch -R smile-yan/rust-learning <id> --exit-status --interval 30`
Expected: Deploy frontend 成功（以往约 2–4 分钟）。

- [ ] **Step 6: 线上验证**

写 `verify-online.cjs`：

```js
const { chromium } = require("playwright");
(async () => {
  const checks = [
    ["#chapter/3/0", ["module3-compile-time-defense.svg", "module3-compile-vs-runtime.svg"]],
    ["#chapter/3/1", ["module3-ownership-move.svg", "module3-ownership-borrow.svg"]],
    ["#chapter/3/2", ["module3-data-race.svg", "module3-channel-ownership.svg"]],
    ["#chapter/3/3", ["module3-inheritance-vs-composition.svg", "module3-trait-contract.svg"]],
    ["#chapter/3/4", ["module3-explicit-mut.svg", "module3-explicit-error.svg"]],
    ["#chapter/3/5", ["module3-unsafe-boundary.svg", "module3-unsafe-scope.svg"]],
    ["#chapter/3/6", ["module3-ai-infra-map.svg", "module3-ai-four-strengths.svg"]],
  ];
  const browser = await chromium.launch();
  const page = await browser.newPage({ viewport: { width: 1200, height: 900 } });
  let fail = 0;
  for (const [hash, imgs] of checks) {
    await page.goto("https://rust.smileyan.cn/" + hash, { waitUntil: "networkidle" });
    await page.waitForTimeout(800);
    for (const src of imgs) {
      const el = page.locator('img[src*="' + src + '"]').first();
      const ok = await el.isVisible().catch(() => false);
      console.log((ok ? "OK  " : "MISS") + "  " + hash + "  " + src);
      if (!ok) fail++;
    }
  }
  await browser.close();
  console.log(fail === 0 ? "ALL PASS" : fail + " MISSING");
  process.exit(fail === 0 ? 0 : 1);
})();
```

Run: `node verify-online.cjs`
Expected: 14 行 `OK` + `ALL PASS`。若因 CDN/浏览器缓存偶发 MISS，等 1–2 分钟重跑确认。
另抽查 `#chapter/3/0` 与 `#chapter/3/2` 截图确认 ASCII 图已被 SVG 替换。
最后 `rm -f verify-online.cjs` 并汇报。
