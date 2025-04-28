import Login from '@/views/LoginView.vue'
import Main from '@/views/MainApp.vue'

import { createRouter, createWebHistory } from 'vue-router'
Login
const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      name: 'main',
      component: Main,
    },
    {
      name: 'login',
      component: Login,
    },
  ],
})

export default router
