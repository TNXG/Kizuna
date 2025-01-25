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
	<div class="min-h-screen min-w-screen bg-gray-100 flex items-center justify-center p-6">
		<div class="w-full max-w-2xl bg-white shadow-lg rounded-lg overflow-hidden">
			<div v-if="isLoading" class="p-8 space-y-6">
				<div class="h-8 bg-gray-200 rounded-md w-3/4 animate-pulse" />
				<div class="h-6 bg-gray-200 rounded-md w-1/2 animate-pulse" />
				<div class="h-32 bg-gray-200 rounded-md w-full animate-pulse" />
			</div>
			<div v-else class="divide-y divide-gray-200">
				<!-- Program Info -->
				<div class="p-8 space-y-4">
					<div class="flex items-center space-x-6">
						<img v-if="eventData.icon" :src="`data:image/png;base64,${eventData.icon}`" alt="Program Icon" class="w-16 h-16 rounded-md">
						<div>
							<h1 class="text-3xl font-light text-gray-900 mb-1">
								{{ eventData.data?.window_name }}
							</h1>
							<p class="text-lg text-gray-600 font-light">
								{{ eventData.data?.process }}
							</p>
						</div>
					</div>
				</div>

				<!-- Media Info -->
				<div v-if="eventData.data?.media" class="p-8 space-y-6">
					<div class="flex items-start space-x-6">
						<img
							v-if="eventData.AlbumThumbnail" :src="`data:image/png;base64,${eventData.AlbumThumbnail}`"
							alt="Album Thumbnail" class="w-32 h-32 object-cover rounded-md"
						>
						<div class="flex-1 space-y-3">
							<h2 class="text-2xl font-medium text-gray-900">
								{{ eventData.data.media.title }}
							</h2>
							<p v-if="eventData.data.media.artist" class="text-xl text-gray-700 font-light">
								{{ eventData.data.media.artist }}
							</p>
							<p v-if="eventData.data.media.AlbumTitle" class="text-lg text-gray-600 font-light">
								{{ eventData.data.media.AlbumTitle }}
							</p>
							<p v-if="eventData.data.media.AlbumArtist" class="text-md text-gray-500">
								{{ eventData.data.media.AlbumArtist }}
							</p>
						</div>
					</div>
					<p v-if="eventData.data.media.SourceAppName" class="text-sm text-gray-400 uppercase tracking-wide">
						{{ eventData.data.media.SourceAppName }}
					</p>
				</div>

				<!-- Timestamp -->
				<div class="p-4 bg-gray-50">
					<p class="text-sm text-gray-400">
						更新时间: {{ formatTimestamp(eventData.data?.timestamp) }}
					</p>
				</div>
			</div>
		</div>
	</div>
</template>
