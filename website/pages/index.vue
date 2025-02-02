<script setup lang="ts">
import { useSeoMeta } from 'unhead';


useSeoMeta({
    author: 'Justin',
    description: 'A personal portfolio',
    title: 'Justin\'s Portfolio',
})
</script>

<template>
    <main>
        <div class="primary">
            <h1 class="section-area">Education</h1>
        </div>
        <div class="other">
            <h1 class="section-area">Skills</h1>
        </div>
    </main>
</template>

<style lang="css" scoped>
main {
    display: flex;
    flex-direction: row;
}

@media (max-width: 768px) {
    main {
        flex-direction: column;
    }
}

/* primary emphasis on this part. */
.primary {
    flex-basis: 60%;
}

.other {
    flex-grow: 1;
}

.section-area {
    padding-left: 1rem;
    color: var(--color-primary);
}
</style>