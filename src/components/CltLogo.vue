<script setup lang="ts">
// Brand mark for the CLT LCD Remote Controller. The artwork lives at
// `src/assets/logo.png` (a copy of the canonical PNG also used as the
// desktop application icon under `src-tauri/icons/`). Importing it
// through Vite means the bundler hashes the URL and inlines small
// variants automatically.
import logoUrl from "../assets/logo.png";

defineProps<{ size?: number }>();
</script>

<template>
  <img
    :src="logoUrl"
    :width="size ?? 36"
    :height="size ?? 36"
    alt="CLT logo"
    class="rounded-md select-none"
    draggable="false"
  />
</template>
