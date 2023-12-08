<script lang="ts">
	import Event from '$lib/components/event/Event.svelte';

	let events = [
		{
			id: 1,
			icon: '😴',
			title: 'Test Event',
			channel_title: 'Test Channel',
			user_name: 'Test User',
			ts: '2023-12-08T10:10:10.000Z'
		},

		{
			id: 2,
			icon: '🤑',
			title: 'new-event-title',
			channel_title: 'Test Channel',
			user_name: 'Test User',
			ts: '2023-12-07T10:10:10.000Z'
		},

		{
			id: 3,
			icon: '🤑',
			title: 'new-event-title',
			channel_title: 'Test Channel',
			user_name: 'Test User',
			ts: '2023-12-01T10:10:10.000Z'
		}
	];
</script>

<div class="flex flex-row justify-between border-b border-b-base-content/10 p-4">
	<h1 class="text-3xl font-bold">feed</h1>
</div>

<div class="flex flex-col gap-4">
	{#each events as event}
		<Event {event} />
	{/each}
</div>
