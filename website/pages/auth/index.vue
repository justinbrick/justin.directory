<script setup lang="ts">
import { navigateTo, useRoute } from '#app';
import { OAuth2Client } from '@badgateway/oauth2-client';
import { inject, onMounted, ref, watch, type Ref } from 'vue';

const oauth2 = inject<OAuth2Client>('oauth2')!;
const codeVerifier = inject<Ref<string | undefined>>('codeVerifier')!;
const route = useRoute();
const token = ref<string>('');

watch([codeVerifier, route.query], async ([verifier, query]) => {
    if (!verifier || !query.code) {
        return;
    }

    try {
        const tokenResponse = await oauth2.authorizationCode.getTokenFromCodeRedirect(
            window.location.href,
            {
                codeVerifier: verifier,
                redirectUri: window.location.origin + '/auth',
            }
        )


        token.value = tokenResponse.accessToken;
    } catch (e) {
        console.error(e);
    }

    await navigateTo('/');
})

async function loginAuthorization() {
    window.location.href = await oauth2.authorizationCode.getAuthorizeUri({
        redirectUri: window.location.origin + '/auth',
        codeVerifier: codeVerifier.value,
        scope: ['openid', 'profile', 'email'],
    })
}
</script>


<template>
    <main>
        <h1>Auth Page</h1>
        <p>Code Verifier: {{ codeVerifier }}</p>
        <p>Access Token: {{ token }}</p>
        <button @click="loginAuthorization">Login</button>
    </main>
</template>