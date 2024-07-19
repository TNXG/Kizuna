import { createRouter, createWebHashHistory } from 'vue-router';
import ServerLogs from '@/layouts/ServerLogs.vue';
import Home from '@/layouts/Home.vue';

const routes = [
    { path: '/', component: Home },
    { path: '/logs/', component: ServerLogs },
];
const router = createRouter({
    history: createWebHashHistory(),
    routes,
});

export default router;