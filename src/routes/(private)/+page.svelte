<script lang="ts">
	import { readEventListForChannel } from "$lib/api/event";
	import Event from "$lib/components/event/Event.svelte";
	import { channel } from "$lib/stores/channel";
	import { events } from "$lib/stores/event";
	import { project } from "$lib/stores/project";
	import { onMount } from "svelte";
	import { fade } from "svelte/transition";

	let socket: WebSocket;
	async function readEvents(channelId: number) {
		const eventRes = await readEventListForChannel(channelId);
		if (eventRes.status === 200) {
			events.set(await eventRes.json());
		} else {
			events.set([]);
		}
	}

	$: if ($channel) {
		readEvents($channel.id);
	}

	function connectToWs(id: number) {
		socket = new WebSocket("ws://localhost:3000/ws/events");

		socket.addEventListener("open", () => {
			socket.send(`channelId:${id}`);
		});
		socket.addEventListener("message", (event) => {
			const data = JSON.parse(event.data);
			console.log(data);
		});
	}

	onMount(() => {
		if (!$channel) {
			return;
		}
	});

	$: if ($channel) {
		connectToWs($channel.id);
	}
</script>

<div class="flex flex-row items-center justify-between border-b border-b-base-content/10 p-4">
	<h1 class="text-3xl font-bold brightness-150">feed</h1>
	<button
		on:click={() => {
			if ($channel) {
				if (socket) {
					socket.send(`channelId:${$channel.id}`);
				}
				readEvents($channel.id);
			}
		}}
		class="pr-4 hover:text-primary"
	>
		<svg
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			stroke-linecap="round"
			stroke-linejoin="round"
			class="h-4 w-4"
		>
			<polyline points="23 4 23 10 17 10" />
			<polyline points="1 20 1 14 7 14" />
			<path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15" />
		</svg>
	</button>
</div>

<div class="flex flex-col items-center gap-4 overflow-auto p-4">
	{#each [...$events] as event, index}
		{#key event.id}
			<div
				class="flex w-full flex-col items-center"
				in:fade|global={{ delay: 75 * (index + 1) }}
			>
				<Event {event} />
			</div>
		{/key}
	{/each}
</div>

{#if $channel && $events.length === 0}
	<div class="flex h-full w-full flex-col items-center justify-center">
		<h1 class="text-3xl font-bold">
			No events in
			<span class="text-primary">#{$channel.title} </span>
		</h1>
	</div>
{/if}

{#if !$project && !$channel && $events.length === 0}
	<div class="flex h-full w-full flex-col items-center justify-center">
		<h1 class="text-3xl font-bold">No project selected</h1>
	</div>
{/if}

{#if $project && !$channel && $events.length === 0}
	<div class="flex h-full w-full flex-col items-center justify-center">
		<h1 class="text-3xl font-bold">
			No channel selected in
			<span class="text-primary">#{$project.title} </span>
		</h1>
	</div>
{/if}
