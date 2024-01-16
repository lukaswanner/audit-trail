import type { Actor } from "$lib/types/actor/ActorTypes";
import { writable } from "svelte/store";

export const actors = writable<Actor[]>([]);
