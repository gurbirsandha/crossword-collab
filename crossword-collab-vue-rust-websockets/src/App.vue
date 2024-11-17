<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import Home from './views/HomeView.vue'
</script>

<template>
    <div>
        <Home></Home>
    </div>
</template>

<style scoped>
.menubar {
  background-color: white;
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 85px;
}
.logout-button {
  margin-left: auto;
}
.logo {
  margin-right: 1rem;
  margin-top: 0.5rem;
}
/* @media (max-width: 1025px) {
  .logo {
    display: none;
  }
} */
</style>