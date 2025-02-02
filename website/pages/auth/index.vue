<script setup lang="ts">
import { useSeoMeta } from '#imports';
import { useRoute } from '#app';
import { onMounted } from 'vue';
import { inject } from 'vue';
import type { OAuth2Provider } from '~/composables/auth';

useSeoMeta({
    title: 'Authentication',
    description: 'Page for authentication',
    author: 'Justin'
});

const oauth2 = inject<OAuth2Provider>('oauth2')!;
const route = useRoute();


onMounted(async () => {
    if (!route.query.code) {
        return;
    }

    await oauth2.tokenCallback();
})

async function loginAuthorization() {
    await oauth2.login(['openid', 'profile', 'email']);
}
</script>


<template>
    <main>
        <h1>Auth Page</h1>
        <button @click="loginAuthorization">Login</button>
    </main>
</template>