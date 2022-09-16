import { createRouter, createWebHistory } from 'vue-router'

import DashboardView from '../views/DashboardView'

const routes = [
    {
        path: '/',
        component: DashboardView,
    },
]

const router = createRouter({
    history: createWebHistory(),
    routes,
})

export default router
