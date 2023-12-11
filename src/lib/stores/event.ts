import type { Event } from "$lib/types/event/EventTypes";
import { writable } from "svelte/store";


export const events = writable<Event[]>([]);
