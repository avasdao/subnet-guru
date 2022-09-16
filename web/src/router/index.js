import { createRouter, createWebHistory } from 'vue-router'

import HomeView from '../views/HomeView'

import AddGuru from '../views/AddGuru'
import MyGurus from '../views/MyGurus'

const routes = [
    {
        path: '/',
        component: HomeView,
    },
    {
        path: '/add',
        component: AddGuru,
    },
    {
        path: '/gurus',
        component: MyGurus,
    },
]

const router = createRouter({
    history: createWebHistory(),
    routes,
})

export default router
