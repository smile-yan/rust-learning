<script setup lang="ts">
import { ref, watch, nextTick } from "vue";
import type { Chapter } from "@/types/chapter";

const props = defineProps<{
  chapter: Chapter | null;
  html: string;
}>();

const contentRef = ref<HTMLElement | null>(null);

watch(
  () => props.html,
  async () => {
    await nextTick();
    if (contentRef.value && window.Prism) {
      contentRef.value
        .querySelectorAll('pre code:not([class*="language-"])')
        .forEach((block) => block.classList.add("language-rust"));
      window.Prism.highlightAllUnder(contentRef.value);
    }
  },
  { immediate: true }
);
</script>

<template>
  <div class="max-w-3xl mx-auto p-6">
    <div v-if="!chapter" class="text-center opacity-60 py-12">选择章节开始学习</div>
    <template v-else>
      <div ref="contentRef" id="theory-content" class="theory-content" v-html="html"></div>

      <div v-if="chapter.hint" id="chapter-hint" class="mt-6 p-4 rounded-lg text-sm">
        <strong class="text-blue-300">💡 提示：</strong>
        <span v-html="chapter.hint"></span>
      </div>

      <div v-if="chapter.exercises?.length" id="chapter-exercises" class="mt-8 p-5 rounded-lg border">
        <h2 class="text-lg font-bold mb-4 flex items-center gap-2">📝 章节练习</h2>
        <div class="space-y-4">
          <div
            v-for="(ex, idx) in chapter.exercises"
            :key="idx"
            class="exercise-card p-4 rounded-lg border"
          >
            <div class="flex items-start justify-between gap-4">
              <div class="flex-1 min-w-0">
                <h3 class="font-semibold text-sm mb-1">{{ idx + 1 }}. {{ ex.title }}</h3>
                <p class="text-sm opacity-90">{{ ex.description }}</p>
              </div>
              <button
                @click="$emit('loadExercise', ex)"
                class="exercise-load-btn flex-shrink-0 px-3 py-1.5 text-xs font-medium rounded transition-colors whitespace-nowrap"
                title="把练习代码加载到右侧编辑器"
              >
                在编辑器中练习
              </button>
            </div>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>
