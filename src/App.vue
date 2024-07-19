<script setup>
import { onMounted, onUnmounted } from 'vue';
import Sidebar from './components/Sidebar.vue';
import { useLogsStore } from '@/stores/logsStore';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';

const logsStore = useLogsStore();
let unlisten;

onMounted(() => {
  invoke('start');
  unlisten = listen('log-event', (event) => {
    logsStore.addLog(event.payload);
    console.log('Received log-event:', event);
  });
});

onUnmounted(() => {
  unlisten.then(fn => fn());
});
</script>

<template>
  <Sidebar>
    <router-view class="ml-0 lg:ml-60" />
  </Sidebar>
</template>
