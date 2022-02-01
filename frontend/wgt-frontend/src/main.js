import * as Vue from 'vue' // in Vue 3
import axios from 'axios'
import VueAxios from 'vue-axios'
import * as VueRouter from 'vue-router';
import App from './App.vue';

import checkTemplate from './components/checkTemplate.vue';

const router = VueRouter.createRouter({
    history: VueRouter.createWebHistory(),
    routes: [
        {
            path: '/',
            name: 'home',
            component: () => import('./pages/Home.vue')
        },
        {
            path: '/checkin',
            name: 'checkin',
            component: checkTemplate,
            props: {title : 'Library Sign In', endpoint : 'checkin'}
        },
        {
            path: '/checkout',
            name: 'checkout',
            component: checkTemplate,
            props: {title : 'Library Sign Out', endpoint : 'checkout'}
        },
    ]
})

const app = Vue.createApp(App);
app.use(router);
app.use(VueAxios, axios)
app.mount('#app');
