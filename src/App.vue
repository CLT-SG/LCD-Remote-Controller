<script setup lang="ts">
// Top-level dashboard view. Polls device status periodically and dispatches
// user actions to the embedded REST API.
import { onMounted, onUnmounted, ref } from "vue";
import {
  Volume2,
  VolumeX,
  Sun,
  Contrast,
  Power,
  Plug,
  RefreshCw,
  Smartphone,
} from "lucide-vue-next";

import { api } from "./api";
import type { ComInfo, DeviceStatus, InputSource, ServerInfo } from "./types";
import DashboardCard from "./components/DashboardCard.vue";
import FluentSlider from "./components/FluentSlider.vue";
import PowerToggle from "./components/PowerToggle.vue";
import InputSourceSelect from "./components/InputSourceSelect.vue";
import ComConfigPanel from "./components/ComConfigPanel.vue";

const status = ref<DeviceStatus | null>(null);
const com = ref<ComInfo | null>(null);
const info = ref<ServerInfo | null>(null);
const errorMessage = ref<string | null>(null);
const showSettings = ref(false);
let pollHandle: number | undefined;

async function loadStatus() {
  try {
    status.value = await api.status();
    errorMessage.value = status.value.last_error ?? null;
  } catch (e) {
    errorMessage.value = (e as Error).message;
  }
}

async function loadCom() {
  try {
    com.value = await api.getCom();
  } catch (e) {
    errorMessage.value = (e as Error).message;
  }
}

async function loadInfo() {
  try {
    info.value = await api.serverInfo();
  } catch {
    /* non-fatal */
  }
}

async function withCall(fn: () => Promise<DeviceStatus>) {
  try {
    const r = await fn();
    status.value = r;
    errorMessage.value = r.last_error ?? null;
  } catch (e) {
    errorMessage.value = (e as Error).message;
  }
}

const onPower = (v: boolean) => withCall(() => api.setPower(v));
const onVolume = (v: number) => withCall(() => api.setVolume(v));
const onBrightness = (v: number) => withCall(() => api.setBrightness(v));
const onContrast = (v: number) => withCall(() => api.setContrast(v));
const onMute = (m: boolean) => withCall(() => api.setMute(m));
const onInput = (s: InputSource) => withCall(() => api.setInput(s));

onMounted(async () => {
  await Promise.all([loadStatus(), loadCom(), loadInfo()]);
  pollHandle = window.setInterval(loadStatus, 5000);
});
onUnmounted(() => {
  if (pollHandle) clearInterval(pollHandle);
});
</script>

<template>
  <div class="min-h-full text-white">
    <header
      class="sticky top-0 z-20 backdrop-blur bg-black/30 border-b border-white/5"
    >
      <div class="max-w-5xl mx-auto px-4 sm:px-6 py-3 flex items-center gap-3">
        <div class="w-8 h-8 rounded-lg bg-accent/20 grid place-items-center">
          <Power class="w-4 h-4 text-accent" />
        </div>
        <div class="flex-1 min-w-0">
          <h1 class="text-base sm:text-lg font-semibold leading-tight truncate">
            LCD Remote Controller
          </h1>
          <p class="text-xs text-white/50 truncate">
            BSD-E6 series · RS-232
            <span v-if="status">
              ·
              <span :class="status.connected ? 'text-emerald-400' : 'text-red-400'">
                {{ status.connected ? "Connected" : "Disconnected" }}
              </span>
            </span>
          </p>
        </div>
        <button
          type="button"
          class="px-3 py-1.5 rounded-fluent text-sm bg-white/5 hover:bg-white/10 border border-white/10"
          @click="showSettings = !showSettings"
        >
          <Plug class="inline w-4 h-4 mr-1 -mt-0.5" />
          {{ showSettings ? "Hide" : "COM" }}
        </button>
        <button
          type="button"
          class="px-2.5 py-1.5 rounded-fluent text-sm bg-white/5 hover:bg-white/10 border border-white/10"
          @click="loadStatus"
          title="Refresh status"
        >
          <RefreshCw class="w-4 h-4" />
        </button>
      </div>
    </header>

    <main class="max-w-5xl mx-auto px-4 sm:px-6 py-6 space-y-4">
      <div
        v-if="errorMessage"
        class="rounded-fluent bg-red-500/10 border border-red-500/30 text-red-200 px-4 py-2 text-sm"
      >
        {{ errorMessage }}
      </div>

      <div v-if="showSettings">
        <DashboardCard
          title="Communication Configuration"
          subtitle="Configure the RS-232 link to the display."
        >
          <ComConfigPanel :info="com" @saved="(v: ComInfo) => (com = v)" />
          <div
            v-if="info"
            class="mt-4 pt-4 border-t border-white/10 text-xs text-white/60 flex items-start gap-2"
          >
            <Smartphone class="w-4 h-4 mt-0.5" />
            <div>
              <div class="font-medium text-white/80 mb-1">Mobile Access</div>
              <div>
                On a phone connected to the same network, open one of:
              </div>
              <ul class="mt-1 space-y-0.5">
                <li v-for="addr in info.addresses" :key="addr">
                  <code class="text-accent">http://{{ addr }}:{{ info.port }}</code>
                </li>
              </ul>
            </div>
          </div>
        </DashboardCard>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
        <DashboardCard title="Power" subtitle="Turn the display on or off.">
          <PowerToggle
            :model-value="status?.power ?? false"
            :disabled="!status?.connected"
            description="Sends command 0x40 to the display."
            @change="onPower"
          />
        </DashboardCard>

        <DashboardCard title="Input Source" subtitle="Switch the active video input.">
          <InputSourceSelect
            :model-value="status?.input ?? null"
            :disabled="!status?.connected"
            @change="onInput"
          />
        </DashboardCard>

        <DashboardCard title="Volume" subtitle="0–100, with mute toggle.">
          <div class="space-y-4">
            <FluentSlider
              :model-value="status?.volume ?? 0"
              :icon="status?.muted ? VolumeX : Volume2"
              label="Volume"
              unit="%"
              :disabled="!status?.connected || (status?.muted ?? false)"
              @change="onVolume"
            />
            <div class="flex items-center justify-between text-sm pt-2 border-t border-white/5">
              <span class="text-white/70">Mute</span>
              <PowerToggle
                :model-value="status?.muted ?? false"
                :disabled="!status?.connected"
                label=""
                @change="onMute"
              />
            </div>
          </div>
        </DashboardCard>

        <DashboardCard title="Brightness" subtitle="0–100">
          <FluentSlider
            :model-value="status?.brightness ?? 0"
            :icon="Sun"
            label="Brightness"
            unit="%"
            :disabled="!status?.connected"
            @change="onBrightness"
          />
        </DashboardCard>

        <DashboardCard title="Contrast" subtitle="0–100" class="lg:col-span-2">
          <FluentSlider
            :model-value="status?.contrast ?? 0"
            :icon="Contrast"
            label="Contrast"
            unit="%"
            :disabled="!status?.connected"
            @change="onContrast"
          />
        </DashboardCard>
      </div>

      <footer class="text-center text-xs text-white/40 pt-4">
        v{{ info?.version ?? "0.1.0" }} · BSD-E6 LCD Remote Controller
      </footer>
    </main>
  </div>
</template>
