<template>
  <div class="flex flex-col gap-[128px]">
    <div class="flex justify-between">
      <div class="text-2xl">{{ $t("plan.title") }}</div>
      <RouterLink to="/planner/new"
        ><div class="btn btn-primary">{{ $t("plan.newPlanButton") }}</div>
      </RouterLink>
    </div>

    <div>
      {{ $t("plan.noPlan") }}
    </div>
  </div>
</template>

<script setup lang="ts"></script>
