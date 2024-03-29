import type { Channel } from "$lib/types/channel/ChannelTypes";
import { writable } from "svelte/store";

export const channel = writable<Channel | null>(undefined);
export const channels = writable<Channel[]>([]);
