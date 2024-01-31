<script lang="ts">
	import { readActorListForProject } from "$lib/api/actor";
	import ListItem from "$lib/components/settings/ListItem.svelte";
	import Loading from "$lib/layout/loading/Loading.svelte";
	import { actors } from "$lib/stores/actor";
	import { project } from "$lib/stores/project";
	import { onMount } from "svelte";

	let loading = true;

	async function updateActorsList() {
		loading = true;
		let res = await readActorListForProject($project!.id);
		if (res.status === 200) {
			try {
				const updatedActors = await res.json();
				actors.set(updatedActors);
			} catch (e) {
				console.error(e);
			}
		}
		loading = false;
	}

	$: $project, updateActorsList();

	onMount(async () => {
		updateActorsList();
	});
</script>

<div class="flex flex-row justify-between border-b border-b-base-content/10 p-4">
	<h1 class="text-3xl font-bold brightness-150">actors</h1>
</div>

{#if loading}
	<div class="relative">
		<Loading />
	</div>
{/if}
<div class="flex w-full max-w-2xl flex-col items-start p-4">
	<div
		class="flex w-full flex-row items-center justify-start gap-4 rounded-tl-md rounded-tr-md border-l-2 border-r-2 border-t-2 border-neutral bg-base-300 p-4"
	>
		<div class="flex items-center justify-center rounded-lg bg-primary/10 p-4">
			<svg
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
				class="h-4 w-4 text-primary"
				><circle cx="12" cy="12" r="4"></circle><path
					d="M16 8v5a3 3 0 0 0 6 0v-1a10 10 0 1 0-3.92 7.94"
				></path></svg
			>
		</div>
		<h1 class="text-2xl brightness-150">actors for {$project?.title}</h1>
	</div>
	{#if $actors.length === 0}
		<div
			class="flex w-full flex-col items-start justify-center gap-4 rounded-bl-md rounded-br-md border-b-2 border-l-2 border-r-2 border-t-2 border-neutral bg-base-300 p-4"
		>
			<p class="text-base-content">no actors yet created</p>
		</div>
	{/if}
	{#each $actors as actor, index}
		<div
			data-last={index === $actors.length - 1}
			class="flex w-full flex-col items-start justify-center gap-4 border-l-2 border-r-2 border-t-2 border-neutral bg-base-300 p-4 data-[last=true]:rounded-bl-md data-[last=true]:rounded-br-md data-[last=true]:border-b-2"
		>
			<ListItem name={actor.name} hrefLink={`/settings/actors/${actor.id}`} />
		</div>
	{/each}
</div>
