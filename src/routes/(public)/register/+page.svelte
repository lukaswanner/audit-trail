<script lang="ts">
	import RegisterCard from '$lib/components/auth/RegisterCard.svelte';
	import type { PseudoEvent } from '$lib/types/login/login';

	import { addToEventLog } from '$lib/utils/login/handleEventLog';
	import { formatRelative } from 'date-fns';
	import { flip } from 'svelte/animate';
	import { slide } from 'svelte/transition';

	let events: PseudoEvent[] = [];
	let clear: number;

	$: {
		clearInterval(clear);
		clear = setInterval(() => {
			events.shift();
			events = [...events];
		}, 2000);
	}
</script>

<div class="grid h-full w-full grid-cols-1 grid-rows-[1fr] lg:grid-cols-[2fr_4fr]">
	<RegisterCard
		addToEventLog={(msg) => {
			events = addToEventLog(events, msg);
		}}
	/>
	<div class="hidden flex-col items-center justify-start gap-4 overflow-auto lg:flex">
		{#each events as event (event)}
			<div
				animate:flip
				transition:slide|local
				class="grid w-full max-w-md grid-cols-[auto_1fr] gap-4 rounded-3xl border-2 border-secondary p-4"
			>
				<div
					class="flex aspect-square w-16 flex-col items-center justify-center rounded-3xl bg-primary/30"
				>
					<div class="text-3xl">{event.icon}</div>
				</div>
				<div class="flex flex-col justify-between">
					<p class="text-bold text-xl">{event.title}</p>
					<div class="flex w-full flex-row justify-between gap-2">
						<p>Website</p>
						<p>{formatRelative(event.timeStamp, new Date())}</p>
					</div>
				</div>
			</div>
		{/each}
	</div>
</div>
