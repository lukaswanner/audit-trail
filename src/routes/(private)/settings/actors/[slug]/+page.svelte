<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { readActorListForProject, updateActor } from '$lib/api/actor';
	import { actors } from '$lib/stores/actor';
	import { project } from '$lib/stores/project';

	let error: string;
	let actorName = decodeURIComponent($page.url.pathname.split('/')[3]);

	let inputName = '';

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

	async function handleNameChange() {
		// make sure we have the latest actor list
		await updateActorsList();
		const currentActor = $actors.find((actor) => actor.name === actorName);
		if (!$project || !$project.id) {
			error = 'no project selected';
			return;
		}
		if (!currentActor) {
			error = 'actor not found';
			return;
		}
		const payload = {
			id: currentActor.id,
			projectId: $project.id,
			name: inputName,
			properties: currentActor.properties
		};

		const res = await updateActor(payload);
		if (res.status === 200) {
			await updateActorsList();
			goto('/settings/actors');
		}
	}
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
	<h1 class="text-3xl font-bold brightness-150">update actor</h1>
</div>

<div class="flex flex-col items-start gap-4 p-4">
	<form
		class="flex w-full max-w-md flex-col items-start justify-center gap-4 rounded-md border-2 border-neutral bg-base-300 p-4"
		on:submit|preventDefault={handleNameChange}
	>
		actor name
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
		<button disabled={inputName.trim() === ''} class="btn btn-success w-24">update</button>
	</form>
</div>
