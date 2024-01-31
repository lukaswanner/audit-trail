<script lang="ts">
	import { createApikey, readApikeyList } from "$lib/api/apikey";
	import { onMount } from "svelte";
	import Loading from "$lib/layout/loading/Loading.svelte";
	import Apikey from "$lib/components/api/Apikey.svelte";
	import { apikeys } from "$lib/stores/apikey";
	import { project } from "$lib/stores/project";

	let loading = true;

	async function readApikeys(initialLoading = true) {
		loading = initialLoading;
		const res = await readApikeyList();
		if (res.status === 200) {
			try {
				apikeys.set(await res.json());
			} catch (error) {
				console.error(error);
			}
		}
		loading = false;
	}

	async function createNewApikey() {
		if (!$project) return;
		const res = await createApikey($project.id);
		if (res.status === 200) {
			try {
				readApikeys(false);
			} catch (error) {
				console.error(error);
			}
		}
	}

	onMount(() => {
		readApikeys();
	});
</script>

<div class="flex flex-row justify-between border-b border-b-base-content/10 p-4">
	<h1 class="text-3xl font-bold brightness-150">api</h1>
</div>

<div class="mb-8 flex h-full max-w-2xl flex-col items-start gap-4 overflow-auto p-4">
	<div class="w-full">
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
				>
					<polygon points="12 2 2 7 12 12 22 7 12 2" />
					<polyline points="2 17 12 22 22 17" />
					<polyline points="2 12 12 17 22 12" />
				</svg>
			</div>
			<h1 class="text-2xl brightness-150">your api keys</h1>
		</div>
		<div
			class="flex w-full flex-col items-start justify-center gap-4 rounded-bl-md rounded-br-md border-b-2 border-l-2 border-r-2 border-t-2 border-neutral bg-base-300 p-4"
		>
			{#if !$project}
				<div class="my-auto">
					<h2 class="text-xl font-bold">No project selected</h2>
				</div>
			{/if}
			{#if $project && $apikeys.length === 0}
				<h2 class="text-xl font-bold">
					No api keys in
					<span class="text-primary">#{$project.title} </span>
				</h2>
			{/if}
			{#if $project && $apikeys.length > 0}
				<h2 class="text-xl font-bold">
					{$apikeys.length} api keys in
					<span class="text-primary">#{$project.title} </span>
				</h2>
			{/if}
		</div>
	</div>

	<div class="w-full">
		{#each $apikeys as apikey, index}
			<div
				data-last={index === $apikeys.length - 1}
				data-first={index === 0}
				class="flex w-full flex-col items-start justify-center gap-4 border-l-2 border-r-2 border-t-2 border-neutral bg-base-300 p-4 data-[first=true]:rounded-tl-md data-[first=true]:rounded-tr-md data-[last=true]:rounded-bl-md data-[last=true]:rounded-br-md data-[last=true]:border-b-2"
			>
				<Apikey {apikey} />
			</div>
		{/each}
	</div>
	<button on:click={createNewApikey} class="btn btn-primary"> + create key</button>
	{#if loading}
		<Loading />
	{/if}
</div>
