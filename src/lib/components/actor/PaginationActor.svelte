<script lang="ts">
	import { actor } from "$lib/stores/actor";
	import type { Event } from "$lib/types/event/EventTypes";
	import ActivityEvent from "./ActivityEvent.svelte";

	export let events: Event[];

	let pageIndex = 1;

	function handleIncrement() {
		pageIndex++;
	}

	function handleReset() {
		pageIndex = 1;
	}
</script>

<div class="mb-4 flex flex-row items-center gap-4">
	<div class="flex items-center justify-center rounded-lg bg-primary/10 p-4">
		<svg
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			stroke-linecap="round"
			stroke-linejoin="round"
			class="h-4 w-4 text-primary"
		>
			<polyline points="22 12 18 12 15 21 9 3 6 12 2 12" />
		</svg>
	</div>
	<p class="text-xl">timeline</p>
</div>

{#if events && events.length > 0}
	{#each events.slice(0, 10 * pageIndex) as event, index}
		<ActivityEvent {event} last={index + 1 === 10 * pageIndex}/>
	{/each}
{:else}
	<p>
		no activity
		{#if $actor}for
			<span class="text-primary">{$actor.name} </span>
		{/if}
	</p>
{/if}
<div class="flex flex-row justify-between gap-4">
	<button
		disabled={pageIndex === Math.round(events.length / 10)}
		on:click={handleIncrement}
		class="group btn btn-outline btn-primary flex w-28 flex-col items-center justify-center"
	>
		<p>load more</p>
		<svg
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			stroke-linecap="round"
			stroke-linejoin="round"
			class="h-4 w-4 group-hover:animate-bounce"
			><circle cx="12" cy="12" r="10" />
			<polyline points="8 12 12 16 16 12" />
			<line x1="12" y1="8" x2="12" y2="16" />
		</svg>
	</button>
	<button
		disabled={pageIndex === 1}
		on:click={handleReset}
		class="group btn btn-outline btn-primary flex w-28 flex-col items-center justify-center"
	>
		<p>hide</p>
		<svg
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			stroke-linecap="round"
			stroke-linejoin="round"
			class="h-4 w-4 group-hover:animate-bounce"
		>
			<circle cx="12" cy="12" r="10" />
			<polyline points="16 12 12 8 8 12" />
			<line x1="12" y1="16" x2="12" y2="8" />
		</svg>
	</button>
</div>
