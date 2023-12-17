import type { Insight } from "$lib/types/insight/Insight";
import { writable } from "svelte/store";


export const insights = writable<Insight[]>([]);
