import { createApp } from "vue";
import App from "./App.vue";
import router from './router'
import './assets/css/main.css'
import { createPinia } from 'pinia';
import { Icon } from '@iconify/vue';

const app = createApp(App);

app.use(router);

app.use(createPinia());

app.component('Icon', Icon);

app.mount("#app");
