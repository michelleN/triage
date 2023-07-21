import { createApp } from 'vue'
import './style.css'
import { createStore } from 'vuex'
import App from './App.vue'


// Create a new store instance.
const store = createStore({
    state () {

      
      return {
        daysInMonth: {January: 31, February: 28, March: 31, April: 30, May: 31, June: 30, July: 31, August: 31, September: 30, October: 31, November: 30, December: 31},
        monthNames: [
          "January", "February", "March", "April", "May", "June",
          "July", "August", "September", "October", "November", "December"
        ],
        weekDays: ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"],
        currentDate: new Date(),
        currentMonth: new Date().getMonth(),
        currentYear: new Date().getFullYear(),
        firstDay: new Date(new Date().getFullYear(), new Date().getMonth(), 1).getDay(),
        lastDay: new Date(new Date().getFullYear(), new Date().getMonth() + 1, 0).getDay(),
        weeks: [],
        
      }
    },
    mutations: { // synchronous
      increment (state) {
        state.count++
      }
    }
  })

let app = createApp(App)
app.use(store)
app.mount('#app')