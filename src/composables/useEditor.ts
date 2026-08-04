import { onMounted, onUnmounted, nextTick, type Ref } from "vue";
import { EditorView, keymap } from "@codemirror/view";
import { basicSetup } from "codemirror";
import { oneDark } from "@codemirror/theme-one-dark";
import { rust } from "@codemirror/lang-rust";
import { syntaxHighlighting, HighlightStyle, indentUnit } from "@codemirror/language";
import { tags } from "@lezer/highlight";

const TAB_INDENT = "    ";

function insertTab(view: EditorView) {
  const { state, dispatch } = view;
  const selection = state.selection.main;

  if (selection.empty) {
    dispatch({
      changes: { from: selection.head, insert: TAB_INDENT },
      selection: {
        anchor: selection.head + TAB_INDENT.length,
        head: selection.head + TAB_INDENT.length,
      },
      userEvent: "input.indent",
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
    userEvent: "input.indent",
  });
  return true;
}

function unindent(view: EditorView) {
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
      userEvent: "input.indent",
    });
  }
  return true;
}

function themeExtension(isDark: boolean) {
  return EditorView.theme({
    "&": { height: "100%" },
    ".cm-scroller": { overflow: "auto" },
    ".cm-content": {
      fontFamily: "'JetBrains Mono', 'Fira Code', Consolas, monospace",
      fontSize: "14px",
      backgroundColor: isDark ? "transparent" : "#ffffff",
      color: isDark ? "#e2e8f0" : "#1e293b",
    },
    ".cm-gutters": {
      backgroundColor: isDark ? "#111827" : "#f8fafc",
      color: isDark ? "#6b7280" : "#94a3b8",
      borderRight: isDark ? "1px solid #374151" : "1px solid #e2e8f0",
    },
    ".cm-activeLineGutter": { backgroundColor: isDark ? "#1f2937" : "#e2e8f0" },
    ".cm-activeLine": { backgroundColor: isDark ? "#1f2937" : "#f1f5f9" },
    ".cm-selectionBackground": { backgroundColor: isDark ? "#2563eb66" : "#bfdbfe" },
    ".cm-cursor": { borderLeftColor: isDark ? "#e2e8f0" : "#1e293b" },
    ".cm-lineNumbers": { color: isDark ? "#6b7280" : "#94a3b8" },
  });
}

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
  { tag: tags.propertyName, color: "#005cc5" },
]);

export function useEditor(
  containerRef: Ref<HTMLElement | null>,
  options: { runCode: () => void }
) {
  let editor: EditorView | null = null;

  function createExtensions(isDark: boolean) {
    return [
      basicSetup,
      rust(),
      indentUnit.of(TAB_INDENT),
      themeExtension(isDark),
      isDark ? oneDark : syntaxHighlighting(lightHighlight, { fallback: true }),
      keymap.of([
        { key: "Tab", run: insertTab, preventDefault: true },
        { key: "Shift-Tab", run: unindent, preventDefault: true },
        {
          key: "Ctrl-Enter",
          run: () => {
            options.runCode();
            return true;
          },
          preventDefault: true,
        },
        {
          key: "Cmd-Enter",
          run: () => {
            options.runCode();
            return true;
          },
          preventDefault: true,
        },
      ]),
    ];
  }

  function init(isDark: boolean) {
    if (!containerRef.value) return;
    editor = new EditorView({
      doc: "",
      extensions: createExtensions(isDark),
      parent: containerRef.value,
    });
  }

  onMounted(() => {
    init(true);
  });

  onUnmounted(() => {
    editor?.destroy();
    editor = null;
  });

  function setCode(code: string) {
    if (!editor) return;
    editor.dispatch({
      changes: { from: 0, to: editor.state.doc.length, insert: code },
    });
  }

  function getCode(): string {
    return editor?.state.doc.toString() ?? "";
  }

  function setTheme(isDark: boolean) {
    if (!editor) {
      init(isDark);
      return;
    }
    const doc = editor.state.doc.toString();
    editor.destroy();
    init(isDark);
    if (editor && doc) {
      editor.dispatch({ changes: { from: 0, to: editor.state.doc.length, insert: doc } });
    }
  }

  function requestMeasure() {
    nextTick(() => editor?.requestMeasure());
  }

  return { setCode, getCode, setTheme, requestMeasure };
}
