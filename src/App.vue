<script setup lang="ts"></script>

<template>
  <main>
    <div class="fab fab-flower">
      <div tabindex="0" role="button" class="btn btn-lg btn-circle btn-success">M</div>
      <button class="fab-main-action btn btn-circle btn-lg">M</button>
      <button class="btn btn-lg btn-circle">
        <RouterLink to="/homepage">Homepage</RouterLink>
      </button>
      <button class="btn btn-lg btn-circle">
        <RouterLink to="/programmation">Programmation</RouterLink>
      </button>
    </div>
    <RouterView />
  </main>
</template>

<style scoped></style>
