#!/usr/bin/env node
// 一次性迁移脚本：js/chapters.json -> chapters/**/*.md + .vitepress/sidebar.ts
// 用法：node scripts/migrate-to-md.mjs [jsonPath] [projectRoot]
import { readFileSync, writeFileSync, mkdirSync, rmSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const MODULE_SLUGS = {
  "基础入门": "basic",
  "中等应用": "intermediate",
  "高级应用": "advanced",
  "Rust 哲学": "philosophy",
  "Q & A": "qa"
};

export function slugify(title) {
  const ascii = String(title)
    .toLowerCase()
    .replace(/&/g, " and ")
    .match(/[a-z0-9]+/g);
  return ascii ? ascii.join("-") : "chapter";
}

// 多行文本转为 YAML 块标量内容，每行缩进 spaces 空格（空行不缩进）
function indentBlock(text, spaces) {
  const pad = " ".repeat(spaces);
  return String(text)
    .replace(/\s+$/, "")
    .split("\n")
    .map((line) => (line.trim() ? pad + line : ""))
    .join("\n");
}

// JSON 字符串是合法的 YAML 双引号标量
const yamlStr = (s) => JSON.stringify(String(s));

export function buildFrontMatter(ch, moduleName, order) {
  let out = "---\n";
  out += `title: ${yamlStr(ch.title)}\n`;
  out += `module: ${yamlStr(moduleName)}\n`;
  out += `order: ${order}\n`;
  if (ch.code) {
    out += `code: |\n${indentBlock(ch.code, 2)}\n`;
  }
  if (ch.hint) {
    out += `hint: ${yamlStr(ch.hint)}\n`;
  }
  if (Array.isArray(ch.exercises) && ch.exercises.length > 0) {
    out += "exercises:\n";
    for (const ex of ch.exercises) {
      out += `  - title: ${yamlStr(ex.title)}\n`;
      out += `    description: ${yamlStr(ex.description)}\n`;
      out += `    code_template: |\n${indentBlock(ex.code_template, 6)}\n`;
    }
  }
  out += "---\n";
  return out;
}

// 正文中裸露的 <T> / <K, V> 这类泛型写法会被 Vue 编译器当作未闭合标签导致构建失败，
// 在代码围栏、行内代码与图片 alt 之外把 `<`（后跟 ASCII 字母）转义为 \<
// （与源数据 `\<T\>` 的既有写法一致，标题 slug / aria-label 也保持干净）。
// 注意：图片 alt 不能转义——markdown-it 生成 alt 属性时会正确转义原始 `<T>`，
// 而转义形式（\< 或 &lt;）经 renderInlineAsText 处理后 `<` 反而丢失
function escapeRawAngles(text) {
  let inFence = false;
  return String(text)
    .split("\n")
    .map((line) => {
      if (line.trimStart().startsWith("```")) {
        inFence = !inFence;
        return line;
      }
      if (inFence) return line;
      return line.replace(
        /(`[^`]*`)|(!\[[^\]]*\])|(?<!\\)<(?=[A-Za-z])/g,
        (m, code, alt) => (code || alt ? m : "\\<")
      );
    })
    .join("\n");
}

export function buildMarkdown(ch, moduleName, order) {
  const body = escapeRawAngles(
    String(ch.theory || "")
      .trim()
      .replace(/]\(images\//g, "](/images/")
  );
  const hasPlayground = Boolean(ch.code) || (Array.isArray(ch.exercises) && ch.exercises.length > 0);
  let md = buildFrontMatter(ch, moduleName, order) + "\n" + body + "\n";
  if (hasPlayground) {
    md += "\n<RustPlayground />\n";
  }
  return md;
}

export function main(argv = process.argv.slice(2)) {
  const root = dirname(dirname(fileURLToPath(import.meta.url)));
  const jsonPath = argv[0] || join(root, "js", "chapters.json");
  const outRoot = argv[1] || root;

  const modules = JSON.parse(readFileSync(jsonPath, "utf8"));

  // 防御性校验：清目录之前先确认所有模块名已知且 chapters 是数组，
  // 避免 JSON 损坏时旧的 chapters/ 产物已被删除才报错
  for (const mod of modules) {
    if (!MODULE_SLUGS[mod.name]) throw new Error(`未知模块名: ${mod.name}`);
    if (!Array.isArray(mod.chapters)) throw new Error(`模块 ${mod.name} 的 chapters 不是数组`);
  }

  const chaptersDir = join(outRoot, "chapters");
  rmSync(chaptersDir, { recursive: true, force: true });

  const sidebar = [];
  let count = 0;
  for (const mod of modules) {
    const slug = MODULE_SLUGS[mod.name];
    if (!slug) throw new Error(`未知模块名: ${mod.name}`);
    const items = [];
    mod.chapters.forEach((ch, i) => {
      const order = i + 1;
      const filename = `${String(order).padStart(2, "0")}-${slugify(ch.title)}.md`;
      const rel = `chapters/${slug}/${filename}`;
      const abs = join(outRoot, rel);
      mkdirSync(dirname(abs), { recursive: true });
      writeFileSync(abs, buildMarkdown(ch, mod.name, order));
      items.push({ text: ch.title, link: `/${rel.replace(/\.md$/, "")}` });
      count++;
    });
    sidebar.push({ text: mod.name, items });
  }

  const sidebarTs =
    "// 本文件由 scripts/migrate-to-md.mjs 生成，请勿手改\n" +
    "export const sidebar = " +
    JSON.stringify(sidebar, null, 2) +
    ";\n";
  mkdirSync(join(outRoot, ".vitepress"), { recursive: true });
  writeFileSync(join(outRoot, ".vitepress", "sidebar.ts"), sidebarTs);

  console.log(`已生成 ${count} 个章节 md 与 .vitepress/sidebar.ts`);
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  main();
}
