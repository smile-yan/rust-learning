import { createApp, ref, computed, onMounted, watch, nextTick } from "vue";
import { EditorView, basicSetup, rust, oneDark, keymap, syntaxHighlighting, HighlightStyle, tags } from "../libs/codemirror-bundle.js?v=3";

const { marked } = window;

const THEME_KEY = "rust-learning-theme";

function getPreferredTheme() {
  const saved = localStorage.getItem(THEME_KEY);
  if (saved === "light" || saved === "dark") {
    return saved;
  }
  return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
}

createApp({
  setup() {
    const modules = ref([]);
    const currentGlobalIdx = ref(0);
    const theme = ref(getPreferredTheme());
    const menuOpen = ref(false);
    const outputText = ref("点击「运行」按钮查看输出结果");
    const outputClass = ref("text-gray-400");
    const loadError = ref("");
    const editorEl = ref(null);
    const theoryEl = ref(null);
    let editor = null;

    const TAB_INDENT = "    ";

    function insertTab(view) {
      const { state, dispatch } = view;
      const selection = state.selection.main;

      if (selection.empty) {
        dispatch({
          changes: { from: selection.head, insert: TAB_INDENT }
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
      dispatch({ changes });
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
        dispatch({ changes });
      }
      return true;
    }

    document.documentElement.setAttribute("data-theme", theme.value);

    const allChapters = computed(() => {
      const list = [];
      modules.value.forEach((mod, moduleIdx) => {
        mod.chapters.forEach((ch, chapterIdxInModule) => {
          list.push({
            ...ch,
            moduleName: mod.name,
            moduleIdx,
            chapterIdxInModule,
            globalIdx: list.length
          });
        });
      });
      return list;
    });

    const chapterIndexMap = computed(() => {
      const map = new Map();
      let globalIdx = 0;
      modules.value.forEach((mod, moduleIdx) => {
        mod.chapters.forEach((_, chapterIdxInModule) => {
          map.set(`${moduleIdx}-${chapterIdxInModule}`, globalIdx++);
        });
      });
      return map;
    });

    const currentChapter = computed(() => allChapters.value[currentGlobalIdx.value]);

    const renderedTheory = computed(() => {
      const ch = currentChapter.value;
      return ch ? marked.parse(ch.theory) : "";
    });

    function getGlobalIdx(moduleIdx, chapterIdxInModule) {
      return chapterIndexMap.value.get(`${moduleIdx}-${chapterIdxInModule}`);
    }

    function getModuleChapterFromGlobal(globalIdx) {
      const ch = allChapters.value[globalIdx];
      return ch ? { moduleIdx: ch.moduleIdx, chapterIdxInModule: ch.chapterIdxInModule } : null;
    }

    function updateUrlHash(globalIdx) {
      const loc = getModuleChapterFromGlobal(globalIdx);
      if (loc) {
        window.location.hash = `#chapter/${loc.moduleIdx}/${loc.chapterIdxInModule}`;
      }
    }

    function parseUrlHash() {
      const match = window.location.hash.match(/^#chapter\/(\d+)\/(\d+)$/);
      if (match) {
        const moduleIdx = parseInt(match[1], 10);
        const chapterIdxInModule = parseInt(match[2], 10);
        const globalIdx = getGlobalIdx(moduleIdx, chapterIdxInModule);
        if (globalIdx !== undefined && globalIdx >= 0 && globalIdx < allChapters.value.length) {
          return globalIdx;
        }
      }
      return null;
    }

    function initEditorWithDoc(doc = "", isDark = true) {
      const themeExtension = EditorView.theme({
        "&": { height: "100%" },
        ".cm-scroller": { overflow: "auto" },
        ".cm-content": {
          fontFamily: "'JetBrains Mono', 'Fira Code', Consolas, monospace",
          fontSize: "14px",
          backgroundColor: isDark ? "transparent" : "#ffffff",
          color: isDark ? "#e2e8f0" : "#1e293b"
        },
        ".cm-gutters": {
          backgroundColor: isDark ? "#111827" : "#f8fafc",
          color: isDark ? "#6b7280" : "#94a3b8",
          borderRight: isDark ? "1px solid #374151" : "1px solid #e2e8f0"
        },
        ".cm-activeLineGutter": {
          backgroundColor: isDark ? "#1f2937" : "#e2e8f0"
        },
        ".cm-activeLine": {
          backgroundColor: isDark ? "#1f2937" : "#f1f5f9"
        },
        ".cm-selectionBackground": {
          backgroundColor: isDark ? "#2563eb66" : "#bfdbfe"
        },
        ".cm-cursor": {
          borderLeftColor: isDark ? "#e2e8f0" : "#1e293b"
        },
        ".cm-lineNumbers": {
          color: isDark ? "#6b7280" : "#94a3b8"
        }
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

      const extensions = [
        basicSetup,
        rust(),
        themeExtension,
        isDark ? oneDark : syntaxHighlighting(lightHighlight, { fallback: true }),
        keymap.of([
          {
            key: "Tab",
            run: insertTab
          },
          {
            key: "Shift-Tab",
            run: unindent
          },
          {
            key: "Ctrl-Enter",
            run: () => {
              runCode();
              return true;
            }
          },
          {
            key: "Cmd-Enter",
            run: () => {
              runCode();
              return true;
            }
          }
        ])
      ];
      editor = new EditorView({
        doc,
        extensions,
        parent: editorEl.value
      });
    }

    async function loadData() {
      const res = await fetch("./js/chapters.json?v=3");
      if (!res.ok) {
        throw new Error(`无法加载章节数据: HTTP ${res.status}`);
      }
      modules.value = await res.json();
    }

    function loadChapter(globalIdx) {
      currentGlobalIdx.value = globalIdx;
      updateUrlHash(globalIdx);
      clearOutput();
      if (theoryEl.value) {
        theoryEl.value.scrollTop = 0;
      }
      if (window.innerWidth < 768) {
        closeMenu();
      }
    }

    watch(currentChapter, (ch) => {
      if (editor && ch) {
        editor.dispatch({
          changes: {
            from: 0,
            to: editor.state.doc.length,
            insert: ch.code
          }
        });
      }
    });

    watch(theme, (newTheme) => {
      if (editor) {
        const currentDoc = editor.state.doc.toString();
        editor.destroy();
        initEditorWithDoc(currentDoc, newTheme === "dark");
      }
    });

    watch(renderedTheory, async () => {
      await nextTick();
      if (theoryEl.value && window.Prism) {
        // 没有语言标记的代码块默认按 Rust 高亮
        theoryEl.value.querySelectorAll('pre code:not([class*="language-"])').forEach((block) => {
          block.classList.add('language-rust');
        });
        window.Prism.highlightAllUnder(theoryEl.value);
      }
    });

    async function runCode() {
      outputText.value = "⏳ 正在编译运行，请稍候...";
      outputClass.value = "text-gray-200";

      const code = editor.state.doc.toString();
      const evaluateUrl = (window.RUST_PLAYGROUND && window.RUST_PLAYGROUND.evaluateUrl) || "/evaluate.json";

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

        if (!res.ok) {
          throw new Error(`服务器返回 HTTP ${res.status}`);
        }

        const data = await res.json();
        renderOutput(data);
      } catch (err) {
        if (err.name === "AbortError") {
          outputText.value = "❌ 请求超时，请检查网络或稍后重试。";
        } else {
          outputText.value = `❌ 运行失败: ${err.message}`;
        }
        outputClass.value = "text-red-400";
      }
    }

    function renderOutput(data) {
      // 支持两种响应格式：
      // 1. 新版后端：{ success, stdout, stderr, error }
      // 2. 旧版后端：{ result, error }
      const stdout = typeof data.stdout === "string" ? data.stdout : "";
      const stderr = typeof data.stderr === "string" ? data.stderr : "";
      const result = typeof data.result === "string" ? data.result : "";
      const error = data.error !== null && data.error !== undefined ? String(data.error) : "";

      const output = stdout || result;

      if (error.length > 0) {
        outputText.value = error;
        outputClass.value = "text-red-400";
      } else if (stderr.length > 0 && output.length === 0) {
        outputText.value = stderr;
        outputClass.value = "text-orange-400";
      } else if (output.length > 0) {
        outputText.value = stderr.length > 0 ? `${output}\n${stderr}` : output;
        outputClass.value = "text-green-400";
      } else {
        outputText.value = "（程序没有输出）";
        outputClass.value = "text-gray-200";
      }
    }

    function clearOutput() {
      outputText.value = "点击「运行」按钮查看输出结果";
      outputClass.value = "text-gray-400";
    }

    function toggleTheme() {
      const next = theme.value === "light" ? "dark" : "light";
      localStorage.setItem(THEME_KEY, next);
      theme.value = next;
      document.documentElement.setAttribute("data-theme", next);
    }

    function toggleMenu() {
      menuOpen.value = !menuOpen.value;
    }

    function closeMenu() {
      menuOpen.value = false;
    }

    onMounted(async () => {
      initEditorWithDoc("", theme.value === "dark");
      try {
        await loadData();
        const hashIdx = parseUrlHash();
        currentGlobalIdx.value = hashIdx !== null ? hashIdx : 0;

        window.addEventListener("hashchange", () => {
          const idx = parseUrlHash();
          if (idx !== null && idx !== currentGlobalIdx.value) {
            currentGlobalIdx.value = idx;
            clearOutput();
            if (theoryEl.value) {
              theoryEl.value.scrollTop = 0;
            }
          }
        });
      } catch (err) {
        loadError.value = err.message;
      }
    });

    return {
      modules,
      currentGlobalIdx,
      theme,
      menuOpen,
      outputText,
      outputClass,
      loadError,
      editorEl,
      theoryEl,
      currentChapter,
      renderedTheory,
      getGlobalIdx,
      loadChapter,
      runCode,
      clearOutput,
      toggleTheme,
      toggleMenu,
      closeMenu
    };
  }
}).mount("#app");
