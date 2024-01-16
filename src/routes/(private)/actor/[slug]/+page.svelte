<script lang="ts">
	import PaginationActor from "$lib/components/actor/PaginationActor.svelte";
	import type { PageData } from "./$types";

	export let data: PageData;

	const actor = data.actor;
	let events = data.events;
</script>

<div class="flex flex-row border-b border-b-base-content/10 p-4">
	<h1 class="text-3xl font-bold brightness-150">Actor</h1>
</div>

{#if data.error}
	<div class="p-4">
		<p class="self-center text-2xl font-bold text-error">
			{data.error}
		</p>
	</div>
{/if}

<div class="h-full overflow-auto">
	{#if actor}
		<div class="m-4 max-w-lg rounded-md border-2 border-neutral bg-base-300">
			<div class="flex w-full flex-row items-center gap-4 border-b-2 border-neutral p-4">
				<div class="rounded bg-base-100 p-4">
					<svg
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
						class="h-4 w-4 text-primary"
					>
						<circle cx="12" cy="12" r="4" />
						<path d="M16 8v5a3 3 0 0 0 6 0v-1a10 10 0 1 0-3.92 7.94" />
					</svg>
				</div>
				<p class="text-xl">{actor.name}</p>
			</div>
			<div class="border-b-2 border-neutral p-4">
				<div class="flex flex-col gap-2">
					{#each Object.entries(actor.properties) as [key, value]}
						<div class="flex flex-row items-center justify-between">
							<p class="text-xl">{key}:</p>
							<p class="text-xl">{value}</p>
						</div>
					{/each}
				</div>
			</div>
			<div class="flex flex-col gap-4 p-4">
				<PaginationActor {events} />
			</div>
		</div>
	{:else}
		<div class="p-4">
			<p class="self-center text-2xl font-bold">No actor found with that id</p>
		</div>
	{/if}
</div>
