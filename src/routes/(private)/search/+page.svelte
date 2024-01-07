<script lang="ts">
	import Loading from '$lib/layout/loading/Loading.svelte';
	import { onMount } from 'svelte';
	import type { PageData } from './$types';
	import { readEventListWithMatchingTags } from '$lib/api/event';
	import type { Event as EventType } from '$lib/types/event/EventTypes';
	import Event from '$lib/components/event/Event.svelte';

	let loading = true;
	let events: EventType[] = [];
	export let data: PageData;

	onMount(async () => {
		if (data.key && data.value) {
			try {
				const res = await readEventListWithMatchingTags(data.key, data.value);
				events = await res.json();
				console.log(events);
				loading = false;
			} catch (err) {
				console.log(err);
			}
		}
		loading = false;
		console.log(loading, events);
	});
</script>

<div class="flex flex-row items-center border-b border-b-base-content/10 p-4">
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
	<h1 class="mx-auto text-3xl font-bold brightness-150">search</h1>
</div>

{#if loading}
	<div class="relative z-10 h-full w-full">
		<Loading />
	</div>
{/if}

{#if !loading && events.length > 0}
	<div class="flex flex-col gap-4">
		{#each events as event}
			<Event {event} />
		{/each}
	</div>
{/if}

{#if !loading && events.length === 0}
	<div class="flex h-full w-full flex-col items-center justify-center">
		<h1 class="text-3xl font-bold">No events found</h1>
	</div>
{/if}
