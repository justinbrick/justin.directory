<script setup lang="ts">
import { NuxtPage, NuxtRouteAnnouncer } from '#components';
import { generateCodeVerifier, OAuth2Client } from '@badgateway/oauth2-client';
import NavBar from './components/NavBar.vue';
import { onMounted, provide, ref } from 'vue';
import { useOAuth2Provider } from './composables/auth';


const oauth2 = useOAuth2Provider({
  server: 'https://login.microsoftonline.com/',
  clientId: '11ec9395-8c5d-4ac7-9bc2-f4505e7053cf',
  authorizationEndpoint: '/organizations/oauth2/v2.0/authorize',
  tokenEndpoint: '/organizations/oauth2/v2.0/token',
  providerName: 'self_api',
  callbackPath: '/auth',
})

provide('oauth2', oauth2);
</script>

<template>
  <header>
    <NavBar />
  </header>
  <NuxtRouteAnnouncer />
  <main>
    <NuxtPage />
  </main>
</template>


<style lang="css">
@import url('./assets/base.css');
</style>
