<script setup lang="ts">
import { Trash2 } from "lucide-vue-next";
import type { SerialLogEntry } from "../types";

defineProps<{
  logs: SerialLogEntry[];
}>();

const emit = defineEmits<{
  (e: "clear"): void;
}>();
</script>

<template>
  <div class="flex flex-col gap-3">
    <div class="flex items-center justify-between">
      <div class="flex gap-3 text-xs">
        <span class="flex items-center gap-1.5 text-white/60">
          <span class="w-2 h-2 rounded-full bg-sky-400" />
          TX
        </span>
        <span class="flex items-center gap-1.5 text-white/60">
          <span class="w-2 h-2 rounded-full bg-emerald-400" />
          RX
        </span>
      </div>
      <button
        type="button"
        class="px-2.5 py-1.5 rounded-fluent text-xs bg-white/5 hover:bg-white/10 border border-white/10 flex items-center gap-1.5 text-white/70 disabled:opacity-40"
        :disabled="logs.length === 0"
        @click="emit('clear')"
      >
        <Trash2 class="w-3.5 h-3.5" />
        Clear
      </button>
    </div>

    <div
      class="h-64 overflow-y-auto rounded-fluent bg-black/20 border border-white/5 p-3 space-y-2 font-mono text-xs"
    >
      <div
        v-for="(entry, index) in logs"
        :key="index"
        class="flex gap-2 items-start"
      >
        <span class="text-white/40 shrink-0 w-16">{{ entry.timestamp }}</span>
        <span
          class="shrink-0 w-8 text-center font-bold rounded px-1"
          :class="
            entry.direction === 'TX'
              ? 'bg-sky-500/20 text-sky-300'
              : 'bg-emerald-500/20 text-emerald-300'
          "
        >
          {{ entry.direction }}
        </span>
        <div class="flex-1 min-w-0 space-y-0.5">
          <p class="text-white/80 break-all">{{ entry.hex }}</p>
          <p v-if="entry.summary" class="text-white/50">{{ entry.summary }}</p>
        </div>
      </div>

      <p v-if="logs.length === 0" class="text-white/30 text-center py-8">
        No traffic yet. Commands and replies will appear here once the COM port
        is configured and the display is polled.
      </p>
    </div>
  </div>
</template>
