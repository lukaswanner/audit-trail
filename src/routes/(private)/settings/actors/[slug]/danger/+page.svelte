<script lang="ts">
	import { goto } from "$app/navigation";
	import { page } from "$app/stores";
	import { deleteActor, readActorListForProject } from "$lib/api/actor";
	import { actors } from "$lib/stores/actor";
	import { project } from "$lib/stores/project";

	let error: string;
	let id = decodeURIComponent($page.url.pathname.split("/")[3]);
	let actorName = $actors.find((actor) => actor.id === Number(id))?.name;

	let inputName = "";

	async function handleDelete() {
		if (!id || Number.isNaN(Number(id))) {
			return;
		}
		const res = await deleteActor(Number(id));
		if (res.status === 204) {
			let res = await readActorListForProject($project!.id);
			if (res.status === 200) {
				try {
					const updatedActorList = await res.json();
					actors.set(updatedActorList);
					goto("/settings/actors");
				} catch (e) {
					console.error(e);
				}
			}
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
	<h1 class="text-3xl font-bold brightness-150">delete actor</h1>
</div>

<div class="flex max-w-2xl flex-col items-start gap-4 p-4">
	<form
		class="flex w-full flex-col items-start justify-center gap-4 rounded-md border-2 border-neutral bg-base-300 p-4"
		on:submit|preventDefault={handleDelete}
	>
		<p>Are you sure you want to delete this actor?</p>
		<p>
			To confirm, please type
			<span class="text-error">
				{actorName}
			</span>
			in the field below.
		</p>
		<input
			type="text"
			name="title"
			class="input input-bordered w-full max-w-md"
			placeholder="actor name"
			bind:value={inputName}
		/>
		{#if error}
			<p class="text-xs text-error">{error}</p>
		{/if}
		<button disabled={inputName.trim() !== actorName?.trim()} class="btn btn-error w-24"
			>delete</button
		>
	</form>
</div>
