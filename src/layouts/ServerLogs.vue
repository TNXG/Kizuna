<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { computed, ref, watchEffect } from "vue";
import { useLogsStore } from "../stores/logsStore";

const logsStore = useLogsStore();
const logs = computed(() => logsStore.logs);

const logContainer = ref<HTMLElement | null>(null);

const scrollToBottom = () => {
	if (logContainer.value) {
		// Type assertion: assert that `logContainer.value` is an HTMLElement
		(logContainer.value as HTMLElement).scrollTop = (logContainer.value as HTMLElement).scrollHeight;
	}
};

watchEffect(() => {
	scrollToBottom();
});

const openLogDirectory = () => {
	invoke("open_log_directory");
};
</script>

<template>
	<div class="p-4">
		<div class="flex justify-between items-center mb-4">
			<h1 class="text-2xl font-bold dark:text-white">
				Logs
			</h1>
			<button class="btn btn-neutral dark:btn-primary dark:text-white" @click="openLogDirectory">
				日志目录
			</button>
		</div>
		<div ref="logContainer" class="bg-gray-100 dark:bg-gray-800 p-4 rounded-lg overflow-y-auto h-80">
			<p v-for="(log, index) in logs" :key="index" class="text-sm text-gray-800 dark:text-gray-200 mb-1">
				{{ log }}
			</p>
		</div>
	</div>
</template>
