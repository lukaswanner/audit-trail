<script lang="ts">
	import Event from '$lib/components/event/Event.svelte';

	let events = [];
</script>

<div class="flex flex-row justify-between border-b border-b-base-content/10 p-4">
	<h1 class="text-3xl font-bold">feed</h1>
</div>

<div class="flex flex-col gap-4">
	{#each events as event}
		<Event {event} />
	{/each}
</div>
