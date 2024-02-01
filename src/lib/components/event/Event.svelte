<script lang="ts">
	import type { Event } from "$lib/types/event/EventTypes";
	import { formatRelative } from "date-fns";
	import Tags from "./Tags.svelte";

	export let event: Event;
	export let enableTags = true;
</script>

<a href="/event/{event.id}" class="block w-full max-w-5xl cursor-pointer">
	<div class="grid grid-cols-[auto,1fr,auto] gap-4 rounded-box border border-neutral p-4">
		<div
			class="flex h-16 w-16 items-center justify-center self-center rounded-full bg-primary/20"
		>
			<p class="text-4xl">{event.icon}</p>
		</div>
		<div class="flex flex-col justify-center">
			<div class="text-2xl font-bold">{event.title}</div>
			<div class="mb-2 text-lg">{event.description}</div>
			<div class="flex flex-row items-center gap-2">
				<p class="text-sm">
					{event.actorName}
				</p>
				<div class="h-1 w-1 rounded-full bg-accent" />
				<p>{formatRelative(new Date(event.ts), new Date())}</p>
			</div>
		</div>
		{#if enableTags}
			<Tags tags={event.tags} />
		{/if}
	</div>
</a>
