<script lang="ts">
	import { goto } from "$app/navigation";
	import { page } from "$app/stores";
	import { deleteNotification } from "$lib/api/notification";
	import { project } from "$lib/stores/project";

	let error: string;
	let id = decodeURIComponent($page.url.pathname.split("/")[3]);

	let projectInput = "";

	async function handleDelete() {
		if (!id) {
			return;
		}
		const res = await deleteNotification(parseInt(id));
		if (res.status === 204) {
			goto("/settings/notifications");
		}
	}
</script>

<div class="flex flex-row items-center gap-4 border-b border-b-base-content/10 p-4">
	<a href="/settings/channels">
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
	<h1 class="text-3xl font-bold brightness-150">delete notification user</h1>
</div>

<div class="flex max-w-2xl flex-col items-start gap-4 p-4">
	<form
		class="flex w-full flex-col items-start justify-center gap-4 rounded-md border-2 border-neutral bg-base-300 p-4"
		on:submit|preventDefault={handleDelete}
	>
		<div>
			<p>Are you sure you want to delete this notification user?</p>
			<p>This user will not get any notifications from this project anymore</p>
		</div>
		<p>
			To confirm, please type
			<span class="text-error">
				{$project?.title}
			</span>
			in the field below.
		</p>
		<input
			type="text"
			name="title"
			class="input input-bordered w-full max-w-md"
			placeholder="project name"
			bind:value={projectInput}
		/>
		{#if error}
			<p class="text-xs text-error">{error}</p>
		{/if}
		<button
			disabled={projectInput.trim() !== $project?.title?.trim()}
			class="btn btn-error w-24">delete</button
		>
	</form>
</div>
