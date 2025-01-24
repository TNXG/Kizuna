<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { onMounted, onUnmounted } from "vue";

import Sidebar from "./components/Sidebar.vue";
import { useLogsStore } from "./stores/logsStore";

const logsStore = useLogsStore();
let unlisten: Promise<() => void>;

onMounted(() => {
	invoke("start");
	unlisten = listen("log-event", (event) => {
		logsStore.addLog(event.payload as string);
	});
});

onUnmounted(() => {
	unlisten.then(fn => fn());
});
</script>

<template>
	<Sidebar>
		<NuxtPage class="ml-0 lg:ml-60" />
	</Sidebar>
</template>
