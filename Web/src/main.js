import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import { dbAuth } from './firebase/config.js'
//global styles css

import './assets/main.css'

let app 

dbAuth.onAuthStateChanged(() => {
    if(!app){
    app = createApp(App).use(router).mount('#app')
    }
})

