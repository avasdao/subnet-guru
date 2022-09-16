import { createRouter, createWebHistory } from 'vue-router'

import HomeView from '../views/HomeView'

import AddGuru from '../views/AddGuru'
import MyGurus from '../views/MyGurus'
import ToolboxView from '../views/ToolboxView'

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
    {
        path: '/toolbox',
        component: ToolboxView,
    },
]

const router = createRouter({
    history: createWebHistory(),
    routes,
})

export default router
