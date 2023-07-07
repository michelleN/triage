<script>
export default {
    data() {
        return {
            daysInMonth: [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
            monthNames: this.$store.state.monthNames,
            weekDays: ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"]
        }
    },
    methods: {
        async fetchData() {

        },
    },
    computed: {
        firstDay() {
            return new Date(this.$store.getters.selectedYear, this.$store.getters.selectedMonth, 1).getDay()
        },
        lasttDay() {
            return new Date(this.$store.getters.selectedYear, this.$store.getters.selectedMonth + 1, 0).getDay()
        },
        fillCalenderDates() {
            let data = []
            let daysInPrevMonth = this.$store.getters.selectedMonth == 0 ? this.daysInMonth[11] : this.daysInMonth[this.$store.getters.selectedMonth - 1]
            for (let i = daysInPrevMonth - this.firstDay + 1; i <= daysInPrevMonth; i++) {
                data.push({ date: i, active: false })
            }
            for (let i = 1; i <= this.daysInMonth[this.$store.getters.selectedMonth]; i++) {
                data.push({ date: i, active: true })
            }
            let i = 1
            while (data.length % 7 != 0) {
                data.push({ date: i, active: false })
                i++
            }
            return data
        },
        createTaskList() {
            let lastPair = ""
            let data = this.$store.state.currentTriageData
            let tasks = []
            let numTasks = 0
            let daysInCurrMonth = this.daysInMonth[this.$store.getters.selectedMonth]
            let currColumn = this.firstDay
            let currRow = 0
            let lastrow = 0
            for (let i = 1; i <= daysInCurrMonth; i++) {
                let currentPair = data[i]
                if (i == 1) {
                    tasks.push({
                        data: currentPair,
                        length: 1,
                        column: currColumn,
                        row: currRow
                    })
                } else if (currRow > lastrow) {
                    tasks.push({
                        data: currentPair,
                        length: 1,
                        column: currColumn,
                        row: currRow
                    })
                    numTasks++
                }
                else if (currentPair == lastPair) {
                    tasks[numTasks].length++
                } else {
                    tasks.push({
                        data: data[i],
                        length: 1,
                        column: currColumn,
                        row: currRow
                    })
                    numTasks++
                }
                lastPair = currentPair
                currColumn++
                lastrow = currRow
                if ((currColumn == 7)) {
                    currColumn = 0
                    currRow++
                }
            }

            return tasks
        }
    },
    mounted() {
        this.$store.dispatch("getCurrentTriageSchedule")
    },
}
</script>
<template>
    <div class="calendar">
        <span v-for="day in weekDays" class="day-name">{{ day }}</span>
        <div v-for="date in fillCalenderDates" class="day" v-bind:class="{ 'day-disabled': !date.active }">{{ date.date }}</div>
        <section v-for="task in createTaskList" class="task task--warning" 
        :style="{'grid-row': task.row + 2 + `/span 1`, 'grid-column': task.column + 1 + `/span ` + task.length}">
        <span>{{task.data}}</span>
    </section>
    </div>
</template>

<style lang="scss" scoped>
.calendar {
    display: grid;
    width: 100%;
    grid-template-columns: repeat(7, minmax(120px, 1fr));
    grid-template-rows: 50px;
    grid-auto-rows: 120px;
    border: 1px solid rgba(166, 168, 179);
    overflow: auto;
    max-width: 1600px;

    .day {
        border-bottom: 1px solid rgba(166, 168, 179, 0.12);
        border-right: 1px solid rgba(166, 168, 179, 0.12);
        text-align: right;
        padding: 14px 20px;
        letter-spacing: 1px;
        font-size: 12px;
        box-sizing: border-box;
        color: #98a0a6;
        position: relative;
        pointer-events: none;
        z-index: 1;

        &:nth-of-type(7n + 7) {
            border-right: 0;
        }

        &:nth-of-type(n + 1):nth-of-type(-n + 7) {
            grid-row: 2;
        }

        &:nth-of-type(n + 8):nth-of-type(-n + 14) {
            grid-row: 3;
        }

        &:nth-of-type(n + 15):nth-of-type(-n + 21) {
            grid-row: 4;
        }

        &:nth-of-type(n + 22):nth-of-type(-n + 28) {
            grid-row: 5;
        }

        &:nth-of-type(n + 29):nth-of-type(-n + 35) {
            grid-row: 6;
        }

        &:nth-of-type(n + 36):nth-of-type(-n + 42) {
            grid-row: 7;
        }

        &:nth-of-type(7n + 1) {
            grid-column: 1/1;
        }

        &:nth-of-type(7n + 2) {
            grid-column: 2/2;
        }

        &:nth-of-type(7n + 3) {
            grid-column: 3/3;
        }

        &:nth-of-type(7n + 4) {
            grid-column: 4/4;
        }

        &:nth-of-type(7n + 5) {
            grid-column: 5/5;
        }

        &:nth-of-type(7n + 6) {
            grid-column: 6/6;
        }

        &:nth-of-type(7n + 7) {
            grid-column: 7/7;
        }

        &-name {
            font-size: 12px;
            text-transform: uppercase;
            color: #99a1a7;
            text-align: center;
            border-bottom: 1px solid rgba(166, 168, 179, 0.12);
            line-height: 50px;
            font-weight: 500;
        }

        &-disabled {
            color: rgba(#98a0a6, 0.6);
            backdrop-filter: brightness(90%);
            // background-image: url("data:image/svg+xml,%3Csvg width='40' height='40' viewBox='0 0 40 40' xmlns='http://www.w3.org/2000/svg'%3E%3Cg fill='%23f9f9fa' fill-opacity='1' fill-rule='evenodd'%3E%3Cpath d='M0 40L40 0H20L0 20M40 40V20L20 40'/%3E%3C/g%3E%3C/svg%3E");
            cursor: not-allowed;
        }
    }
}

.task {
    padding: 8px 12px;
    margin: 10px;
    font-size: 14px;
    align-self: center;
    position: relative;

    span {
        position: sticky;
        left: 1rem;
    }
    
    &:nth-child(even) {
        border-left: 3px solid $seagreen;
        background-color: rgba($seagreen, 0.2);
    }
    &:nth-child(odd) {
        border-left: 3px solid $rust;
        background-color: rgba($rust, 0.2);
    }
}
</style>