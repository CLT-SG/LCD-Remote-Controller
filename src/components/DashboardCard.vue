<script setup lang="ts">
defineProps<{ title?: string; subtitle?: string }>();
</script>

<template>
  <section
    class="rounded-fluent bg-surface-card/70 backdrop-blur border border-surface-stroke p-5 shadow-flyout"
  >
    <header v-if="title || subtitle" class="mb-4">
      <h3 v-if="title" class="text-base font-semibold text-white/90">{{ title }}</h3>
      <p v-if="subtitle" class="text-xs text-white/50 mt-0.5">{{ subtitle }}</p>
    </header>
    <slot />
  </section>
</template>
