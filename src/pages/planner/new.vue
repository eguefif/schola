<template>
  <div>
    <div class="flex justify-between mb-[32px]">
      <div class="text-2xl">{{ $t("plan.newPlan.title") }}</div>
      <RouterLink to="/planner">
        <div class="btn btn-primary">
          <ArrowLeftIcon class="size-[24px]" />
        </div>
      </RouterLink>
    </div>

    <form class="flex flex-col gap-[32px]" @submit="submitPlan">
      <label class="input w-[512px]">
        <span class="label">{{ $t("plan.newPlan.name") }}</span>
        <input
          id="namePlan"
          type="text"
          :placeholder="$t('plan.newPlan.namePlaceHolder')"
          class="input input-xs sm:input-sm md:input-md lg:input-md xl:input-md"
          v-model="planName"
        />
      </label>
      <label class="select">
        <span class="label">{{ $t("plan.newPlan.grade") }}</span>
        <select class="input" v-model="planGrade">
          <option v-for="grade in grades">{{ grade }}</option>
        </select>
      </label>

      <button type="submit" class="btn btn-primary w-[192px] p-[8px]">
        {{ $t("plan.newPlan.submit") }}
      </button>
    </form>
  </div>
</template>
<script setup lang="ts">
import { ArrowLeftIcon } from "@heroicons/vue/24/solid";
import { ref } from "vue";

const grades = [
  "Petite section",
  "Moyenne section",
  "Grande section",
  "CP",
  "CE1",
  "CE2",
  "CM1",
  "CM2",
];

const planName = ref<string>("");
const planGrade = ref<string>("Petite section");

const submitPlan = (e: Event) => {
  e.preventDefault();
  console.log(planName);
  console.log(planGrade);
};
</script>
