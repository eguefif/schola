<template>
  <div class="flex flex-col gap-[128px]">
    <div class="flex justify-between">
      <div class="text-2xl">{{ $t("plan.title") }}</div>
      <RouterLink to="/planner/new"
        ><div class="btn btn-primary">{{ $t("plan.newPlanButton") }}</div>
      </RouterLink>
    </div>

    <div v-if="!plans">
      {{ $t("plan.noPlan") }}
    </div>
    <div v-else>
      <ul class="list">
        <li v-for="plan in plans" key="plan.id" class="li-row w-fit shadow-md p-[16px] bg-base-200 rounded-md">
          <div class="flex flex-row gap-8 items-center">
            <div class="text-xl">{{ plan.name }}</div>
            <div>{{ plan.grade }}</div>
            <button class="btn btn-md btn-square btn-ghost"><XCircleIcon class="size-8" /></button>
          </div>
        </li>
      </ul>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { XCircleIcon } from "@heroicons/vue/24/outline";
import { ref } from "vue";

interface YearPlan {
  id: number;
  name: string;
  grade: string;
}

let plans = ref<YearPlan[] | null>(null);

invoke("list_plans_cmd").then((message: YearPlan[]) => (plans.value = message));
</script>
