<script lang="ts">
	import { page } from '$app/stores';
	import { deleteActor, readActorList } from '$lib/api/actor';
	import { actors } from '$lib/stores/actor';

	let error: string;
	let actorName = decodeURIComponent($page.url.pathname.split('/')[3]);

	let inputName = '';

	async function handleDelete() {
		const id = $actors.find((actor) => actor.name === actorName)?.id;
		if (!id) {
			return;
		}
		const res = await deleteActor(id);
		if (res.status === 204) {
			let res = await readActorList();
			if (res.status === 200) {
				try {
					const updatedActorList = await res.json();
					actors.set(updatedActorList);
				} catch (e) {
					console.error(e);
				}
			}
		}
	}
</script>

<div class="flex flex-row justify-between border-b border-b-base-content/10 p-4">
	<h1 class="text-3xl font-bold brightness-150">delete actor</h1>
</div>

<div class="flex flex-col items-start gap-4 p-4">
	<form
		class="flex w-full max-w-md flex-col items-start justify-center gap-4 rounded-md border-2 border-neutral bg-base-300 p-4"
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
