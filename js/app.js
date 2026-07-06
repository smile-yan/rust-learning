import { EditorView, basicSetup } from "https://esm.sh/codemirror@6.0.1";
import { rust } from "https://esm.sh/@codemirror/lang-rust@6.0.1";
import { oneDark } from "https://esm.sh/@codemirror/theme-one-dark@6.1.2";
import { keymap } from "https://esm.sh/@codemirror/view@6.22.0";

let modules = [];
let editor;
let currentGlobalIdx = 0;
const allChapters = [];
const chapterIndexMap = new Map();

const THEME_KEY = "rust-learning-theme";

function getPreferredTheme() {
  const saved = localStorage.getItem(THEME_KEY);
  if (saved === "light" || saved === "dark") {
    return saved;
  }
  return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
}

function applyTheme(theme) {
  document.documentElement.setAttribute("data-theme", theme);
  const sun = document.getElementById("theme-icon-sun");
  const moon = document.getElementById("theme-icon-moon");
  if (sun && moon) {
    sun.classList.toggle("hidden", theme !== "dark");
    moon.classList.toggle("hidden", theme === "dark");
  }
  updateEditorTheme(theme);
}

function toggleTheme() {
  const current = document.documentElement.getAttribute("data-theme") || "light";
  const next = current === "light" ? "dark" : "light";
  localStorage.setItem(THEME_KEY, next);
  applyTheme(next);
}

function updateEditorTheme(theme) {
  if (!editor) return;
  const isDark = theme === "dark";
  const currentDoc = editor.state.doc.toString();
  editor.destroy();
  initEditorWithDoc(currentDoc, isDark);
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
    parent: document.getElementById("editor")
  });
}

async function loadData() {
  const res = await fetch("./js/chapters.json");
  if (!res.ok) {
    throw new Error(`无法加载章节数据: HTTP ${res.status}`);
  }
  modules = await res.json();

  modules.forEach((mod, moduleIdx) => {
    mod.chapters.forEach((ch, chapterIdxInModule) => {
      const globalIdx = allChapters.length;
      allChapters.push({
        ...ch,
        moduleName: mod.name,
        moduleIdx,
        chapterIdxInModule,
        globalIdx
      });
      chapterIndexMap.set(`${moduleIdx}-${chapterIdxInModule}`, globalIdx);
    });
  });
}

async function init() {
  applyTheme(getPreferredTheme());

  try {
    await loadData();
    renderChapterList();
    initEditor();
    loadChapter(0);
  } catch (err) {
    document.getElementById("theory-content").innerHTML = `
      <div class="p-4 bg-red-900/30 border border-red-700 rounded text-red-100">
        <h2 class="font-bold text-lg mb-2">加载失败</h2>
        <p>${err.message}</p>
      </div>
    `;
  }

  document.getElementById("run-btn").addEventListener("click", runCode);
  document.getElementById("clear-output").addEventListener("click", clearOutput);
  document.getElementById("menu-toggle").addEventListener("click", toggleMenu);
  document.getElementById("sidebar-overlay").addEventListener("click", closeMenu);
  document.getElementById("theme-toggle").addEventListener("click", toggleTheme);
}

function initEditor() {
  const theme = document.documentElement.getAttribute("data-theme") || "dark";
  initEditorWithDoc("", theme === "dark");
}

function renderChapterList() {
  const nav = document.getElementById("chapter-list");
  nav.innerHTML = modules
    .map((mod, moduleIdx) => {
      const buttons = mod.chapters
        .map(
          (ch, chapterIdxInModule) => {
            const globalIdx = chapterIndexMap.get(`${moduleIdx}-${chapterIdxInModule}`);
            return `
            <button
              data-idx="${globalIdx}"
              class="chapter-btn w-full text-left px-3 py-2 rounded-md text-sm transition-colors flex items-center gap-2"
            >
              <span class="chapter-num inline-flex items-center justify-center w-5 h-5 rounded-full text-xs">${chapterIdxInModule + 1}</span>
              <span class="truncate">${ch.title}</span>
            </button>
          `;
          }
        )
        .join("");

      return `
        <div class="mb-4">
          <h3 class="px-3 py-1 text-xs font-semibold text-orange-400 uppercase tracking-wider mb-1">${mod.name}</h3>
          <div class="space-y-1">${buttons}</div>
        </div>
      `;
    })
    .join("");

  nav.addEventListener("click", (e) => {
    const btn = e.target.closest("[data-idx]");
    if (!btn) return;
    loadChapter(parseInt(btn.dataset.idx, 10));
    if (window.innerWidth < 768) closeMenu();
  });
}

function loadChapter(globalIdx) {
  currentGlobalIdx = globalIdx;
  const ch = allChapters[globalIdx];

  document.querySelectorAll(".chapter-btn").forEach((btn) => {
    const active = parseInt(btn.dataset.idx, 10) === globalIdx;
    btn.classList.toggle("active", active);
  });

  const theoryEl = document.getElementById("theory-content");
  theoryEl.innerHTML = marked.parse(ch.theory);

  const hintEl = document.getElementById("chapter-hint");
  if (ch.hint) {
    hintEl.innerHTML = `<strong class="text-blue-300">💡 提示：</strong> ${ch.hint}`;
    hintEl.classList.remove("hidden");
  } else {
    hintEl.classList.add("hidden");
  }

  if (editor) {
    editor.dispatch({
      changes: {
        from: 0,
        to: editor.state.doc.length,
        insert: ch.code
      }
    });
  }

  clearOutput();
  document.getElementById("theory").scrollTop = 0;
}

async function runCode() {
  const outputEl = document.getElementById("output");
  outputEl.textContent = "⏳ 正在编译运行，请稍候...";
  outputEl.classList.remove("text-red-400", "text-green-400", "text-gray-400");
  outputEl.classList.add("text-gray-200");

  const code = editor.state.doc.toString();

  try {
    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), 25000);

    const res = await fetch("https://play.rust-lang.org/evaluate.json", {
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
      outputEl.textContent = "❌ 请求超时，请检查网络或稍后重试。";
    } else {
      outputEl.textContent = `❌ 运行失败: ${err.message}`;
    }
    outputEl.classList.add("text-red-400");
    outputEl.classList.remove("text-gray-200");
  }
}

function renderOutput(data) {
  const outputEl = document.getElementById("output");
  const result = typeof data.result === "string" ? data.result : "";
  const error = typeof data.error === "string" ? data.error : "";

  outputEl.classList.remove("text-red-400", "text-green-400", "text-gray-200", "text-gray-400");

  if (error.length > 0) {
    outputEl.textContent = error;
    outputEl.classList.add("text-red-400");
  } else if (result.length > 0) {
    outputEl.textContent = result;
    outputEl.classList.add("text-green-400");
  } else {
    outputEl.textContent = "（程序没有输出）";
    outputEl.classList.add("text-gray-200");
  }
}

function clearOutput() {
  const outputEl = document.getElementById("output");
  outputEl.textContent = "点击「运行」按钮查看输出结果";
  outputEl.classList.remove("text-red-400", "text-green-400");
  outputEl.classList.add("text-gray-400");
}

function toggleMenu() {
  const sidebar = document.getElementById("sidebar");
  const overlay = document.getElementById("sidebar-overlay");
  const isClosed = sidebar.classList.contains("-translate-x-full");
  if (isClosed) {
    sidebar.classList.remove("-translate-x-full");
    overlay.classList.remove("hidden");
  } else {
    closeMenu();
  }
}

function closeMenu() {
  document.getElementById("sidebar").classList.add("-translate-x-full");
  document.getElementById("sidebar-overlay").classList.add("hidden");
}

init();
