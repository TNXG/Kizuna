<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-shell";
import { onMounted, ref } from "vue";
import Avatar from "./Avatar.vue";

const curYear = new Date().getFullYear();
const backend_version = ref("");

onMounted(async () => {
	try {
		const version = await invoke("get_version");
		backend_version.value = version as string;
	}
	catch (error) {
		console.error("Failed to get version:", error);
	}
});

const sections = [
	{
		name: "主页",
		icon: "mingcute:home-2-line",
		path: "/",
	},
	{
		name: "日志",
		icon: "mingcute:align-left-line",
		path: "/log/",
	},
	{
		name: "设置",
		icon: "mingcute:settings-2-line",
		path: "/setting/",
	},
];

const openInBrowser = async (url: string) => {
	await open(url);
};
</script>

<template>
	<div class="drawer lg:drawer-open">
		<input id="my-drawer" type="checkbox" class="drawer-toggle">
		<div class="drawer-content">
			<label for="my-drawer" class="drawer-button lg:hidden">
				<Icon name="solar:sidebar-code-bold" class="w-8 h-8 m-4" />
			</label>
			<slot />
		</div>
		<div class="drawer-side">
			<label for="my-drawer" aria-label="close sidebar" class="drawer-overlay" />
			<aside
				class="bg-gray-100 max-w-60 w-60 h-screen p-4 flex flex-col justify-between border-r border-gray-300 fixed top-0 left-0 z-50 dark:bg-gray-800 dark:border-gray-950 md:flex"
			>
				<div>
					<div class="flex flex-col items-center">
						<div class="text-base font-bold mb-4 flex items-center">
							<Avatar class="w-10 h-10 mr-2" />
							Kizuna
						</div>
						<div class="w-full lg:w-56 max-w-56">
							<ul class="menu rounded-[1rem] text-base bg-base-200 w-full">
								<li v-for="(item, index) in sections" :key="index" class="mb-2">
									<router-link
										:to="item.path" :class="{ active: $route.path === item.path }"
										class="mb-2 flex items-center w-full"
									>
										<Icon :name="item.icon" class="w-6 h-6 mr-2" /> {{ item.name }}
									</router-link>
								</li>
							</ul>
						</div>
					</div>
				</div>
				<div class="text-center mt-4 text-sm lg:text-base">
					<div class="divider mb-2" />
					© {{ curYear }} <a
						class="text-blue-500 hover:text-blue-700 dark:text-blue-300 dark:hover:text-blue-500"
						href="javascript:void(0)" @click="openInBrowser('https://github.com/TNXG/Kizuna')"
					>Kizuna</a>
					{{ backend_version }}
					<br>
					<p class="text-sm text-gray-700 dark:text-gray-300">
						Designed by <a
							href="javascript:void(0)"
							class="text-blue-500 hover:text-blue-700 dark:text-blue-300 dark:hover:text-blue-500"
							@click="openInBrowser('https://github.com/TNXG/tnxg-homepage')"
						>tnxg-homepage</a>
					</p>
				</div>
			</aside>
		</div>
	</div>
</template>
