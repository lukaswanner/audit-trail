import type { Apikey } from "$lib/types/apikey/ApikeyTypes";
import { writable } from "svelte/store";

export const apikeys = writable<Apikey[]>([]);
