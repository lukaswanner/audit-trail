<script lang="ts">
	import { deleteProject, readProjectList } from "$lib/api/project";
	import { project, projects } from "$lib/stores/project";

	let error: string;
	let projectTitle = $project?.title;
	let inputTitle = "";

	async function handleDelete() {
		if (!$project) {
			return;
		}
		const res = await deleteProject($project.id);
		if (res.status === 204) {
			let res = await readProjectList();
			if (res.status === 200) {
				try {
					const updatedProjects = await res.json();
					projects.set(updatedProjects);
					if (updatedProjects.length !== 0) {
						project.set(updatedProjects[0]);
					} else {
						project.set(null);
					}
				} catch (e) {
					console.error(e);
				}
			}
		}
	}

	function updateTitle() {
		projectTitle = $project?.title;
		error = "";
	}

	$: $project, updateTitle();
</script>

<div class="flex flex-row justify-between border-b border-b-base-content/10 p-4">
	<h1 class="text-3xl font-bold brightness-150">delete project</h1>
</div>

<div class="flex flex-col items-start gap-4 p-4">
	<form
		class="flex w-full max-w-md flex-col items-start justify-center gap-4 rounded-md border-2 border-neutral bg-base-300 p-4"
		on:submit|preventDefault={handleDelete}
	>
		{#if $project}
			<p>Are you sure you want to delete this project?</p>
			<p>
				To confirm, please type
				<span class="text-error">
					{$project.title}
				</span>
				in the field below.
			</p>
			<input
				type="text"
				name="title"
				disabled={!$project}
				class="input input-bordered w-full max-w-md"
				placeholder="project name"
				bind:value={inputTitle}
			/>
			{#if error}
				<p class="text-xs text-error">{error}</p>
			{/if}
			<button disabled={inputTitle.trim() !== projectTitle?.trim()} class="btn btn-error w-24"
				>delete</button
			>
		{/if}
		{#if !$project}
			<p class="text-md text-error">Please create a project first!</p>
		{/if}
	</form>
</div>
