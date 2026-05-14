<script setup lang="ts">
// Top-level dashboard view. Polls device status periodically and dispatches
// user actions to the embedded REST API.
import { onMounted, onUnmounted, ref } from "vue";
import {
  Volume2,
  VolumeX,
  Sun,
  Moon,
  Contrast,
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
import CltLogo from "./components/CltLogo.vue";
import { useTheme } from "./composables/useTheme";

const status = ref<DeviceStatus | null>(null);
const com = ref<ComInfo | null>(null);
const info = ref<ServerInfo | null>(null);
const errorMessage = ref<string | null>(null);
const showSettings = ref(false);
const { theme, toggle: toggleTheme } = useTheme();
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
        <CltLogo :size="36" class="shrink-0" />
        <div class="flex-1 min-w-0">
          <h1 class="text-base sm:text-lg font-semibold leading-tight truncate">
            CLT LCD Remote Controller
          </h1>
          <p class="text-xs text-white/50 truncate">
            BPLRT-BSD-E6 series · RS-232
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
          aria-label="Refresh status"
        >
          <RefreshCw class="w-4 h-4" />
        </button>
        <button
          type="button"
          class="px-2.5 py-1.5 rounded-fluent text-sm bg-white/5 hover:bg-white/10 border border-white/10"
          @click="toggleTheme"
          :title="theme === 'dark' ? 'Switch to light theme' : 'Switch to dark theme'"
          :aria-label="theme === 'dark' ? 'Switch to light theme' : 'Switch to dark theme'"
        >
          <Sun v-if="theme === 'dark'" class="w-4 h-4" />
          <Moon v-else class="w-4 h-4" />
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
        </DashboardCard>
      </div>

      <!-- Row 1: Power, Input Source and Volume share a single row on
           medium screens and up. -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
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
      </div>

      <!-- Row 2: Brightness and Contrast side-by-side. -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
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

        <DashboardCard title="Contrast" subtitle="0–100">
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

      <!-- Mobile Access lives below the Contrast container so operators
           can quickly find the URL to open on their phone. -->
      <DashboardCard
        v-if="info"
        title="Mobile Access"
        subtitle="Open the dashboard on a phone connected to the same network."
      >
        <div class="flex items-start gap-3 text-sm text-white/80">
          <Smartphone class="w-5 h-5 mt-0.5 text-accent" />
          <div class="flex-1">
            <p class="text-white/70 mb-2">
              Point any browser on the same Wi-Fi network at one of the
              addresses below:
            </p>
            <ul class="space-y-1">
              <li v-for="addr in info.addresses" :key="addr">
                <code
                  class="text-accent bg-white/5 border border-white/10 rounded px-2 py-1 inline-block"
                >http://{{ addr }}:{{ info.port }}</code>
              </li>
            </ul>
          </div>
        </div>
      </DashboardCard>

      <footer class="text-center text-xs text-white/40 pt-4">
        v{{ info?.version ?? "0.1.0" }} · CLT LCD Remote Controller
      </footer>
    </main>
  </div>
</template>
