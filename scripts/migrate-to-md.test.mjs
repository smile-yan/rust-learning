import { test } from "node:test";
import assert from "node:assert/strict";
import { mkdtempSync, readFileSync, existsSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { writeFileSync } from "node:fs";
import {
  slugify,
  buildFrontMatter,
  buildMarkdown,
  main
} from "./migrate-to-md.mjs";

test("slugify 提取 ASCII 词", () => {
  assert.equal(slugify("Hello World println!"), "hello-world-println");
  assert.equal(slugify("常量与变量 let / let mut"), "let-let-mut");
  assert.equal(slugify("Q & A 入门"), "q-and-a");
});

test("slugify 纯中文标题回退为 chapter", () => {
  assert.equal(slugify("所有权"), "chapter");
});

test("buildFrontMatter 生成合法结构", () => {
  const fm = buildFrontMatter(
    {
      title: '常量与变量 let / let mut',
      code: 'fn main() {\n    let x = 5;\n}',
      hint: '注意 mut。',
      exercises: [
        { title: '累加器', description: '循环 5 次。', code_template: 'fn main() {\n}' }
      ]
    },
    "基础入门",
    2
  );
  assert.match(fm, /^---\n/);
  assert.match(fm, /title: "常量与变量 let \/ let mut"\n/);
  assert.match(fm, /module: "基础入门"\n/);
  assert.match(fm, /order: 2\n/);
  // code 块标量：key 后换行，内容每行缩进 2 空格
  assert.match(fm, /code: \|\n  fn main\(\) \{\n      let x = 5;\n  \}\n/);
  // exercises 中 code_template 的缩进深于 key（key 在 4 空格处，内容至少 6 空格）
  assert.match(fm, /    code_template: \|\n      fn main\(\) \{\n      \}\n/);
  assert.match(fm, /\n---\n$/);
});

test("buildFrontMatter 无 code/exercises 时不出现对应字段与组件标记", () => {
  const fm = buildFrontMatter({ title: "Q&A", theory: "x" }, "Q & A", 1);
  assert.ok(!fm.includes("code:"));
  assert.ok(!fm.includes("exercises:"));
});

test("buildMarkdown 改写图片路径并按条件追加组件", () => {
  const withCode = buildMarkdown(
    { title: "t", theory: "\n# t\n\n![图](images/a.svg)\n", code: "fn main() {}" },
    "基础入门",
    1
  );
  assert.ok(withCode.includes("](/images/a.svg)"));
  assert.ok(withCode.trimEnd().endsWith("<RustPlayground />"));

  const noCode = buildMarkdown({ title: "t", theory: "# t\n" }, "Q & A", 1);
  assert.ok(!noCode.includes("RustPlayground"));
});

test("buildMarkdown 转义正文中裸露的泛型尖括号", () => {
  const md = buildMarkdown(
    {
      title: "t",
      theory: [
        "### Box<T>",
        "![Rc<T> 图](images/a.svg)",
        "行内代码 `Rc<T>` 不转义，已转义 \\<T\\> 不重复",
        "```rust",
        "let x: Vec<T> = vec![]; // 代码围栏内不转义",
        "```"
      ].join("\n")
    },
    "中等应用",
    5
  );
  assert.ok(md.includes("### Box\\<T>"));
  // 图片 alt 保持原始 <T>，markdown-it 生成 alt 属性时自行转义
  assert.ok(md.includes("![Rc<T> 图](/images/a.svg)"));
  assert.ok(md.includes("`Rc<T>`"));
  assert.ok(md.includes("\\<T\\> 不重复"));
  assert.ok(md.includes("let x: Vec<T> = vec![];"));
});

test("main 端到端：fixture json 生成 md 与 sidebar.ts", () => {
  const dir = mkdtempSync(join(tmpdir(), "migrate-"));
  const fixture = join(dir, "chapters.json");
  writeFileSync(
    fixture,
    JSON.stringify([
      {
        name: "基础入门",
        chapters: [
          {
            title: "Hello World println!",
            theory: "\n# Hello\n\n![图](images/a.svg)\n",
            code: "fn main() {}",
            hint: "h",
            exercises: [
              { title: "练习1", description: "d", code_template: "fn main() {\n}" }
            ]
          }
        ]
      }
    ])
  );
  main([fixture, dir]);
  const mdPath = join(dir, "chapters", "basic", "01-hello-world-println.md");
  assert.ok(existsSync(mdPath));
  const md = readFileSync(mdPath, "utf8");
  assert.ok(md.includes('title: "Hello World println!"'));
  assert.ok(md.includes("](/images/a.svg)"));
  assert.ok(md.includes("<RustPlayground />"));
  const sidebar = readFileSync(join(dir, ".vitepress", "sidebar.ts"), "utf8");
  assert.ok(sidebar.includes("/chapters/basic/01-hello-world-println"));
  assert.ok(sidebar.includes("基础入门"));
});
