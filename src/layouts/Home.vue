<template>
    <div class="flex items-center justify-center min-h-screen">
        <div class="p-6 max-w-lg mx-auto bg-white shadow-lg rounded-lg">
            <div v-if="eventData" class="space-y-6">
                <!-- 显示程序图标 -->
                <div class="flex items-center space-x-4">
                    <img :src="`data:image/png;base64,${eventData.icon}`" alt="Program Icon"
                        class="w-16 h-16">
                    <div class="flex-1">
                        <h1 class="text-xl font-semibold text-gray-800">{{ eventData.data.window_name }}</h1>
                        <p class="text-lg font-medium text-gray-600">进程: {{ eventData.data.process }}</p>
                    </div>
                </div>

                <!-- 显示歌曲信息 -->
                <div class="border-t border-gray-200 pt-4">
                    <h2 class="text-xl font-semibold text-gray-800">当前歌曲</h2>
                    <p class="text-lg font-medium text-gray-700">标题: {{ eventData.data.media.title }}</p>
                    <p class="text-md text-gray-600">艺术家: {{ eventData.data.media.artist }}</p>
                    <p class="text-sm text-gray-500">来源应用: {{ eventData.data.media.SourceAppName }}</p>
                </div>

                <!-- 显示时间戳 -->
                <div class="border-t border-gray-200 pt-4">
                    <p class="text-sm text-gray-500">{{ formatTimestamp(eventData.data.timestamp) }}</p>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue';
import { listen } from '@tauri-apps/api/event';

let unlisten;
const eventData = ref(null);

onMounted(async () => {
    unlisten = await listen('home-event', (event) => {
        console.log('接收到 home-event 事件:', event);
        eventData.value = event.payload;
    });
});

onUnmounted(() => {
    if (unlisten) {
        unlisten();
    }
});

function formatTimestamp(timestamp) {
    const date = new Date(timestamp * 1000);
    return date.toLocaleString();
}
</script>