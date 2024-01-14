<script lang="ts">
	import Loading from '$lib/layout/loading/Loading.svelte';
	import { onMount } from 'svelte';
	import { readEventList, readEventListWithMatchingTags } from '$lib/api/event';
	import type { Event as EventType } from '$lib/types/event/EventTypes';
	import Event from '$lib/components/event/Event.svelte';
	import { project } from '$lib/stores/project';
	import { goto } from '$app/navigation';
	import { fly } from 'svelte/transition';
	import { backOut } from 'svelte/easing';

	let loading = true;
	let searchQuery = new URLSearchParams(window.location.search);
	let events: EventType[] = [];
	let filteredEvents: EventType[] = [];

	async function searchEvents(event: Event) {
		searchQuery.set('title', event.target.value);
		goto(`/search?${searchQuery.toString()}`, { keepFocus: true });
		filterEvents();
	}

	async function filterEvents() {
		const searchTitle = searchQuery.get('title')?.toLowerCase();
		if (!searchTitle) {
			filteredEvents = events;
		}
		if (searchTitle) {
			filteredEvents = events.filter((event) => {
				return event.title.toLowerCase().includes(searchTitle);
			});
		}
	}

	async function readEvents() {
		const res = await readEventList();
		const allEvents = (await res.json()) as EventType[];
		events = allEvents.filter((event) => {
			return event.projectId === $project!.id;
		});
		filteredEvents = events;
	}

	onMount(async () => {
		const key = searchQuery.get('key');
		const value = searchQuery.get('value');
		const searchTitle = searchQuery.get('title');
		if (key && value) {
			try {
				const res = await readEventListWithMatchingTags(key, value);
				events = await res.json();
				filteredEvents = events;
				loading = false;
			} catch (err) {
				console.log(err);
			}
		} else if (searchTitle) {
			try {
				await readEvents();
				filterEvents();
			} catch (err) {
				console.log(err);
			}
		} else {
			try {
				await readEvents();
				filteredEvents = events;
				loading = false;
			} catch (err) {
				console.log(err);
			}
		}
		loading = false;
	});

	$: $project, readEvents();

	console.log($project);
</script>

<div class="flex flex-row items-center gap-4 border-b border-b-base-content/10 p-4">
	<a href="/">
		<svg
			xmlns="http://www.w3.org/2000/svg"
			class="h-4 w-4 text-3xl font-bold brightness-150"
			viewBox="0 0 320 512"
			><!--!Font Awesome Free 6.5.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.--><path
				opacity="1"
				fill="currentColor"
				d="M9.4 233.4c-12.5 12.5-12.5 32.8 0 45.3l192 192c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3L77.3 256 246.6 86.6c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0l-192 192z"
			/></svg
		>
	</a>
	<input
		type="text"
		on:input={searchEvents}
		placeholder={`Search in for event title in ${$project?.title}`}
		class="input input-bordered w-full"
	/>
</div>

{#if loading}
	<div class="relative z-10 h-full w-full">
		<Loading />
	</div>
{/if}

{#if !loading && filteredEvents.length > 0}
	<div class="flex h-full flex-col gap-4 overflow-auto p-4">
		{#each [...filteredEvents] as event, index}
			{#key event.id}
				<div
					in:fly|global={{
						y: 100,
						delay: 100 * index,
						easing: backOut
					}}
				>
					<Event {event} />
				</div>
			{/key}
		{/each}
	</div>
{/if}

{#if !loading && filteredEvents.length === 0}
	<div class="flex h-full w-full flex-col items-center justify-center">
		<h1 class="text-3xl font-bold">No events found</h1>
	</div>
{/if}
