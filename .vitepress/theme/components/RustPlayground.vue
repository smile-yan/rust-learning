<script setup>
import { ref, computed, onMounted, onBeforeUnmount, watch } from "vue";
import { useData } from "vitepress";
import { EditorView, keymap } from "@codemirror/view";
import { basicSetup } from "codemirror";
import { rust } from "@codemirror/lang-rust";
import { oneDark } from "@codemirror/theme-one-dark";
import { syntaxHighlighting, HighlightStyle, indentUnit } from "@codemirror/language";
import { tags } from "@lezer/highlight";

const { frontmatter, isDark } = useData();

const fm = computed(() => frontmatter.value);
const initialCode = computed(() => fm.value.code || "");
const hint = computed(() => fm.value.hint || "");
const exercises = computed(() =>
  Array.isArray(fm.value.exercises) ? fm.value.exercises : []
);

const editorEl = ref(null);
const outputText = ref("点击「运行」按钮查看输出结果");
const outputClass = ref("output-muted");
const elapsedTime = ref(null);
const hintVisible = ref(false);
let editor = null;

const TAB_INDENT = "    ";

function insertTab(view) {
  const { state, dispatch } = view;
  const selection = state.selection.main;
  if (selection.empty) {
    const insertLen = TAB_INDENT.length;
    dispatch({
      changes: { from: selection.head, insert: TAB_INDENT },
      selection: { anchor: selection.head + insertLen, head: selection.head + insertLen },
      userEvent: "input.indent"
    });
    return true;
  }
  const changes = [];
  let pos = selection.from;
  while (pos <= selection.to) {
    const line = state.doc.lineAt(pos);
    changes.push({ from: line.from, insert: TAB_INDENT });
    pos = line.to + 1;
  }
  dispatch({
    changes,
    selection: selection.map(state.changes(changes)),
    userEvent: "input.indent"
  });
  return true;
}

function unindent(view) {
  const { state, dispatch } = view;
  const selection = state.selection.main;
  const changes = [];
  let pos = selection.from;
  while (pos <= selection.to) {
    const line = state.doc.lineAt(pos);
    let remove = 0;
    while (remove < TAB_INDENT.length && remove < line.text.length && line.text[remove] === " ") {
      remove++;
    }
    if (remove > 0) {
      changes.push({ from: line.from, to: line.from + remove });
    }
    pos = line.to + 1;
  }
  if (changes.length > 0) {
    dispatch({
      changes,
      selection: selection.map(state.changes(changes)),
      userEvent: "input.indent"
    });
  }
  return true;
}

function buildExtensions(dark) {
  const themeExtension = EditorView.theme({
    "&": { height: "100%" },
    ".cm-scroller": { overflow: "auto" },
    ".cm-content": {
      fontFamily: "'JetBrains Mono', 'Fira Code', Consolas, monospace",
      fontSize: "14px",
      backgroundColor: "transparent",
      color: dark ? "#e2e8f0" : "#1e293b"
    },
    ".cm-gutters": {
      backgroundColor: dark ? "#111827" : "#f8fafc",
      color: dark ? "#6b7280" : "#94a3b8",
      borderRight: dark ? "1px solid #374151" : "1px solid #e2e8f0"
    },
    ".cm-activeLineGutter": { backgroundColor: dark ? "#1f2937" : "#e2e8f0" },
    ".cm-activeLine": { backgroundColor: dark ? "#1f2937" : "#f1f5f9" },
    ".cm-selectionBackground": { backgroundColor: dark ? "#2563eb66" : "#bfdbfe" },
    ".cm-cursor": { borderLeftColor: dark ? "#e2e8f0" : "#1e293b" },
    ".cm-lineNumbers": { color: dark ? "#6b7280" : "#94a3b8" }
  });

  const lightHighlight = HighlightStyle.define([
    { tag: tags.keyword, color: "#d73a49" },
    { tag: tags.controlKeyword, color: "#d73a49" },
    { tag: tags.typeName, color: "#6f42c1" },
    { tag: tags.className, color: "#6f42c1" },
    { tag: tags.tagName, color: "#22863a" },
    { tag: tags.name, color: "#24292e" },
    { tag: tags.variableName, color: "#24292e" },
    { tag: tags.string, color: "#032f62" },
    { tag: tags.comment, color: "#6a737d", fontStyle: "italic" },
    { tag: tags.number, color: "#005cc5" },
    { tag: tags.operator, color: "#d73a49" },
    { tag: tags.punctuation, color: "#24292e" },
    { tag: tags.function(tags.variableName), color: "#6f42c1" },
    { tag: tags.propertyName, color: "#005cc5" }
  ]);

  return [
    basicSetup,
    rust(),
    indentUnit.of(TAB_INDENT),
    themeExtension,
    dark ? oneDark : syntaxHighlighting(lightHighlight, { fallback: true }),
    keymap.of([
      { key: "Tab", run: insertTab, preventDefault: true },
      { key: "Shift-Tab", run: unindent, preventDefault: true },
      { key: "Ctrl-Enter", run: () => { runCode(); return true; }, preventDefault: true },
      { key: "Cmd-Enter", run: () => { runCode(); return true; }, preventDefault: true }
    ])
  ];
}

function initEditor(doc) {
  editor = new EditorView({
    doc,
    extensions: buildExtensions(isDark.value),
    parent: editorEl.value
  });
}

async function runCode() {
  if (!editor) return;
  outputText.value = "⏳ 正在编译运行，请稍候...";
  outputClass.value = "output-plain";
  const startTime = performance.now();
  const code = editor.state.doc.toString();
  const evaluateUrl =
    (window.RUST_PLAYGROUND && window.RUST_PLAYGROUND.evaluateUrl) || "/evaluate.json";

  try {
    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), 25000);
    const res = await fetch(evaluateUrl, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        version: "stable",
        edition: "2021",
        crateType: "bin",
        mode: "debug",
        tests: false,
        optimize: "0",
        code
      }),
      signal: controller.signal
    });
    clearTimeout(timeoutId);
    if (!res.ok) throw new Error(`服务器返回 HTTP ${res.status}`);
    renderOutput(await res.json());
  } catch (err) {
    outputText.value =
      err.name === "AbortError"
        ? "❌ 请求超时，请检查网络或稍后重试。"
        : `❌ 运行失败: ${err.message}`;
    outputClass.value = "output-error";
  } finally {
    elapsedTime.value = (performance.now() - startTime) / 1000;
  }
}

function renderOutput(data) {
  const stdout = typeof data.stdout === "string" ? data.stdout : "";
  const stderr = typeof data.stderr === "string" ? data.stderr : "";
  const result = typeof data.result === "string" ? data.result : "";
  const error = data.error !== null && data.error !== undefined ? String(data.error) : "";
  const output = stdout || result;

  if (error.length > 0) {
    outputText.value = error;
    outputClass.value = "output-error";
  } else if (stderr.length > 0 && output.length === 0) {
    outputText.value = stderr;
    outputClass.value = "output-warn";
  } else if (output.length > 0) {
    outputText.value = stderr.length > 0 ? `${output}\n${stderr}` : output;
    outputClass.value = "output-ok";
  } else {
    outputText.value = "（程序没有输出）";
    outputClass.value = "output-plain";
  }
}

function clearOutput() {
  outputText.value = "点击「运行」按钮查看输出结果";
  outputClass.value = "output-muted";
  elapsedTime.value = null;
}

function loadExercise(ex) {
  if (editor && ex && typeof ex.code_template === "string") {
    editor.dispatch({
      changes: { from: 0, to: editor.state.doc.length, insert: ex.code_template }
    });
    clearOutput();
    editorEl.value?.scrollIntoView({ behavior: "smooth", block: "start" });
  }
}

function resetCode() {
  if (editor) {
    editor.dispatch({
      changes: { from: 0, to: editor.state.doc.length, insert: initialCode.value }
    });
    clearOutput();
  }
}

watch(isDark, () => {
  if (editor) {
    const doc = editor.state.doc.toString();
    editor.destroy();
    initEditor(doc);
  }
});

onMounted(() => initEditor(initialCode.value));
onBeforeUnmount(() => editor?.destroy());
</script>

<template>
  <div class="rust-playground">
    <div v-if="exercises.length" class="exercises">
      <div class="section-title">课后练习</div>
      <div v-for="(ex, i) in exercises" :key="i" class="exercise-item">
        <div class="exercise-head">
          <span class="exercise-title">{{ i + 1 }}. {{ ex.title }}</span>
          <button class="btn" @click="loadExercise(ex)">载入编辑器</button>
        </div>
        <div class="exercise-desc">{{ ex.description }}</div>
      </div>
    </div>

    <div class="toolbar">
      <button class="btn btn-primary" @click="runCode">▶ 运行 (Ctrl/Cmd+Enter)</button>
      <button class="btn" @click="resetCode">重置代码</button>
      <button class="btn" @click="clearOutput">清空输出</button>
      <button v-if="hint" class="btn" @click="hintVisible = !hintVisible">提示</button>
      <span v-if="elapsedTime !== null" class="elapsed">{{ elapsedTime.toFixed(2) }}s</span>
    </div>

    <div v-if="hint && hintVisible" class="hint">💡 {{ hint }}</div>

    <div ref="editorEl" class="editor"></div>

    <pre class="output" :class="outputClass">{{ outputText }}</pre>
  </div>
</template>

<style scoped>
.rust-playground {
  margin: 24px 0;
  border: 1px solid var(--vp-c-divider);
  border-radius: 8px;
  overflow: hidden;
}
.section-title {
  font-weight: 600;
  padding: 12px 16px 0;
}
.exercises {
  border-bottom: 1px solid var(--vp-c-divider);
  padding-bottom: 8px;
}
.exercise-item {
  padding: 8px 16px;
}
.exercise-head {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 8px;
}
.exercise-title {
  font-weight: 500;
}
.exercise-desc {
  color: var(--vp-c-text-2);
  font-size: 14px;
  margin-top: 4px;
}
.toolbar {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
  padding: 12px 16px;
}
.btn {
  border: 1px solid var(--vp-c-divider);
  border-radius: 6px;
  padding: 4px 12px;
  font-size: 13px;
  cursor: pointer;
  background: var(--vp-c-bg-soft);
  color: var(--vp-c-text-1);
}
.btn:hover {
  border-color: var(--vp-c-brand-1);
}
.btn-primary {
  background: var(--vp-c-brand-1);
  border-color: var(--vp-c-brand-1);
  color: #fff;
}
.elapsed {
  color: var(--vp-c-text-2);
  font-size: 13px;
  margin-left: auto;
}
.hint {
  margin: 0 16px 12px;
  padding: 8px 12px;
  border-radius: 6px;
  background: var(--vp-c-bg-soft);
  font-size: 14px;
}
.editor {
  height: 320px;
  border-top: 1px solid var(--vp-c-divider);
  border-bottom: 1px solid var(--vp-c-divider);
}
html.dark .editor {
  background: #111827;
}
.output {
  margin: 0;
  padding: 12px 16px;
  font-family: 'JetBrains Mono', 'Fira Code', Consolas, monospace;
  font-size: 13px;
  white-space: pre-wrap;
  min-height: 48px;
}
.output-muted { color: var(--vp-c-text-3); }
.output-plain { color: var(--vp-c-text-1); }
.output-ok { color: #4ade80; }
.output-warn { color: #fb923c; }
.output-error { color: #f87171; }
</style>
