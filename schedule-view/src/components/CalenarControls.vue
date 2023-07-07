<script>
export default {
    data() {
        return {
            monthNames: [
                "January", "February", "March", "April", "May", "June",
                "July", "August", "September", "October", "November", "December"
            ]
        }
    },

}
</script>

<template>
    <div class="calender-controls">
        <div class="prev">&lt;</div>
        <div class="date">{{ monthNames[this.$store.getters.selectedMonth] }} {{ this.$store.getters.selectedYear }}</div>
        <div class="prev">&gt;</div>

    </div>
</template>

<style lang="scss" scoped>
.calender-controls {
    display: flex;
    max-width: 800px;
    padding: 2rem;
    width: 100%;

    .date {
        flex-grow: 1;
        text-align: center;
    }
}
</style>