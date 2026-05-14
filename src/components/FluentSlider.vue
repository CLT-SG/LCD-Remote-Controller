<script setup lang="ts">
// Windows 11 Fluent-style slider with a tooltip flyout that appears while
// dragging. Emits `change` only when the user releases the pointer to avoid
// flooding the device with serial commands while sliding.
import { computed, ref } from "vue";

const props = defineProps<{
  modelValue: number;
  min?: number;
  max?: number;
  step?: number;
  disabled?: boolean;
  label: string;
  unit?: string;
  icon?: any;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", v: number): void;
  (e: "change", v: number): void;
}>();

const min = computed(() => props.min ?? 0);
const max = computed(() => props.max ?? 100);
const step = computed(() => props.step ?? 1);

const dragging = ref(false);
const local = computed({
  get: () => props.modelValue,
  set: (v: number) => emit("update:modelValue", v),
});

const percent = computed(() => {
  const range = max.value - min.value;
  if (range <= 0) return 0;
  return ((local.value - min.value) / range) * 100;
});

function onInput(e: Event) {
  const v = Number((e.target as HTMLInputElement).value);
  local.value = v;
}
function onChange() {
  dragging.value = false;
  emit("change", local.value);
}
</script>

<template>
  <div class="flex flex-col gap-3 w-full">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2 text-sm text-white/80">
        <component :is="icon" v-if="icon" class="w-4 h-4" />
        <span>{{ label }}</span>
      </div>
      <div class="text-sm tabular-nums text-white/90">
        {{ Math.round(local) }}<span v-if="unit" class="text-white/50">{{ unit }}</span>
      </div>
    </div>

    <div class="relative h-6 flex items-center">
      <!-- Track -->
      <div class="absolute inset-x-0 h-1.5 rounded-full bg-white/10" />
      <!-- Filled -->
      <div
        class="absolute h-1.5 rounded-full bg-accent"
        :style="{ width: percent + '%' }"
      />
      <!-- Native input for accessibility / keyboard -->
      <input
        class="fluent-range relative z-10"
        type="range"
        :min="min"
        :max="max"
        :step="step"
        :value="local"
        :disabled="disabled"
        @pointerdown="dragging = true"
        @input="onInput"
        @change="onChange"
        @pointerup="onChange"
        @keyup.enter="onChange"
        :aria-label="label"
      />
      <!-- Drag flyout tooltip -->
      <transition name="fade">
        <div
          v-if="dragging"
          class="pointer-events-none absolute -top-9 -translate-x-1/2 px-2 py-1 rounded-md bg-surface-card border border-surface-stroke shadow-flyout text-xs tabular-nums"
          :style="{ left: percent + '%' }"
        >
          {{ Math.round(local) }}{{ unit ?? "" }}
        </div>
      </transition>
    </div>
  </div>
</template>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 120ms ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
