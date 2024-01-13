<script lang="ts">
	import { project, projects } from '$lib/stores/project';
	import { updateProject, readProjectList } from '$lib/api/project';
	import type { Project, UpdateProjectPayload } from '$lib/types/project/ProjectTypes';

	let projectTitle = $project?.title;
	let error: string;

	async function handleTitleChange(e: Event) {
		const form = e.target as HTMLFormElement;
		const formData = new FormData(form);
		const data = Object.fromEntries(formData.entries());

		if (!$project) {
			error = 'Select a project first';
			return;
		}

		if (!data.title || data.title === '') {
			error = 'Project name is required';
			return;
		}

		if ($projects.find((el) => el.title === data.title)) {
			error = 'Project name already assigned';
			return;
		}

		const res = await updateProject({ ...data, id: $project.id } as UpdateProjectPayload);
		if (res.status === 200) {
			error = '';
			let res = await readProjectList();
			if (res.status === 200) {
				try {
					const updatedProjects = await res.json();
					projects.set(updatedProjects);
					project.set(updatedProjects.find((el: Project) => el.id === $project!.id));
				} catch (e) {
					console.error(e);
				}
			}
		} else if (res.status === 409) {
			error = 'Project already exists';
		} else {
			error = 'Something went wrong';
		}
	}

	function updateTitle() {
		projectTitle = $project?.title;
		error = '';
	}

	$: $project, updateTitle();
</script>

<div class="flex flex-row justify-between border-b border-b-base-content/10 p-4">
	<h1 class="text-3xl font-bold brightness-150">project settings</h1>
</div>

<div class="flex flex-col items-start gap-4 p-4">
	<form
		class="flex w-full max-w-xl flex-col items-start justify-center gap-4 rounded-md border-2 border-neutral bg-base-300 p-4"
		on:submit|preventDefault={handleTitleChange}
	>
		<p>project name</p>
		<input
			type="text"
			name="title"
			class="input input-bordered w-full max-w-md"
			placeholder="project name"
			bind:value={projectTitle}
		/>
		{#if error}
			<p class="text-xs text-error">{error}</p>
		{/if}
		<button class="btn btn-primary w-24">save</button>
	</form>
</div>
