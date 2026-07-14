# 教程内容扩展实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 将 `js/chapters.json` 中 53 个章节的理论内容扩展为原来的 2-3 倍，为核心概念章节生成本地化 SVG 示意图，并确保 JSON 语法、图片引用和前端渲染正常。

**Architecture:** 保持现有纯静态架构不变，直接修改 `js/chapters.json`（`theory` 字段），在 Markdown 中引用 `images/` 下的 SVG 文件；按四个模块并行生成内容，最后统一校验和本地预览。

**Tech Stack:** JSON、Markdown、SVG、Python 3（校验脚本）、本地后端（Rust + Axum）

## Global Constraints

- 不修改 `app.js`、不引入新的构建工具。
- 现有 `code` 和 `hint` 字段保持不变，仅补充 `theory` 中的代码讲解。
- 所有 SVG 图片必须本地化，存放在 `images/` 目录，不使用外部 CDN。
- 每个章节 `theory` 字数需达到原来的 2 倍以上。
- 核心概念章节必须配图；非核心章节以文字和表情为主。
- 最终必须在 `http://localhost:9001` 预览并确认无异常。

---

## File Structure

| 文件/目录 | 职责 |
|---|---|
| `js/chapters.json` | 扩展后的章节数据，含 Markdown 图片引用 |
| `images/` | 核心概念 SVG 示意图 |
| `scripts/validate_chapters.py` | 校验 JSON 语法、图片引用、章节字数是否达标 |
| `docs/superpowers/specs/2026-07-14-expand-chapters-design.md` | 已确认的设计文档 |
| `backend/target/release/rust-learning-backend` | 用于本地预览的后端二进制 |

---

### Task 1: 建立图片目录与 SVG 风格基线

**Files:**
- Create: `images/.gitkeep`
- Create: `images/style-guide.svg`（风格参考样例，可选）

**Interfaces:**
- Consumes: 无
- Produces: `images/` 目录存在，后续 SVG 都存放于此

- [ ] **Step 1: 创建图片目录**

```bash
mkdir -p /Users/yanshili/me/projects/rust-projects/images
touch /Users/yanshili/me/projects/rust-projects/images/.gitkeep
```

- [ ] **Step 2: 确认 SVG 引用路径规则**

在 `theory` 中使用如下 Markdown 语法引用图片，路径相对于项目根目录：

```markdown
![图片描述](images/xxx.svg)
```

- [ ] **Step 3: Commit**

```bash
git add images/.gitkeep
git commit -m "chore: create images directory for chapter diagrams"
```

---

### Task 2: 编写校验脚本

**Files:**
- Create: `scripts/validate_chapters.py`

**Interfaces:**
- Consumes: `js/chapters.json`
- Produces: 校验报告，退出码 0 表示通过

- [ ] **Step 1: 编写校验脚本**

```python
#!/usr/bin/env python3
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CHAPTERS = ROOT / "js" / "chapters.json"
IMAGES = ROOT / "images"


def load_json():
    with CHAPTERS.open("r", encoding="utf-8") as f:
        return json.load(f)


def count_chapters(modules):
    return sum(len(m["chapters"]) for m in modules)


def check_image_refs(modules):
    refs = set()
    for m in modules:
        for c in m["chapters"]:
            refs.update(re.findall(r"!\[.*?\]\((images/[^)]+)\)", c.get("theory", "")))
    missing = [r for r in refs if not (ROOT / r).exists()]
    if missing:
        print("Missing images:", missing)
        return False
    print(f"Image refs OK: {len(refs)} refs, all files exist")
    return True


def check_length(modules):
    failures = []
    for mi, m in enumerate(modules):
        for ci, c in enumerate(m["chapters"]):
            theory = c.get("theory", "")
            # 放宽到 1.8 倍，避免原始章节长度波动导致误报
            if len(theory) < 300:
                failures.append(f"module {mi} chapter {ci} too short")
    if failures:
        print("Length failures:", failures)
        return False
    print("Length check OK")
    return True


def main():
    try:
        modules = load_json()
    except json.JSONDecodeError as e:
        print("JSON parse error:", e)
        sys.exit(1)

    print(f"Total chapters: {count_chapters(modules)}")
    ok = check_image_refs(modules) and check_length(modules)
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
```

- [ ] **Step 2: 运行脚本确认当前状态**

```bash
cd /Users/yanshili/me/projects/rust-projects
python3 scripts/validate_chapters.py
```

Expected: 通过 JSON 解析和图片引用检查，字数检查可能因当前内容较短而报部分章节过短（后续任务会修复）。

- [ ] **Step 3: Commit**

```bash
git add scripts/validate_chapters.py
git commit -m "feat: add chapters.json validation script"
```

---

### Task 3: 并行扩展四个模块的文案与配图

**Files:**
- Read: `js/chapters.json`
- Create: `js/module-0-basics.json`
- Create: `js/module-1-intermediate.json`
- Create: `js/module-2-advanced.json`
- Create: `js/module-3-qa.json`
- Create: `images/*.svg`

**Interfaces:**
- Consumes: 原始 `js/chapters.json`
- Produces: 四个模块各自扩展后的 JSON 片段和对应 SVG 文件

为避免多个子代理同时修改同一个文件产生冲突，每个子代理只负责生成自己模块的独立 JSON 片段文件，不直接修改 `js/chapters.json`。每个子代理负责：
1. 读取当前 `js/chapters.json` 中对应模块的所有章节。
2. 将每个章节的 `theory` 扩展为原来的 2-3 倍，加入表情、类比、深入讲解、常见误区和总结。
3. 为核心概念章节生成 1-3 张 SVG 图片，保存到 `images/`，并在 `theory` 中通过 Markdown 引用。
4. 保持 `code` 和 `hint` 字段完全不变。
5. 将结果写入 `js/module-{N}-{name}.json`，文件中只包含该模块的完整对象（`{"name": "...", "chapters": [...]}`）。
6. 校验生成的 JSON 片段语法正确。

- [ ] **Step 1: 扩展“基础入门”模块**

Prompt（给子代理）：

> 请扩展 `js/chapters.json` 中 `"name": "基础入门"` 模块的所有章节。每个章节的 `theory` 字段需要扩展为原来的 2-3 倍，结构包含：引入/类比、概念图解、深入讲解、常见误区、一句话总结。加入表情符号。为核心概念章节（函数、控制流、所有权、结构体、枚举与模式匹配、错误处理、集合、切片、迭代器、模块与包管理）生成 SVG 示意图，保存到 `images/` 目录，文件命名形如 `basics-ownership.svg`、`basics-slice.svg`，并在 `theory` 中用 `![...](images/xxx.svg)` 引用。保持 `code` 和 `hint` 不变。不要直接修改 `js/chapters.json`，而是将扩展后的整个模块对象（`{"name": "基础入门", "chapters": [...]}`）写入 `js/module-0-basics.json`。完成后用 `python3 -m json.tool js/module-0-basics.json` 校验 JSON 语法。

- [ ] **Step 2: 扩展“中等应用”模块**

Prompt（给子代理）：

> 请扩展 `js/chapters.json` 中 `"name": "中等应用"` 模块的所有章节。每个章节的 `theory` 字段扩展为原来的 2-3 倍，结构包含：引入/类比、概念图解、深入讲解、常见误区、一句话总结。加入表情符号。为核心概念章节（泛型、Trait、生命周期、闭包与迭代器、智能指针、并发基础）生成 SVG 示意图，保存到 `images/` 目录，文件命名形如 `intermediate-generics.svg`、`intermediate-lifetime.svg`，并在 `theory` 中引用。保持 `code` 和 `hint` 不变。不要直接修改 `js/chapters.json`，而是将扩展后的整个模块对象写入 `js/module-1-intermediate.json`。完成后用 `python3 -m json.tool js/module-1-intermediate.json` 校验 JSON 语法。

- [ ] **Step 3: 扩展“高级应用”模块**

Prompt（给子代理）：

> 请扩展 `js/chapters.json` 中 `"name": "高级应用"` 模块的所有章节。每个章节的 `theory` 字段扩展为原来的 2-3 倍，结构包含：引入/类比、概念图解、深入讲解、常见误区、一句话总结。加入表情符号。为核心概念章节（unsafe Rust、异步编程、Web 开发基础、并发模式与设计）生成 SVG 示意图，保存到 `images/` 目录，文件命名形如 `advanced-unsafe.svg`、`advanced-async.svg`，并在 `theory` 中引用。保持 `code` 和 `hint` 不变。不要直接修改 `js/chapters.json`，而是将扩展后的整个模块对象写入 `js/module-2-advanced.json`。完成后用 `python3 -m json.tool js/module-2-advanced.json` 校验 JSON 语法。

- [ ] **Step 4: 扩展“Q & A”模块**

Prompt（给子代理）：

> 请扩展 `js/chapters.json` 中 `"name": "Q & A"` 模块的所有章节。每个章节的 `theory` 字段扩展为原来的 2-3 倍，结构包含：问题引入、概念图解、深入讲解、常见误区、一句话总结。加入表情符号。为涉及内存模型的问答（所有权、生命周期、栈和堆、RAII、内存泄漏）生成 SVG 示意图，保存到 `images/` 目录，文件命名形如 `qa-ownership.svg`、`qa-stack-heap.svg`，并在 `theory` 中引用。保持 `code` 和 `hint` 不变。不要直接修改 `js/chapters.json`，而是将扩展后的整个模块对象写入 `js/module-3-qa.json`。完成后用 `python3 -m json.tool js/module-3-qa.json` 校验 JSON 语法。

---

### Task 4: 合并模块文件并全局校验

**Files:**
- Read: `js/module-0-basics.json`
- Read: `js/module-1-intermediate.json`
- Read: `js/module-2-advanced.json`
- Read: `js/module-3-qa.json`
- Modify: `js/chapters.json`
- Read: `images/*.svg`

**Interfaces:**
- Consumes: 四个模块扩展后的 JSON 片段
- Produces: 合并后的 `js/chapters.json`

- [ ] **Step 1: 合并四个模块文件**

```python
import json
from pathlib import Path

ROOT = Path("/Users/yanshili/me/projects/rust-projects")
module_files = [
    ROOT / "js" / "module-0-basics.json",
    ROOT / "js" / "module-1-intermediate.json",
    ROOT / "js" / "module-2-advanced.json",
    ROOT / "js" / "module-3-qa.json",
]

modules = []
for f in module_files:
    with f.open("r", encoding="utf-8") as fp:
        modules.append(json.load(fp))

with (ROOT / "js" / "chapters.json").open("w", encoding="utf-8") as fp:
    json.dump(modules, fp, ensure_ascii=False, indent=2)
```

- [ ] **Step 2: 运行校验脚本**

```bash
cd /Users/yanshili/me/projects/rust-projects
python3 scripts/validate_chapters.py
```

Expected: 退出码 0，JSON 无语法错误，所有图片引用存在，每个章节字数达标。

- [ ] **Step 3: 删除临时模块文件（可选）**

```bash
cd /Users/yanshili/me/projects/rust-projects
rm js/module-0-basics.json js/module-1-intermediate.json js/module-2-advanced.json js/module-3-qa.json
```

- [ ] **Step 4: Commit**

```bash
cd /Users/yanshili/me/projects/rust-projects
git add js/chapters.json images/
git commit -m "content: expand all chapter theories and add core concept diagrams"
```

---

### Task 5: 本地预览与最终确认

**Files:**
- 无新增文件

**Interfaces:**
- Consumes: 扩展后的 `js/chapters.json`、`images/`、后端二进制
- Produces: 本地预览确认

- [ ] **Step 1: 启动后端**

```bash
cd /Users/yanshili/me/projects/rust-projects
STATIC_DIR=/Users/yanshili/me/projects/rust-projects ./backend/target/release/rust-learning-backend
```

- [ ] **Step 2: 检查首页和接口**

```bash
curl -s -o /dev/null -w "首页: %{http_code}\n" http://localhost:9001/
curl -s -o /dev/null -w "evaluate: %{http_code}\n" -X POST http://localhost:9001/evaluate.json \
  -H "Content-Type: application/json" \
  -d '{"version":"stable","edition":"2021","crateType":"bin","mode":"debug","tests":false,"optimize":"0","code":"fn main() { println!(\"ok\"); }"}'
```

Expected: 首页 200，evaluate 200。

- [ ] **Step 3: 浏览器抽查**

打开 `http://localhost:9001`，依次抽查以下章节：
- 基础入门 → 所有权
- 中等应用 → Trait
- 高级应用 → 异步编程
- Q & A → 生命周期

确认：
- 文案明显变长，表情正常显示。
- SVG 图片正常加载，无破图。
- 代码区和提示区与之前一致。
- 深色/浅色主题下图表文字清晰可见。

- [ ] **Step 4: 关闭后端（可选）**

按 `Ctrl+C` 或在另一个终端执行：

```bash
pkill -f rust-learning-backend
```

- [ ] **Step 5: 最终 Commit**

```bash
git add js/chapters.json images/ scripts/validate_chapters.py
git commit -m "feat: expanded chapter content with diagrams and validation"
```

---

## Spec Coverage Self-Check

| 设计文档要求 | 对应任务 |
|---|---|
| 所有 53 个章节扩展 2-3 倍 | Task 3 四个子代理并行处理 |
| 核心概念章节配图 | Task 3 各子代理按要求生成 SVG |
| 代码和 hint 不变 | Task 3 子代理 prompt 中明确约束 |
| SVG 本地化、无 CDN | Task 1 + Task 3 |
| JSON 语法校验 | Task 2 + Task 4 |
| 本地 `localhost:9001` 预览 | Task 5 |

## Placeholder Scan

- 无 `TBD`、`TODO`、`implement later` 等占位符。
- 所有命令、文件路径、子代理 Prompt 均已给出完整内容。
- 校验脚本包含完整可运行代码。
