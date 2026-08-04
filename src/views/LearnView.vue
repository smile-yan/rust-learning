<script setup lang="ts">
import { ref, computed, watch, onMounted } from "vue";
import { useChapters } from "@/composables/useChapters";
import { useTheme } from "@/composables/useTheme";
import { usePlayground } from "@/composables/usePlayground";
import ChapterSidebar from "@/components/ChapterSidebar.vue";
import TheoryPanel from "@/components/TheoryPanel.vue";
import CodeEditor from "@/components/CodeEditor.vue";
import OutputPanel from "@/components/OutputPanel.vue";
import type { Exercise } from "@/types/chapter";

const { theme, toggleTheme } = useTheme();
const { loading, loadError, modules, currentGlobalIdx, currentChapter, currentHtml, loadChapter, retry } =
  useChapters();
const { outputText, outputClass, elapsedTime, runCode, clearOutput } = usePlayground();

const editorRef = ref<InstanceType<typeof CodeEditor> | null>(null);
const menuOpen = ref(false);
const sidebarCollapsed = ref(false);
const editorCollapsed = ref(false);
const theoryEl = ref<HTMLElement | null>(null);
const mainEl = ref<HTMLElement | null>(null);

const isDark = computed(() => theme.value === "dark");

function handleRunCode() {
  const code = editorRef.value?.getCode() ?? "";
  runCode(code);
}

function handleLoadExercise(ex: Exercise) {
  editorRef.value?.setCode(ex.code_template);
  clearOutput();
  if (window.innerWidth < 768) {
    editorRef.value?.$el.scrollIntoView({ behavior: "smooth", block: "start" });
  }
}

function handleLoadChapter(globalIdx: number) {
  loadChapter(globalIdx);
  clearOutput();
  if (theoryEl.value) theoryEl.value.scrollTop = 0;
  if (mainEl.value) mainEl.value.scrollTop = 0;
  if (window.innerWidth < 768) {
    menuOpen.value = false;
  }
}

function toggleMenu() {
  menuOpen.value = !menuOpen.value;
}

function closeMenu() {
  menuOpen.value = false;
}

function toggleSidebar() {
  sidebarCollapsed.value = !sidebarCollapsed.value;
}

function toggleEditor() {
  editorCollapsed.value = !editorCollapsed.value;
  if (!editorCollapsed.value) {
    editorRef.value?.requestMeasure();
  }
}

onMounted(() => {
  document.getElementById("boot-loader")?.remove();
  retry();
});

watch(
  currentChapter,
  (ch) => {
    if (ch) {
      editorRef.value?.setCode(ch.code);
      clearOutput();
    }
  },
  { immediate: false }
);

const appVersion = import.meta.env.VITE_APP_VERSION ?? "dev";
</script>

<template>
  <div id="app" v-cloak class="h-screen flex flex-col overflow-hidden w-full">
    <div
      v-if="loading"
      class="fixed inset-0 z-30 flex flex-col items-center justify-center gap-4"
      style="background: var(--bg-body); color: var(--text-primary)"
    >
      <div class="w-10 h-10 rounded-full border-4 border-orange-500/25 border-t-orange-500 animate-spin"></div>
      <p class="text-sm tracking-wide animate-pulse">正在加载章节内容…</p>
    </div>

    <header class="h-14 border-b flex items-center px-4 justify-between flex-shrink-0">
      <a href="./" class="flex items-center gap-3" title="回到根目录">
        <svg class="w-7 h-7 text-orange-500" viewBox="0 0 32 32" fill="currentColor">
          <path
            d="M16 2C8.268 2 2 8.268 2 16s6.268 14 14 14 14-6.268 14-14S23.732 2 16 2zm0 4a2 2 0 1 1 0 4 2 2 0 0 1 0-4zm-6 8a2 2 0 1 1 0 4 2 2 0 0 1 0-4zm12 0a2 2 0 1 1 0 4 2 2 0 0 1 0-4zm-9.5 8h7a1 1 0 0 1 0 2h-7a1 1 0 0 1 0-2z"
          />
        </svg>
        <div>
          <h1 class="font-bold text-lg leading-tight">Rust 学习之旅</h1>
          <p class="text-xs opacity-70">理论与实践结合的交互式教程</p>
        </div>
      </a>

      <div class="flex items-center gap-2">
        <span class="text-xs opacity-60 hidden sm:inline">v{{ appVersion }}</span>
        <a
          id="github-link"
          href="https://github.com/smile-yan/rust-learning"
          target="_blank"
          rel="noopener noreferrer"
          class="p-2 rounded-md border transition-colors"
          aria-label="GitHub 仓库"
          title="GitHub 仓库"
        >
          <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
            <path
              d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12"
            />
          </svg>
        </a>

        <button
          id="theme-toggle"
          @click="toggleTheme"
          class="p-2 rounded-md border transition-colors"
          aria-label="切换主题"
          title="切换主题"
        >
          <svg
            id="theme-icon-sun"
            :class="['w-5 h-5', theme !== 'dark' ? 'hidden' : 'block']"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"
            />
          </svg>
          <svg
            id="theme-icon-moon"
            :class="['w-5 h-5', theme === 'dark' ? 'hidden' : 'block']"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"
            />
          </svg>
        </button>

        <button
          id="menu-toggle"
          @click="toggleMenu"
          class="md:hidden p-2 rounded transition-colors"
          aria-label="切换菜单"
        >
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M4 6h16M4 12h16M4 18h16"
            />
          </svg>
        </button>
      </div>
    </header>

    <div class="flex-1 flex overflow-hidden">
      <aside
        id="sidebar"
        :class="[
          'fixed inset-y-0 left-0 z-20 w-72 border-r flex-shrink-0 overflow-y-auto pt-14 md:pt-0 md:static transform transition-transform duration-200',
          sidebarCollapsed ? 'md:w-0 md:overflow-hidden md:border-r-0' : 'md:w-1/5',
          menuOpen ? 'translate-x-0' : '-translate-x-full md:translate-x-0',
        ]"
      >
        <ChapterSidebar
          :modules="modules"
          :current-global-idx="currentGlobalIdx"
          @load="handleLoadChapter"
        />
      </aside>

      <button
        id="sidebar-collapse-btn"
        @click="toggleSidebar"
        class="collapse-toggle hidden md:flex w-4 flex-shrink-0 items-center justify-center"
        :title="sidebarCollapsed ? '展开目录' : '收起目录'"
      >
        <span class="collapse-knob">
          <svg
            class="w-3.5 h-3.5 transition-transform duration-200"
            :class="{ 'rotate-180': sidebarCollapsed }"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M15 19l-7-7 7-7" />
          </svg>
        </span>
      </button>

      <div
        id="sidebar-overlay"
        @click="closeMenu"
        :class="['fixed inset-0 z-10 md:hidden', menuOpen ? 'block' : 'hidden']"
      ></div>

      <main
        ref="mainEl"
        class="flex-1 flex flex-col md:flex-row overflow-y-auto md:overflow-hidden min-w-0"
      >
        <section
          id="theory"
          ref="theoryEl"
          class="md:flex-1 md:min-w-0 md:overflow-y-auto border-b md:border-b-0 md:border-r"
        >
          <div v-if="loadError" class="max-w-3xl mx-auto p-6">
            <div class="p-4 bg-red-900/30 border border-red-700 rounded text-red-100">
              <h2 class="font-bold text-lg mb-2">加载失败</h2>
              <p>{{ loadError }}</p>
              <button
                @click="retry"
                class="mt-3 px-4 py-1.5 bg-orange-600 hover:bg-orange-500 text-white text-sm rounded transition-colors"
              >
                重新加载
              </button>
            </div>
          </div>
          <TheoryPanel
            v-else
            :chapter="currentChapter"
            :html="currentHtml"
            @load-exercise="handleLoadExercise"
          />
        </section>

        <CodeEditor
          ref="editorRef"
          :code="currentChapter?.code ?? ''"
          :is-dark="isDark"
          class="md:min-w-0 flex flex-col md:min-h-[50vh]"
          :class="[
            editorCollapsed ? 'md:flex-none md:w-0 md:overflow-hidden' : 'md:flex-1',
          ]"
          @run="handleRunCode"
        >
          <OutputPanel
            :output-text="outputText"
            :output-class="outputClass"
            :elapsed-time="elapsedTime"
            @clear="clearOutput"
          />
        </CodeEditor>

        <button
          id="editor-collapse-btn"
          @click="toggleEditor"
          class="collapse-toggle hidden md:flex w-4 flex-shrink-0 items-center justify-center"
          :title="editorCollapsed ? '展开代码编辑器' : '收起代码编辑器'"
        >
          <span class="collapse-knob">
            <svg
              class="w-3.5 h-3.5 transition-transform duration-200"
              :class="{ 'rotate-180': editorCollapsed }"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M9 5l7 7-7 7" />
            </svg>
          </span>
        </button>
      </main>
    </div>
  </div>
</template>
