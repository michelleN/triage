<script>
export default{ 
    data() { 
    return { 
        daysInMonth: this.$store.state.daysInMonth,
        monthNames: this.$store.state.monthNames,
        weekDays:this.$store.state.weekDays,
        currentMonth: this.$store.state.currentMonth,
        currentDate: this.$store.state.currentDate,
        currentYear: this.$store.state.currentYear,
        weeks: this.$store.state.weeks,
        firstDay: this.$store.state.firstDay,
        lastDay: this.$store.state.lastDay,
        } 
    },
    mounted() {
        let week = [];
        let totalDays = this.daysInMonth[this.monthNames[this.currentMonth]]
       
        // fill in empty cells for first week
        for (let i = 0; i < this.firstDay; i++) {
            week.push('');
        } 
       
        for (let i = 1; i <= totalDays; i++) {
            week.push(i);

            if (week.length === 7 || i === totalDays) {
                this.weeks.push(week);
                week = [];
            }
        }
    },
}
</script>
<template> 
<li v-for="item in daysInMonth">{{ item }}</li>
<h2> {{ monthNames[currentMonth] }} {{ currentYear }}</h2>

<table>
<thead>
    <tr>
        <th>Sun</th>
        <th>Mon</th>
        <th>Tue</th>
        <th>Wed</th>
        <th>Thu</th>
        <th>Fri</th>
        <th>sat</th>
    </tr>
</thead>
<tbody>
    <tr v-for="week in weeks">
        <td v-for="day in week">{{ day }}</td>
    </tr>
    
</tbody>
</table>
</template>
<style calendar>  </style>