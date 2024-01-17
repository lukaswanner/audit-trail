<script lang="ts">
	import { createActor, readActorListForProject } from "$lib/api/actor";
	import { project } from "$lib/stores/project";
	import { actor, actors } from "$lib/stores/actor";
	import type { ActorPayload } from "$lib/types/actor/ActorTypes";
	import { page } from "$app/stores";

	let actorName = "";
	let modalRef: HTMLDialogElement;
	let error: string;

	async function fetchActors() {
		const res = await readActorListForProject($project!.id);
		if (res.status === 200) {
			const actorRes = await res.json();
			actors.set(actorRes);
		}
	}

	async function handleNewActor(e: Event) {
		const form = e.target as HTMLFormElement;
		const formData = new FormData(form);
		const data = Object.fromEntries(formData.entries());

		if (!$project) {
			error = "Select a project first";
			return;
		}
		const payload: ActorPayload = {
			projectId: $project.id,
			name: data.name as string,
			properties: {}
		};
		const res = await createActor(payload);
		if (res.status === 201) {
			await fetchActors();
			modalRef.close();
		} else if (res.status === 409) {
			error = "Actor already exists";
		} else {
			error = "Something went wrong";
		}
	}
</script>

<div class="flex w-full flex-row items-center justify-between px-4">
	<h2 class="text-xl font-bold brightness-150">Actors</h2>
	<button
		disabled={!$project}
		on:click={() => modalRef.showModal()}
		class="btn btn-ghost rounded-full fill-base-content brightness-150 transition-colors hover:fill-primary hover:brightness-100"
	>
		<svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 448 512"
			><!--!Font Awesome Free 6.5.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2023 Fonticons, Inc.--><path
				d="M256 80c0-17.7-14.3-32-32-32s-32 14.3-32 32V224H48c-17.7 0-32 14.3-32 32s14.3 32 32 32H192V432c0 17.7 14.3 32 32 32s32-14.3 32-32V288H400c17.7 0 32-14.3 32-32s-14.3-32-32-32H256V80z"
			/></svg
		>
	</button>
</div>
{#each $actors as singleActor}
	<a
		data-active={singleActor.id.toString() === $page.url.pathname.split("/")[2]}
		on:click={actor.set(singleActor)}
		href={`/actor/${singleActor.id}`}
		class="btn btn-ghost flex w-full flex-grow flex-row flex-nowrap items-center justify-start gap-2 data-[active=true]:bg-base-content/20 data-[active=true]:text-primary"
	>
		<svg
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			stroke-linecap="round"
			stroke-linejoin="round"
			class="h-4 w-4"
		>
			<circle cx="12" cy="12" r="4" />
			<path d="M16 8v5a3 3 0 0 0 6 0v-1a10 10 0 1 0-3.92 7.94" />
		</svg>

		<p class="h-12 overflow-hidden text-ellipsis whitespace-nowrap font-bold leading-[3rem]">
			{singleActor.name}
		</p>
	</a>
{/each}

<dialog id="channel-modal" bind:this={modalRef} class="modal modal-bottom sm:modal-middle">
	<form class="modal-box" on:submit|preventDefault={handleNewActor}>
		<h3 class="text-lg font-bold">Create new actor</h3>
		<p class="py-4">Create a new actor bind events to it.</p>
		<input
			name="name"
			type="text"
			placeholder="Actor name"
			class="input input-bordered w-full"
			bind:value={actorName}
		/>
		{#if error}
			<p class="text-error">{error}</p>
		{/if}
		<div class="modal-action justify-start">
			<button disabled={actorName.length === 0} class="btn btn-success" type="submit"
				>Create</button
			>
			<form method="dialog">
				<button class="btn btn-outline">Close</button>
			</form>
		</div>
	</form>
</dialog>
