<script lang="ts">
	import type { Event } from '$lib/types/event/EventTypes';

	export let event: Event;
</script>

<div class="flex flex-row justify-between rounded border border-neutral p-2">
	<div>
		<p>{event.title}</p>
		<p>#{event.channelTitle}</p>
	</div>
	<div>
		<p>{new Date(event.ts).toLocaleDateString()}</p>
		<p>{new Date(event.ts).toLocaleTimeString()}</p>
	</div>
</div>
