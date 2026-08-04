<script setup lang="ts">
import type { Module } from "@/types/chapter";

const props = defineProps<{
  modules: Module[];
  currentGlobalIdx: number;
}>();

const emit = defineEmits<{
  load: [globalIdx: number];
}>();

function getGlobalIdx(moduleIdx: number, chapterIdxInModule: number): number {
  let idx = 0;
  for (let mi = 0; mi < moduleIdx; mi++) {
    idx += props.modules[mi]?.chapters.length ?? 0;
  }
  return idx + chapterIdxInModule;
}
</script>

<template>
  <nav id="chapter-list" class="p-3">
    <div v-for="(mod, moduleIdx) in modules" :key="moduleIdx" class="mb-4">
      <h3 class="px-3 py-1 text-xs font-semibold text-orange-400 uppercase tracking-wider mb-1">
        {{ mod.name }}
      </h3>
      <div class="space-y-1">
        <button
          v-for="(ch, chapterIdxInModule) in mod.chapters"
          :key="chapterIdxInModule"
          @click="emit('load', getGlobalIdx(moduleIdx, chapterIdxInModule))"
          :class="[
            'chapter-btn w-full text-left px-3 py-2 rounded-md text-sm transition-colors flex items-center gap-2',
            { active: currentGlobalIdx === getGlobalIdx(moduleIdx, chapterIdxInModule) },
          ]"
        >
          <span class="chapter-num inline-flex items-center justify-center w-5 h-5 rounded-full text-xs">
            {{ chapterIdxInModule + 1 }}
          </span>
          <span class="truncate">{{ ch.title }}</span>
        </button>
      </div>
    </div>
  </nav>
</template>
