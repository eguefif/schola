<script setup lang="ts">
import CellMenu from "@components/CellMenu.vue";
</script>

<template>
  <main class="m-[64px]">
    <CellMenu />
    <div class="w-[768px] mx-auto">
      <RouterView />
    </div>
  </main>
</template>

<style scoped></style>
