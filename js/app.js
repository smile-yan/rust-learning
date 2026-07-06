import { createApp, ref, computed, onMounted, watch } from "vue";
import { EditorView, basicSetup, rust, oneDark, keymap } from "../libs/codemirror-bundle.js";

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

      const extensions = [
        basicSetup,
        rust(),
        themeExtension,
        keymap.of([
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
      if (isDark) {
        extensions.push(oneDark);
      }
      editor = new EditorView({
        doc,
        extensions,
        parent: editorEl.value
      });
    }

    async function loadData() {
      const res = await fetch("./js/chapters.json");
      if (!res.ok) {
        throw new Error(`无法加载章节数据: HTTP ${res.status}`);
      }
      modules.value = await res.json();
    }

    function loadChapter(globalIdx) {
      currentGlobalIdx.value = globalIdx;
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

    async function runCode() {
      outputText.value = "⏳ 正在编译运行，请稍候...";
      outputClass.value = "text-gray-200";

      const code = editor.state.doc.toString();

      try {
        const controller = new AbortController();
        const timeoutId = setTimeout(() => controller.abort(), 25000);

        const res = await fetch("/evaluate.json", {
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
      const result = typeof data.result === "string" ? data.result : "";
      const error = typeof data.error === "string" ? data.error : "";

      if (error.length > 0) {
        outputText.value = error;
        outputClass.value = "text-red-400";
      } else if (result.length > 0) {
        outputText.value = result;
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
        currentGlobalIdx.value = 0;
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
