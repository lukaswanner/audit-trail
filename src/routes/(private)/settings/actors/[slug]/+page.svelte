<script lang="ts">
	import { goto } from "$app/navigation";
	import { page } from "$app/stores";
	import { readActorListForProject, updateActor } from "$lib/api/actor";
	import Properties from "$lib/components/actor/Properties.svelte";
	import { actors } from "$lib/stores/actor";
	import { project } from "$lib/stores/project";
	import type { Actor } from "$lib/types/actor/ActorTypes";
	import { onMount } from "svelte";

	let error: string;
	let actorId = decodeURIComponent($page.url.pathname.split("/")[3]);
	let currentActor: Actor | undefined;

	let inputName = "";
	let properties: Map<string, string>;

	async function updateActorsList() {
		let res = await readActorListForProject($project!.id);
		if (res.status === 200) {
			try {
				const updatedActors = await res.json();
				actors.set(updatedActors);
			} catch (e) {
				console.error(e);
			}
		}
		error = "";
	}

	function add(key: string, value: string) {
		properties.set(key, value);
		properties = properties;
	}
	function remove(key: string) {
		properties.delete(key);
		properties = properties;
	}

	async function handleNameChange() {
		// make sure we have the latest actor list
		if (Number.isNaN(Number(actorId))) {
			error = "invalid actor id";
			return;
		}
		await updateActorsList();
		currentActor = $actors.find((actor) => actor.id === Number(actorId));
		if (!$project || !$project.id) {
			error = "no project selected";
			return;
		}
		if (!currentActor) {
			error = "actor not found";
			return;
		}
		const payload = {
			id: currentActor.id,
			projectId: $project.id,
			name: inputName,
			properties: Object.fromEntries(properties)
		};

		const res = await updateActor(payload);
		if (res.status === 200) {
			await updateActorsList();
			goto("/settings/actors");
		}
	}

	onMount(() => {
		if (Number.isNaN(Number(actorId))) {
			error = "invalid actor id";
		}
		currentActor = $actors.find((actor) => actor.id === Number(actorId));
		if (!currentActor) {
			error = "actor not found";
		}
		if (currentActor && currentActor.name) {
			inputName = currentActor.name;
			properties = new Map(Object.entries(currentActor.properties));
		}
	});
</script>

<div class="flex flex-row items-center gap-4 border-b border-b-base-content/10 p-4">
	<a href="/settings/actors">
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
	<h1 class="text-3xl font-bold">
		<span class="brightness-150"> update </span>
		{#if currentActor && currentActor.name}
			<span class="brightness-150">
				{currentActor.name}
			</span>
		{:else}
			<span class="text-error">actor not found</span>
		{/if}
	</h1>
</div>

<div class="w-full max-w-2xl p-4">
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
		<h1 class="text-2xl">
			<span class="brightness-150"> settings for </span>
			{#if currentActor && currentActor.name}
				<span class="brightness-150">
					{currentActor.name}
				</span>
			{:else}
				<span class="text-error">actor not found</span>
			{/if}
		</h1>
	</div>
	<div
		class="flex w-full flex-col items-start justify-center gap-4 rounded-bl-md rounded-br-md border-b-2 border-l-2 border-r-2 border-t-2 border-neutral bg-base-300 p-4"
	>
		<p class="text-base-content">change the name, add, remove or update properties</p>
	</div>
</div>
<div class="flex max-w-2xl flex-col items-start gap-4 p-4">
	<form class="flex w-full flex-col gap-4" on:submit|preventDefault={handleNameChange}>
		<div
			class="flex w-full flex-col items-start justify-center gap-4 rounded-md border-2 border-neutral bg-base-300 p-4"
		>
			<p>actor name</p>
			<input
				type="text"
				name="name"
				class="input input-bordered w-full max-w-md"
				placeholder="actor name"
				bind:value={inputName}
			/>
			{#if error}
				<p class="text-xs text-error">{error}</p>
			{/if}
		</div>
		<div
			class="flex w-full flex-col items-start justify-center gap-4 rounded-md border-2 border-neutral bg-base-300 p-4"
		>
			<div class="w-full">
				<p class="mb-4">Properties</p>
				{#if currentActor && currentActor.properties}
					<Properties {properties} {add} {remove} />
				{/if}
			</div>
			<button class="btn btn-primary w-24">update</button>
		</div>
	</form>
</div>
