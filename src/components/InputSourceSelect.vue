<script setup lang="ts">
import type { InputSource } from "../types";

const sources: { id: InputSource; label: string }[] = [
  { id: "vga", label: "VGA" },
  { id: "hdmi", label: "HDMI" },
  { id: "dp", label: "DP" },
];

const props = defineProps<{ modelValue: InputSource | null; disabled?: boolean }>();
const emit = defineEmits<{
  (e: "update:modelValue", v: InputSource): void;
  (e: "change", v: InputSource): void;
}>();

function pick(s: InputSource) {
  if (props.disabled) return;
  emit("update:modelValue", s);
  emit("change", s);
}
</script>

<template>
  <div class="flex flex-wrap gap-2">
    <button
      v-for="s in sources"
      :key="s.id"
      type="button"
      @click="pick(s.id)"
      :disabled="disabled"
      class="px-4 py-2 min-w-[72px] rounded-fluent text-sm font-medium border transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
      :class="modelValue === s.id
        ? 'bg-accent border-accent text-white'
        : 'bg-white/5 border-surface-stroke text-white/80 hover:bg-white/10'"
    >
      {{ s.label }}
    </button>
  </div>
</template>
