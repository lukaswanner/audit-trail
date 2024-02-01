<script lang="ts">
	import type { Event } from "$lib/types/event/EventTypes";
	import Tags from "../event/Tags.svelte";

	export let event: Event;
	export let last: boolean;
	export let tabIndex: number;
</script>

<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
<div tabindex={tabIndex} class="collapse mb-4 border border-neutral p-4">
	<div class="collapse-title flex flex-row flex-wrap gap-4">
		<div class:column={!last} class="relative flex flex-col justify-self-center">
			<div class="flex h-16 w-16 items-center justify-center rounded-full bg-primary/20">
				<p class="text-4xl">
					{event.icon}
				</p>
			</div>
		</div>
		<div class="flex flex-col justify-self-start">
			<div>
				<p class="text-xl brightness-150">
					#{event.channelTitle}: {event.title}
				</p>
				<p class="text-sm">
					{new Date(event.ts).toLocaleDateString()}
					{new Date(event.ts).toLocaleTimeString()}
				</p>
			</div>
		</div>
		<div class="flex flex-row items-center justify-center sm:ml-auto">
			<Tags tags={event.tags} />
		</div>
	</div>
	<div class="collapse-content flex flex-col gap-4">
		<p>
			{event.description}
		</p>
		<a
			href={`/event/${event.id}`}
			class="btn btn-outline btn-secondary h-8 min-h-[2rem] max-w-fit"
		>
			go to event
		</a>
	</div>
</div>
