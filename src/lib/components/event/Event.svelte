<script lang="ts">
	import type { Event } from '$lib/types/event/EventTypes';
	import { formatRelative } from 'date-fns';
	import Tags from './Tags.svelte';

	export let event: Event;
	export let enableTags = true;
</script>

<a href="/event/{event.id}" class="block max-w-5xl cursor-pointer">
	<div class="grid grid-cols-[auto,1fr,auto] gap-4 rounded-md border border-neutral p-4">
		<div
			class="flex aspect-square flex-col items-center justify-center rounded-3xl bg-primary/10 p-2"
		>
			<div class="text-4xl">{event.icon}</div>
		</div>
		<div class="flex flex-col justify-center">
			<div class="text-2xl font-bold">{event.title}</div>
			<p class="text-base-content">
				{event.userName} | <span>{formatRelative(new Date(event.ts), new Date())}</span>
			</p>
		</div>
		{#if enableTags}
			<Tags tags={event.tags} />
		{/if}
	</div>
</a>
