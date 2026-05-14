<script setup lang="ts">
// COM (serial) port configuration panel. Lets the operator choose the port,
// baud rate and framing for the RS-232 link to the LCD.
import { reactive, ref, watch } from "vue";
import { api } from "../api";
import type { ComConfig, ComInfo } from "../types";

const props = defineProps<{ info: ComInfo | null }>();
const emit = defineEmits<{ (e: "saved", info: ComInfo): void }>();

const baudRates = [9600, 19200, 38400, 57600, 115200];
const parities = ["none", "odd", "even"] as const;
const flows = ["none", "software", "hardware"] as const;

const draft = reactive<ComConfig>({
  port: "",
  baud_rate: 115200,
  data_bits: 8,
  stop_bits: 1,
  parity: "none",
  flow_control: "none",
  timeout_ms: 500,
});

const saving = ref(false);
const testing = ref(false);
const message = ref<{ kind: "ok" | "err"; text: string } | null>(null);

watch(
  () => props.info,
  (info: ComInfo | null) => {
    if (info) Object.assign(draft, info.config);
  },
  { immediate: true },
);

async function refresh() {
  const info = await api.getCom();
  emit("saved", info);
}

async function save() {
  saving.value = true;
  message.value = null;
  try {
    const info = await api.setCom({ ...draft });
    emit("saved", info);
    message.value = { kind: "ok", text: "Configuration saved." };
  } catch (e) {
    message.value = { kind: "err", text: (e as Error).message };
  } finally {
    saving.value = false;
  }
}

async function test() {
  testing.value = true;
  message.value = null;
  try {
    const r = await api.testCom();
    message.value = r.ok
      ? { kind: "ok", text: "Port opened successfully." }
      : { kind: "err", text: r.error ?? "Failed to open port." };
  } catch (e) {
    message.value = { kind: "err", text: (e as Error).message };
  } finally {
    testing.value = false;
  }
}
</script>

<template>
  <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
    <label class="flex flex-col gap-1 text-sm">
      <span class="text-white/70">COM Port</span>
      <div class="flex gap-2">
        <select v-model="draft.port" class="field flex-1">
          <option value="" disabled>Select a port…</option>
          <option v-for="p in props.info?.available_ports ?? []" :key="p" :value="p">
            {{ p }}
          </option>
          <option v-if="draft.port && !(props.info?.available_ports ?? []).includes(draft.port)" :value="draft.port">
            {{ draft.port }} (saved)
          </option>
        </select>
        <button type="button" class="btn-secondary" @click="refresh" title="Rescan ports">↻</button>
      </div>
    </label>

    <label class="flex flex-col gap-1 text-sm">
      <span class="text-white/70">Baud Rate</span>
      <select v-model.number="draft.baud_rate" class="field">
        <option v-for="b in baudRates" :key="b" :value="b">{{ b }}</option>
      </select>
    </label>

    <label class="flex flex-col gap-1 text-sm">
      <span class="text-white/70">Data Bits</span>
      <select v-model.number="draft.data_bits" class="field">
        <option :value="7">7</option>
        <option :value="8">8</option>
      </select>
    </label>

    <label class="flex flex-col gap-1 text-sm">
      <span class="text-white/70">Stop Bits</span>
      <select v-model.number="draft.stop_bits" class="field">
        <option :value="1">1</option>
        <option :value="2">2</option>
      </select>
    </label>

    <label class="flex flex-col gap-1 text-sm">
      <span class="text-white/70">Parity</span>
      <select v-model="draft.parity" class="field">
        <option v-for="p in parities" :key="p" :value="p">{{ p }}</option>
      </select>
    </label>

    <label class="flex flex-col gap-1 text-sm">
      <span class="text-white/70">Flow Control</span>
      <select v-model="draft.flow_control" class="field">
        <option v-for="f in flows" :key="f" :value="f">{{ f }}</option>
      </select>
    </label>

    <label class="flex flex-col gap-1 text-sm sm:col-span-2">
      <span class="text-white/70">Read Timeout (ms)</span>
      <input v-model.number="draft.timeout_ms" type="number" min="50" step="50" class="field" />
    </label>

    <div class="sm:col-span-2 flex flex-wrap gap-2 items-center">
      <button class="btn-primary" :disabled="saving || !draft.port" @click="save">
        {{ saving ? "Saving…" : "Save" }}
      </button>
      <button class="btn-secondary" :disabled="testing || !draft.port" @click="test">
        {{ testing ? "Testing…" : "Test Connection" }}
      </button>
      <span
        v-if="message"
        class="text-xs px-2 py-1 rounded"
        :class="message.kind === 'ok' ? 'bg-emerald-500/20 text-emerald-300' : 'bg-red-500/20 text-red-300'"
      >
        {{ message.text }}
      </span>
    </div>
  </div>
</template>

<style scoped>
.field {
  @apply bg-white/5 border border-surface-stroke rounded-md px-3 py-2 text-white/90 outline-none focus:border-accent;
}
.btn-primary {
  @apply px-4 py-2 rounded-fluent bg-accent hover:bg-accent-hover active:bg-accent-pressed text-white font-medium disabled:opacity-50 disabled:cursor-not-allowed;
}
.btn-secondary {
  @apply px-4 py-2 rounded-fluent bg-white/10 hover:bg-white/15 text-white/90 disabled:opacity-50 disabled:cursor-not-allowed;
}
</style>
