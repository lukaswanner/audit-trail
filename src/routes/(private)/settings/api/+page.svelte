<script lang="ts">
	import { channel } from "$lib/stores/channel";
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

<div class="flex flex-col items-center gap-4 overflow-auto p-4">
	<div class="flex flex-col items-center">
		<h2 class="text-2xl brightness-150">api keys</h2>
		<p>API keys are required for publishing data to your project.</p>
	</div>

	{#if loading}
		<Loading />
	{/if}
	{#each $apikeys as apikey}
		<Apikey {apikey} />
	{/each}
	{#if $apikeys.length === 0}
		<h2 class="text-3xl font-bold">
			No api keys in
			<span class="text-primary">#{$channel.title} </span>
		</h2>
	{/if}
	<button on:click={createNewApikey} class="btn btn-primary"> + create key</button>
</div>
