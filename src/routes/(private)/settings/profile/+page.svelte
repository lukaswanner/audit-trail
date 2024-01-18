<script lang="ts">
	import { goto } from "$app/navigation";
	import { readUser, deleteUser } from "$lib/api/user";
	import Loading from "$lib/layout/loading/Loading.svelte";
	import { projects } from "$lib/stores/project";
	import type { User } from "$lib/types/user/user";
	import { onMount } from "svelte";

	let loading = true;
	let user: User;
	let modalRef: HTMLDialogElement;
	let userMail = "";
	let error = "";

	async function deleteUserAndRedirect() {
		const res = await deleteUser();
		if (res.status === 204) {
			goto("/login");
		} else {
			error = "something went wrong";
		}
	}

	onMount(async () => {
		const res = await readUser();
		if (res.status === 200) {
			user = await res.json();
		}
		loading = false;
	});
</script>

<div class="flex flex-row justify-between border-b border-b-base-content/10 p-4">
	<h1 class="text-3xl font-bold brightness-150">profile</h1>
</div>

{#if loading}
	<div class="relative">
		<Loading />
	</div>
{/if}

{#if user}
	<div class="flex flex-col items-start gap-4 p-4">
		<div class="flex w-full flex-col">
			<div
				class="rounded-tr-mid flex w-full max-w-xl flex-col items-start justify-center gap-4 rounded-tl-md border-l-2 border-r-2 border-t-2 border-neutral bg-base-300 p-4"
			>
				<div class="flex flex-row items-center gap-4">
					<div class="flex items-center justify-center rounded bg-base-100 p-2">
						<svg
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
							class="h-4 w-4 brightness-150"
						>
							<path
								d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"
							/>
							<polyline points="22,6 12,13 2,6" />
						</svg>
					</div>

					<h2 class="brightness-150">{user.email}</h2>
				</div>
			</div>
			<div
				class="flex w-full max-w-xl flex-col items-start justify-center gap-4 rounded-bl-md rounded-br-md border-2 border-neutral bg-base-300 p-4"
			>
				<div class="flex flex-row items-center gap-4">
					<p>email</p>
				</div>
			</div>
		</div>
		<div class="flex w-full flex-col">
			<div
				class="rounded-tr-mid flex w-full max-w-xl flex-col items-start justify-center gap-4 rounded-tl-md border-l-2 border-r-2 border-t-2 border-neutral bg-base-300 p-4"
			>
				<div class="flex flex-row items-center gap-4">
					<div class="flex items-center justify-center rounded bg-base-100 p-2">
						<svg
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
							class="h-4 w-4 brightness-150"
						>
							<path
								d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"
							/>
							<polyline points="22,6 12,13 2,6" />
						</svg>
					</div>

					<h2 class="brightness-150">projects</h2>
				</div>
			</div>
			{#each $projects as project, index}
				<div
					data-last={index === $projects.length - 1}
					class="flex w-full max-w-xl flex-col items-start justify-center gap-4 border-l-2 border-r-2 border-t-2 border-neutral bg-base-300 p-4 data-[last=true]:rounded-bl-md data-[last=true]:rounded-br-md data-[last=true]:border-b-2"
				>
					<p>{project.title}</p>
				</div>
			{/each}
		</div>
		<div class="flex w-full flex-col">
			<div
				class="rounded-tr-mid flex w-full max-w-xl flex-col items-start justify-center gap-4 rounded-tl-md border-l-2 border-r-2 border-t-2 border-neutral bg-base-300 p-4"
			>
				<div class="flex flex-row items-center gap-4">
					<div class="flex items-center justify-center rounded bg-base-100 p-2">
						<svg
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
							class="h-4 w-4 brightness-150"
						>
							<circle cx="12" cy="12" r="10" />
							<line x1="15" y1="9" x2="9" y2="15" />
							<line x1="9" y1="9" x2="15" y2="15" />
						</svg>
					</div>

					<h2 class="brightness-150">danger zone</h2>
				</div>
			</div>
			<div
				class="flex w-full max-w-xl flex-col items-start justify-center gap-4 rounded-bl-md rounded-br-md border-2 border-neutral bg-base-300 p-4"
			>
				<button
					on:click={() => modalRef.showModal()}
					class="btn btn-outline btn-error btn-sm"
				>
					delete account</button
				>
				{#if error}
					<p class="text-error">{error}</p>
				{/if}
			</div>
		</div>
	</div>
{:else}
	<div class="flex h-full w-full flex-col items-center justify-center">
		<h1 class="text-3xl font-bold">No user found</h1>
	</div>
{/if}

<dialog bind:this={modalRef} class="modal modal-bottom sm:modal-middle">
	<form class="modal-box" on:submit|preventDefault={deleteUserAndRedirect}>
		<h3 class="text-lg font-bold">Delete account</h3>
		<p class="py-2">Are you sure you want to delete this channel?</p>
		<p class="pb-4">
			To confirm, please type <span class="text-error">{user?.email} </span> in the field below.
		</p>
		<input
			name="title"
			type="text"
			placeholder="Project name"
			class="input input-bordered w-full"
			bind:value={userMail}
		/>
		{#if error}
			<p class="text-error">{error}</p>
		{/if}
		<div class="modal-action justify-start">
			<button disabled={userMail !== user?.email} class="btn btn-error" type="submit"
				>Delete</button
			>
			<form method="dialog">
				<button class="btn btn-outline">Close</button>
			</form>
		</div>
	</form>
</dialog>
