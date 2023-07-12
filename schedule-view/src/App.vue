<script setup>
import Calendar from './components/Calendar.vue'
import CalenarControls from './components/CalenarControls.vue'
</script>

<template>
  <div class="main">
    <CalenarControls />
    <Calendar />
  </div>
</template>

<style lang="scss">
body {
  width: 100%;
}

.main {
  width: 100%;
  overflow: hidden;
  min-height: 100vh;
  padding: 8rem 2rem 0 2rem;
  display: flex;
  flex-direction: column;
  align-items: center;
}
</style>
