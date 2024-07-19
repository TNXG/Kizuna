import { defineStore } from 'pinia';

export const useLogsStore = defineStore('logs', {
    state: () => ({
        logs: []
    }),
    actions: {
        addLog(log) {
            this.logs.push(log);
        },
        setLogs(newLogs) {
            this.logs = newLogs;
        }
    }
});