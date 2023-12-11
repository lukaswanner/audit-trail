<script lang="ts">
	import { readEventListForChannel } from '$lib/api/event';
	import Event from '$lib/components/event/Event.svelte';
	import { channel } from '$lib/stores/channel';
	import { events } from '$lib/stores/event';

	async function readEvents(channelTitle: string) {
		const eventRes = await readEventListForChannel(channelTitle);
		if (eventRes.status === 200) {
			events.set(await eventRes.json());
		} else {
			events.set([]);
		}
	}

	$: if ($channel) {
		readEvents($channel.title);
	}
</script>

<div class="flex flex-row justify-between border-b border-b-base-content/10 p-4">
	<h1 class="text-3xl font-bold">feed</h1>
</div>

<div class="flex flex-col gap-4">
	{#each $events as event}
		<Event {event} />
	{/each}
</div>