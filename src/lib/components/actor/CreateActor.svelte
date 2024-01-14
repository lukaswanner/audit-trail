<script lang="ts">
	import { project } from '$lib/stores/project';
	import { createActor, readActorListForProject } from '$lib/api/actor';
	import type { ActorPayload } from '$lib/types/actor/ActorTypes';
	import { actors } from '$lib/stores/actor';
	import { onMount } from 'svelte';

	let error: string;

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
		error = '';
	}

	async function handleCreateActor(e: Event) {
		const form = e.target as HTMLFormElement;
		const formData = new FormData(form);
		const data = Object.fromEntries(formData.entries());

		if (!$project || !$project.id) {
			error = 'Select a project first';
			return;
		}

		if (!data.name || data.name === '') {
			error = 'Actor name cannot be empty';
			return;
		}

		const properties = {};

		const payload: ActorPayload = {
			name: data.name as string,
			projectId: $project.id,
			properties: properties
		};

		const res = await createActor(payload);
		if (res.status === 201) {
			error = '';
			await updateActorsList();
		} else {
			error = 'Something went wrong';
		}
	}

	$: $project, updateActorsList();

	onMount(async () => {
		updateActorsList();
	});
</script>

<div class="flex flex-col items-start gap-4 p-4">
	<form
		class="flex w-full max-w-md flex-col items-start justify-center gap-4 rounded-md border-2 border-neutral bg-base-300 p-4"
		on:submit|preventDefault={handleCreateActor}
	>
		<p>actor name</p>
		<input
			type="text"
			name="name"
			class="input input-bordered w-full max-w-md"
			placeholder="project name"
		/>
		{#if error}
			<p class="text-xs text-error">{error}</p>
		{/if}
		<button class="btn btn-primary w-24">create</button>
	</form>
</div>
