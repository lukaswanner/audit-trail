<script lang="ts">
	import type { Event } from '$lib/types/event/EventTypes';
	import { formatRelative } from 'date-fns';

	export let event: Event;
</script>

<div class="grid grid-cols-[auto,1fr,auto] gap-4 p-4">
	<div
		class="flex aspect-square flex-col items-center justify-center rounded-3xl bg-primary/10 p-2"
	>
		<div class="text-4xl">{event.icon}</div>
	</div>
	<div class="flex flex-col justify-center">
		<div class="text-2xl font-bold">{event.title}</div>
		<div class="text-base-content/50">{event.channelTitle}</div>
	</div>
	<div class="flex flex-col items-center justify-center">
		<div class="text-base-content/50">{event.userName}</div>
		<div class="text-base-content/50">{formatRelative(new Date(event.ts), new Date())}</div>
	</div>
</div>
