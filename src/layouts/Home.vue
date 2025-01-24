<script setup lang="ts">
import { listen } from "@tauri-apps/api/event";
import { computed, onMounted, onUnmounted, ref } from "vue";
import { useEventStore } from "../stores/eventStore";

const eventStore = useEventStore();
const eventData = computed(() => eventStore.eventData);
const isLoading = ref(true);

onMounted(async () => {
	const unlisten = await listen("home-event", (event) => {
		eventStore.setEventData(event.payload as ReturnData);
		isLoading.value = false;
	});

	onUnmounted(() => {
		if (unlisten) {
			unlisten();
		}
	});
});

function formatTimestamp(timestamp: number) {
	const date = new Date(timestamp * 1000);
	return date.toLocaleString();
}
</script>

<template>
	<div class="flex items-center justify-center min-h-screen min-w-screen">
		<div class="p-6 max-w-lg mx-auto bg-white shadow-lg rounded-lg min-w-[300px]">
			<div v-if="isLoading" class="space-y-6">
				<!-- 骨架屏占位 -->
				<div class="flex items-center space-x-4">
					<div class="skeleton w-16 h-16 rounded-full" />
					<div class="flex-1">
						<div class="skeleton w-3/4 h-6 mb-2" />
						<div class="skeleton w-1/2 h-4" />
					</div>
				</div>
				<div class="border-t border-gray-200 pt-4">
					<div class="skeleton w-1/2 h-4" />
				</div>
			</div>
			<div v-else class="space-y-6">
				<!-- 显示程序图标 -->
				<div class="flex items-center space-x-4">
					<img :src="`data:image/png;base64,${eventData.icon}`" alt="Program Icon" class="w-16 h-16">
					<div class="flex-1">
						<h1 class="text-xl font-semibold text-gray-800">
							{{ eventData.data?.window_name }}
						</h1>
						<p class="text-lg font-medium text-gray-600">
							进程: {{ eventData.data?.process }}
						</p>
					</div>
				</div>

				<!-- 显示歌曲信息 -->
				<div v-if="eventData.data?.media" class="border-t border-gray-200 pt-4">
					<h2 class="text-xl font-semibold text-gray-800">
						当前歌曲
					</h2>
					<p class="text-lg font-medium text-gray-700">
						标题: {{ eventData.data?.media?.title }}
					</p>
					<p class="text-md text-gray-600">
						艺术家: {{ eventData.data?.media?.artist }}
					</p>
					<p class="text-sm text-gray-500">
						来源应用: {{ eventData.data?.media?.SourceAppName }}
					</p>
				</div>

				<!-- 显示时间戳 -->
				<div class="border-t border-gray-200 pt-4">
					<p class="text-sm text-gray-500">
						{{ formatTimestamp(eventData.data?.timestamp) }}
					</p>
				</div>
			</div>
		</div>
	</div>
</template>
