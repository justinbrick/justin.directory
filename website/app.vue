<script setup lang="ts">
import { NuxtPage, NuxtRouteAnnouncer } from '#components';
import { OAuth2Client } from '@badgateway/oauth2-client';
import NavBar from './components/NavBar.vue';
import { provide } from 'vue';

const oauth2 = new OAuth2Client({
  clientId: '11ec9395-8c5d-4ac7-9bc2-f4505e7053cf',
  authorizationEndpoint: 'https://login.microsoftonline.com/organizations/oauth2/v2.0/authorize',
});

// we provide this system for the whole app
provide('oauth2', oauth2);
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
