<script lang="ts">
	import { readActor } from "$lib/api/actor";
	import { readEventListForActor } from "$lib/api/event";
	import PaginationActor from "$lib/components/actor/PaginationActor.svelte";
	import { actor } from "$lib/stores/actor";
	import type { Event } from "$lib/types/event/EventTypes";
	import { onMount } from "svelte";

	let error = "";
	let events: Event[] = [];

	async function readEvents(actorId: number) {
		const res = await readEventListForActor(actorId);
		if (!res.ok) {
			error = "Something went wrong";
			return;
		}
		try {
			const newEvents = await res.json();
			events = newEvents.sort(
				(a: Event, b: Event) => new Date(b.ts).getDate() - new Date(a.ts).getDate()
			);
		} catch (e) {
			error = "Something went wrong";
		}
	}

	$: if ($actor) {
		readEvents($actor.id);
	}

	onMount(async () => {
		if ($actor) {
			await readEvents($actor.id);
		} else {
			const actorId = Number(location.pathname.split("/")[2]);
			const actorRes = await readActor(actorId);
			if (actorRes.ok) {
				try {
					const newActor = await actorRes.json();
					actor.set(newActor);
				} catch (e) {
					error = "Something went wrong";
					return;
				}
			} else {
				error = "Something went wrong";
				return;
			}
		}
	});
</script>

<div class="flex flex-row border-b border-b-base-content/10 p-4">
	<h1 class="text-3xl font-bold brightness-150">Actor</h1>
</div>

{#if error}
	<div class="p-4">
		<p class="self-center text-2xl font-bold text-error">
			{error}
		</p>
	</div>
{/if}
<div class="h-full overflow-auto">
	{#if $actor}
		<div class="m-4">
			<div class="flex w-full flex-row items-center gap-4 border-b-2 border-neutral p-4">
				<div class="flex items-center justify-center rounded-lg bg-primary/10 p-4">
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
				<p class="text-3xl font-bold">{$actor.name}</p>
			</div>
			<div class="border-b-2 border-neutral p-4">
				<div class="mb-4 flex flex-row items-center gap-4">
					<div class="flex items-center justify-center rounded-lg bg-primary/10 p-4">
						<svg
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
							class="h-4 w-4 text-primary"
						>
							<circle cx="12" cy="12" r="10" />
							<line x1="12" y1="16" x2="12" y2="12" />
							<line x1="12" y1="8" x2="12.01" y2="8" />
						</svg>
					</div>
					<p class="text-xl font-bold">properties</p>
				</div>

				{#if Object.keys($actor.properties).length > 0}
					<div class="flex flex-col gap-2">
						{#each Object.entries($actor.properties) as [key, value]}
							<div class="flex flex-row items-center justify-between">
								<p>{key}:</p>
								<p>{value}</p>
							</div>
						{/each}
					</div>
				{:else}
					<p>
						no properties for
						<span class="text-primary">{$actor.name} </span>
					</p>
				{/if}
			</div>
			<div class="flex flex-col p-4">
				<PaginationActor {events} />
			</div>
		</div>
	{:else}
		<div class="p-4">
			<p class="self-center text-2xl font-bold">No actor found with that id</p>
		</div>
	{/if}
</div>
