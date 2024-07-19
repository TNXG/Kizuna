// src/stores/eventStore.js
import { defineStore } from 'pinia';

export const useEventStore = defineStore('event', {
  state: () => ({
    eventData: null,
  }),
  actions: {
    setEventData(data) {
      this.eventData = data;
    },
  },
});
