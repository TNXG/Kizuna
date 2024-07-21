import { createRouter, createWebHashHistory } from 'vue-router';
import ServerLogs from '@/layouts/ServerLogs.vue';
import Home from '@/layouts/Home.vue';
import Setting from '@/layouts/Setting.vue';

const routes = [
    { path: '/', component: Home },
    { path: '/logs/', component: ServerLogs },
    { path: '/Setting/', component: Setting },
];
const router = createRouter({
    history: createWebHashHistory(),
    routes,
});

export default router;