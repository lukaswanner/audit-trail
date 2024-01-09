<script lang="ts">
	import type { Event } from '$lib/types/event/EventTypes';
	import { formatRelative } from 'date-fns';

	export let event: Event;
</script>

<a href="/event/{event.id}" class="block max-w-5xl cursor-pointer overflow-hidden">
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
		<div class="flex flex-col items-end justify-center">
			<div class="flex flex-col gap-2">
				{#if event.tags.length > 0}
					<div class="rounded-md border border-primary bg-primary/50 px-2 py-1">
						<p>
							{Object.keys(event.tags[0])[0]}:
							<span class="brightness-150">
								{Object.values(event.tags[0])[0]}
							</span>
						</p>
					</div>
				{/if}
				{#if event.tags.length > 1}
					<div
						id="has-tooltip"
						class="w-fit self-end rounded-md border border-primary bg-primary/50 px-2 py-1"
					>
						+{event.tags.length - 1}
						<span id="tooltip-wrapper">
							<div
								class="relative right-full top-9 flex h-fit w-fit flex-col gap-2 rounded-md border border-base-300 bg-base-300/80 p-4"
							>
								{#each event.tags.slice(1) as tag}
									<div
										class="whitespace-nowrap rounded-md border border-primary bg-primary/50 px-2 py-1"
									>
										<p>
											{Object.keys(tag)[0]}:
											<span class="brightness-150">
												{Object.values(tag)[0]}
											</span>
										</p>
									</div>
								{/each}
							</div>
						</span>
					</div>
				{/if}
			</div>
		</div>
	</div>
</a>

<style>
	#tooltip-wrapper {
		position: absolute;
		visibility: hidden;
	}

	#has-tooltip:hover #tooltip-wrapper {
		visibility: visible;
	}
</style>
