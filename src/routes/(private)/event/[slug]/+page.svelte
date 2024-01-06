<script lang="ts">
	import { readEvent } from '$lib/api/event';
	import Loading from '$lib/layout/loading/Loading.svelte';
	import { onMount } from 'svelte';
	import type { Event as EventType } from '$lib/types/event/EventTypes';
	import type { PageData } from './$types';
	import { formatRelative } from 'date-fns';
	import Tag from '$lib/components/event/Tag.svelte';

	let loading = true;
	let error: boolean;
	let event: EventType;

	export let data: PageData;

	async function readSlugEvent(eventId: number) {
		const eventRes = await readEvent(eventId);
		if (eventRes.status === 200) {
			const newEvent = await eventRes.json();
			if (newEvent) {
				event = newEvent;
				loading = false;
			}
		}
	}

	onMount(() => {
		if (!isNaN(parseInt(data.slug))) {
			loading = true;
			readSlugEvent(parseInt(data.slug));
		} else {
			loading = false;
			error = true;
		}
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
	<h1 class="mx-auto text-3xl font-bold brightness-150">log</h1>
</div>

{#if loading}
	<div class="relative">
		<Loading />
	</div>
{:else if error}
	<p class="self-center text-2xl font-bold">
		Can't find event with the following id:
		<span class="text-error">{data.slug}</span>
	</p>
{:else}
	<div class="grid grid-cols-[auto,1fr,auto] gap-4 p-4">
		<div
			class="flex aspect-square flex-col items-center justify-center rounded-3xl bg-primary/10 p-2"
		>
			<div class="text-4xl">{event.icon}</div>
		</div>
		<div class="flex flex-col justify-center brightness-150">
			<div class="text-2xl font-bold">{event.title}</div>
			<div class="text-base-content/50">{event.channelTitle}</div>
		</div>
		<div class="flex flex-col items-center justify-center">
			<div class="text-base-content/50">{event.userName}</div>
			<div class="text-base-content/50">{formatRelative(new Date(event.ts), new Date())}</div>
		</div>
	</div>

	<div class="divider my-0" />
	<div class="flex flex-row flex-wrap items-center gap-4 p-4">
		{#each event.tags as tag}
			<a href="/search">
				<Tag {tag} />
			</a>
		{/each}
	</div>
{/if}
