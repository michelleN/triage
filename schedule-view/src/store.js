import { createStore } from 'vuex'

const store = createStore({
    state() {
        return {
            currentMonth: new Date().getMonth(),
            monthNames: [
                "January", "February", "March", "April", "May", "June",
                "July", "August", "September", "October", "November", "December"
            ],
            currentYear: new Date().getFullYear(),
            selectedMonth: new Date().getMonth(),
            selectedYear: new Date().getFullYear(),
            currentTriageData: {}
        }
    },
    getters: {
        selectedMonth: (state) => { return state.selectedMonth },
        selectedYear: (state) => { return state.selectedYear }
    },
    mutations: { // synchronous
    },
    actions: {
        async getCurrentTriageSchedule(context) {
            let res = await fetch('http://localhost:3000/schedule/' + context.state.monthNames[context.state.selectedMonth])
            if (res.ok) {
                let data = await res.json()
                context.state.currentTriageData = data
            } else {
                console.log(await res.text())
            }
        
        }
    }
})

export { store }