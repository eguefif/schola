<template>
  <div class="fab fab-flower">
    <div tabindex="0" role="button" class="btn btn-lg btn-circle btn-success">M</div>
    <button class="fab-main-action btn btn-circle btn-lg">M</button>
    <button class="btn btn-lg btn-circle">
      <RouterLink to="/"><HomeIcon class="size-8" /></RouterLink>
    </button>
    <button class="btn btn-lg btn-circle">
      <RouterLink to="/planner"><TableCellsIcon class="size-8" /></RouterLink>
    </button>
  </div>
</template>

<script setup lang="ts">
import { HomeIcon } from "@heroicons/vue/24/solid";
import { TableCellsIcon } from "@heroicons/vue/24/solid";
</script>
