import { defineStore } from "pinia";

export const useLogsStore = defineStore("logs", {
	state: () => ({
		logs: [] as string[],
	}),
	actions: {
		addLog(log: string) {
			this.logs.push(log);
		},
		setLogs(newLogs: string[]) {
			this.logs = newLogs;
		},
	},
});
