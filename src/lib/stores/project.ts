import type { Project } from '$lib/types/project/ProjectTypes';
import { writable } from 'svelte/store';


export const project = writable<Project | null>(undefined);
export const projects = writable<Project[]>([]);
