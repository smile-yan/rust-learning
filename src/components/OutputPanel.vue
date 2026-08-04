<script setup lang="ts">
defineProps<{
  outputText: string;
  outputClass: string;
  elapsedTime: number | null;
}>();

const emit = defineEmits<{
  clear: [];
}>();
</script>

<template>
  <div class="h-56 border-t flex flex-col flex-shrink-0">
    <div class="h-8 px-4 flex items-center justify-between text-xs border-b">
      <span>输出面板</span>
      <div class="flex items-center gap-3">
        <span v-if="elapsedTime !== null" class="opacity-80">耗时 {{ elapsedTime.toFixed(2) }}s</span>
        <button id="clear-output" @click="emit('clear')" class="transition-colors">清空</button>
      </div>
    </div>
    <pre
      id="output"
      :class="['flex-1 p-4 overflow-auto text-sm font-mono', outputClass]"
    >{{ outputText }}</pre>
  </div>
</template>
