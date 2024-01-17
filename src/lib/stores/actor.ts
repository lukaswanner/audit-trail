import type { Actor } from "$lib/types/actor/ActorTypes";
import { writable } from "svelte/store";

export const actor = writable<Actor | null>(undefined);
export const actors = writable<Actor[]>([]);
