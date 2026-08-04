import { ref, computed, watch } from "vue";
import type { Manifest, Chapter, ChapterLocation } from "@/types/chapter";

type ChapterItem = Chapter & {
  moduleName: string;
  moduleIdx: number;
  chapterIdxInModule: number;
  globalIdx: number;
};

export function useChapters() {
  const loading = ref(true);
  const loadError = ref("");
  const manifest = ref<Manifest | null>(null);
  const currentGlobalIdx = ref(0);
  const currentHtml = ref("");

  const modules = computed(() => manifest.value?.modules ?? []);

  const allChapters = computed<ChapterItem[]>(() => {
    const list: ChapterItem[] = [];
    modules.value.forEach((mod, moduleIdx) => {
      mod.chapters.forEach((ch, chapterIdxInModule) => {
        list.push({
          ...ch,
          moduleName: mod.name,
          moduleIdx,
          chapterIdxInModule,
          globalIdx: list.length,
        });
      });
    });
    return list;
  });

  const chapterIndexMap = computed(() => {
    const map = new Map<string, number>();
    allChapters.value.forEach((ch) => {
      map.set(`${ch.moduleIdx}-${ch.chapterIdxInModule}`, ch.globalIdx);
    });
    return map;
  });

  const currentChapter = computed(() => allChapters.value[currentGlobalIdx.value]);

  function getGlobalIdx(moduleIdx: number, chapterIdxInModule: number): number | undefined {
    return chapterIndexMap.value.get(`${moduleIdx}-${chapterIdxInModule}`);
  }

  function getLocationFromGlobal(globalIdx: number): ChapterLocation | null {
    const ch = allChapters.value[globalIdx];
    return ch ? { moduleIdx: ch.moduleIdx, chapterIdxInModule: ch.chapterIdxInModule } : null;
  }

  function updateUrlHash(globalIdx: number) {
    const loc = getLocationFromGlobal(globalIdx);
    if (loc) {
      window.location.hash = `#chapter/${loc.moduleIdx}/${loc.chapterIdxInModule}`;
    }
  }

  function parseUrlHash(): number | null {
    const match = window.location.hash.match(/^#chapter\/(\d+)\/(\d+)$/);
    if (!match) return null;
    const moduleIdx = parseInt(match[1], 10);
    const chapterIdxInModule = parseInt(match[2], 10);
    const globalIdx = getGlobalIdx(moduleIdx, chapterIdxInModule);
    if (globalIdx !== undefined && globalIdx >= 0 && globalIdx < allChapters.value.length) {
      return globalIdx;
    }
    return null;
  }

  async function loadManifest() {
    let lastErr: Error | null = null;
    for (let attempt = 0; attempt < 3; attempt++) {
      try {
        const res = await fetch("/chapters/manifest.json");
        if (!res.ok) throw new Error(`无法加载章节数据: HTTP ${res.status}`);
        manifest.value = (await res.json()) as Manifest;
        return;
      } catch (err) {
        lastErr = err instanceof Error ? err : new Error(String(err));
        if (attempt < 2) {
          await new Promise((resolve) => setTimeout(resolve, 700 * (attempt + 1)));
        }
      }
    }
    throw lastErr ?? new Error("未知错误");
  }

  async function loadChapterHtml(ch: Chapter) {
    try {
      const res = await fetch(`/${ch.htmlPath}`);
      if (!res.ok) throw new Error(`无法加载章节内容: HTTP ${res.status}`);
      currentHtml.value = await res.text();
    } catch (err) {
      loadError.value = err instanceof Error ? err.message : String(err);
      currentHtml.value = "";
    }
  }

  function loadChapter(globalIdx: number) {
    currentGlobalIdx.value = globalIdx;
    updateUrlHash(globalIdx);
    loadError.value = "";
    const ch = currentChapter.value;
    if (ch) {
      loadChapterHtml(ch);
    }
  }

  watch(
    currentChapter,
    (ch) => {
      if (ch) loadChapterHtml(ch);
    },
    { immediate: false }
  );

  async function bootstrap() {
    loading.value = true;
    loadError.value = "";
    try {
      await loadManifest();
      const hashIdx = parseUrlHash();
      currentGlobalIdx.value = hashIdx !== null ? hashIdx : 0;
      const ch = currentChapter.value;
      if (ch) await loadChapterHtml(ch);
    } catch (err) {
      loadError.value = err instanceof Error ? err.message : String(err);
    } finally {
      loading.value = false;
    }
  }

  function retry() {
    bootstrap();
  }

  return {
    loading,
    loadError,
    modules,
    allChapters,
    currentGlobalIdx,
    currentChapter,
    currentHtml,
    getGlobalIdx,
    loadChapter,
    retry,
  };
}
