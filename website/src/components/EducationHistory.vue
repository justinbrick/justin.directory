<script setup lang="ts">
import education from '@/assets/json/education.json'

function shortDate(date: string) {
    return new Date(date).toLocaleDateString('en', { month: 'short', year: '2-digit' })
}
</script>

<template>
    <div class="education">
        <h2>Education</h2>
        <div>
            <div v-for="school in education" :key="school.school">
                <h3>{{ school.school }}</h3>
                <p>{{ school.degree }}</p>
                <p>{{ shortDate(school.from) }} - {{ shortDate(school.to) }}</p>
            </div>
        </div>
    </div>
</template>

<style scoped>
.education {
    margin-top: 20px;
}
</style>