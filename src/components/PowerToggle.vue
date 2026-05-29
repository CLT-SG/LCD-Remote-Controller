<script setup lang="ts">
// Fluent-style toggle switch used for the display power state.
import { computed } from "vue";

const props = defineProps<{
  modelValue: boolean;
  disabled?: boolean;
  label?: string;
  description?: string;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", v: boolean): void;
  (e: "change", v: boolean): void;
}>();

const checked = computed({
  get: () => props.modelValue,
  set: (v: boolean) => emit("update:modelValue", v),
});

function toggle() {
  if (props.disabled) return;

  const next = !checked.value;
  checked.value = next;
  emit("change", next);
}
</script>

<template>
  <div class="flex items-center justify-between gap-4">
    <div class="flex flex-col">
      <span class="text-sm font-medium text-white/90">{{ label ?? "Power" }}</span>
      <span v-if="description" class="text-xs text-white/50">{{ description }}</span>
    </div>
    <button
      type="button"
      role="switch"
      :aria-checked="checked"
      :disabled="disabled"
      @click="toggle"
      class="relative inline-flex h-7 w-12 shrink-0 cursor-pointer rounded-full border-2 transition-colors duration-200 focus:outline-none focus-visible:ring-2 focus-visible:ring-accent focus-visible:ring-offset-2 focus-visible:ring-offset-surface disabled:opacity-50 disabled:cursor-not-allowed"
      :class="checked
        ? 'bg-accent border-accent'
        : 'bg-transparent border-white/40 hover:border-white/70'"
    >
      <span
        class="pointer-events-none inline-block h-4 w-4 rounded-full bg-white shadow ring-0 transition-transform duration-200 my-auto"
        :class="checked ? 'translate-x-6' : 'translate-x-1'"
      />
    </button>
  </div>
</template>
