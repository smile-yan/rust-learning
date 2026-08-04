<script setup lang="ts">
import { ref, watch } from "vue";
import { useEditor } from "@/composables/useEditor";

const props = defineProps<{
  code: string;
  isDark: boolean;
}>();

const emit = defineEmits<{
  run: [];
}>();

const editorEl = ref<HTMLElement | null>(null);
const { setCode, getCode, setTheme, requestMeasure } = useEditor(editorEl, {
  runCode: () => emit("run"),
});

watch(() => props.code, setCode, { immediate: true });
watch(() => props.isDark, setTheme);

defineExpose({
  setCode,
  getCode,
  requestMeasure,
});
</script>

<template>
  <section id="editor-panel" class="flex flex-col min-h-0 flex-1">
    <div class="h-11 border-b flex items-center px-4 justify-between flex-shrink-0">
      <div class="flex items-center gap-2 text-sm">
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"
          />
        </svg>
        <span>实践代码</span>
      </div>
      <button
        id="run-btn"
        @click="emit('run')"
        class="flex items-center gap-1.5 px-4 py-1.5 text-white text-sm font-medium rounded transition-colors"
      >
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z"
          />
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
          />
        </svg>
        运行
      </button>
    </div>
    <div id="editor" ref="editorEl" class="h-[45vh] md:h-auto md:flex-1 md:min-h-0"></div>
    <slot />
  </section>
</template>
