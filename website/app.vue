<script setup lang="ts">
import { NuxtPage, NuxtRouteAnnouncer } from '#components';
import NavBar from './components/NavBar.vue';

</script>

<template>
  <header>
    <NavBar />
  </header>
  <NuxtRouteAnnouncer />
  <NuxtPage />

</template>

<style lang="css">
@import url('./assets/base.css');
</style>
