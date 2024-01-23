<script lang="ts">
	import type { Event } from "$lib/types/event/EventTypes";
	import Tags from "../event/Tags.svelte";

	export let event: Event;
	export let last: boolean;
</script>

<div
	id="wrapper"
	class:mb-4={last}
	class="grid h-20 grid-cols-[5%_75%_20%] gap-4 transition-all hover:h-36"
>
	<div class:column={!last} class="relative flex flex-col justify-self-center">
		<div class="flex h-10 w-10 items-center justify-center rounded-full bg-primary">
			<p class="text-xl">
				{event.icon}
			</p>
		</div>
	</div>
	<div class="flex w-full flex-col justify-self-start">
		<div class="h-14">
			<p class="text-xl brightness-150">
				{new Date(event.ts).toLocaleDateString()}
				{new Date(event.ts).toLocaleTimeString()}
			</p>

			<p class="text-xl brightness-150">
				{event.title}
			</p>
		</div>
		<div class="actions mt-4 hidden transition-all">
			<Tags tags={event.tags} />
		</div>
	</div>
	<div class="flex flex-col justify-self-end pr-8">
		<div class="h-14">
			<p class="text-xl">
				#{event.channelTitle}
			</p>
		</div>
		<div class="actions mt-4 hidden transition-all">
			<a href={`/event/${event.id}`} class="btn btn-outline btn-secondary h-8 min-h-[2rem]">
				go to event
			</a>
		</div>
	</div>
</div>

<style>
	.column::after {
		content: "";
		position: absolute;
		top: 2.5rem;
		bottom: 4px;
		left: calc(50% - 1px);
		display: block;
		width: 2px;
		background-color: theme("colors.secondary");
	}

	#wrapper:hover .actions {
		display: flex;
		flex-direction: row;
		width: 100%;
		justify-content: space-between;
		gap: 1rem;
	}
</style>
