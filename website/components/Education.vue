<script setup lang="ts">
const {institution, degree, startDate, endDate} = defineProps<{
    institution: string
    degree: string
    startDate: string
    endDate: string
}>();
</script>

<template>
    <div class="education">
        <h2>{{ institution }}</h2>
        <p>{{ degree }}</p>
        <p>{{ startDate }} - {{ endDate }}</p>
    </div>
</template>


<style scoped lang="css">

.education {
    margin: 1rem;
}

</style>