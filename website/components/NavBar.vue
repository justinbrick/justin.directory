<script setup lang="ts">
</script>

<template>
    <nav>
        <NuxtLink to="/">Home</NuxtLink>
        <NuxtLink to="/contact">Contact</NuxtLink>
    </nav>
</template>

<style scoped>
nav {
    background-color: var(--color-background-mute);
    padding: 1rem;
}

a {
    display: inline;
}

a::after {
    content: '';
    border: 1px solid var(--color-text);
    margin: 0 0.5em;
}

a:last-child::after {
    display: none;
}
</style>