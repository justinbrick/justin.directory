<script setup lang="ts">
import { NuxtPage, NuxtRouteAnnouncer } from '#components';
import { generateCodeVerifier, OAuth2Client } from '@badgateway/oauth2-client';
import NavBar from './components/NavBar.vue';
import { onMounted, provide, ref } from 'vue';

const oauth2 = new OAuth2Client({
  server: 'https://login.microsoftonline.com/',
  clientId: '11ec9395-8c5d-4ac7-9bc2-f4505e7053cf',
  authorizationEndpoint: '/organizations/oauth2/v2.0/authorize',
  tokenEndpoint: '/organizations/oauth2/v2.0/token',
});

// we generate new code verifier per session for security reasons
const codeVerifier = ref<string>();
onMounted(async () => {
  codeVerifier.value = window.sessionStorage.getItem("codeVerifier") ?? await generateCodeVerifier();
  window.sessionStorage.setItem("codeVerifier", codeVerifier.value);
})

provide('oauth2', oauth2);
provide('codeVerifier', codeVerifier);
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
