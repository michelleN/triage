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
    methods: {
        
        async fetchData() { 
            let res = await fetch('http://localhost:3000/schedule/july')
            return await res.json()
        },
    },
    async mounted() {
        /*
        {
            date: number
            isEmpty: bool
            triageMembers: string
        }
        */
        let week = [];
        let totalDays = this.daysInMonth[this.monthNames[this.currentMonth]]
       
        // fill in empty cells for first week
        for (let i = 0; i < this.firstDay; i++) {
            week.push({
                isEmpty: true
            });
        } 

        let triageData = await this.fetchData()
       
        for (let i = 1; i <= totalDays; i++) {
            week.push({
                isEmpty: false,
                date: i,
                triageMembers: triageData[i]

            });

            console.log(week)

            if (week.length === 7 || i === totalDays) {
                this.weeks.push(week);
                week = [];
            }
        }
    },
}
</script>
<template> 
<h2> {{ monthNames[currentMonth] }} {{ currentYear }}</h2>

<table class="calender">
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
        <td class="day" v-for="day in week">
            <div v-if="day.isEmpty">

            </div>
            <div v-else>
                <div class="date">{{  day.date}}</div>
                <div class="data">{{  day.triageMembers}}</div>
            </div>
        </td>
    </tr>
    
</tbody>
</table>
</template>

<style scoped>

.day {
    border: 1px solid green;
    padding: 1rem;
}
.data {
    display: flex;
    align-items: flex-end;
    min-height: 50px;
}
</style>