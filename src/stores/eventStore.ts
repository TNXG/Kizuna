import { defineStore } from "pinia";

export const useEventStore = defineStore("event", {
	state: () => ({
		eventData: {} as ReturnData,

	}),
	actions: {
		setEventData(data: ReturnData) {
			this.eventData = data;
		},
	},
});
