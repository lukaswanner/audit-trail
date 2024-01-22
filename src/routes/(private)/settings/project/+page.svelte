<script lang="ts">
	import { project, projects } from "$lib/stores/project";
	import { updateProject, readProjectList } from "$lib/api/project";
	import type { Project, UpdateProjectPayload } from "$lib/types/project/ProjectTypes";

	let projectTitle = $project?.title;
	let error: string;

	async function handleTitleChange(e: Event) {
		const form = e.target as HTMLFormElement;
		const formData = new FormData(form);
		const data = Object.fromEntries(formData.entries());

		if (!$project) {
			error = "Select a project first";
			return;
		}

		if (!data.title || data.title === "") {
			error = "Project name is required";
			return;
		}

		if ($projects.find((el) => el.title === data.title)) {
			error = "Project name already assigned";
			return;
		}

		const res = await updateProject({ ...data, id: $project.id } as UpdateProjectPayload);
		if (res.status === 200) {
			error = "";
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
			error = "Project already exists";
		} else {
			error = "Something went wrong";
		}
	}

	function updateTitle() {
		projectTitle = $project?.title;
		error = "";
	}

	$: $project, updateTitle();
</script>

<div class="flex flex-row justify-between border-b border-b-base-content/10 p-4">
	<h1 class="text-3xl font-bold brightness-150">project settings</h1>
</div>

<div class="flex w-full max-w-2xl flex-col items-start p-4">
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
			>
				<path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" />
			</svg>
		</div>
		<h1 class="text-2xl brightness-150">config for {$project?.title}</h1>
	</div>
	<div
		class="flex w-full flex-col items-start justify-center gap-4 rounded-bl-md rounded-br-md border-b-2 border-l-2 border-r-2 border-t-2 border-neutral bg-base-300 p-4"
	>
		<p class="text-base-content">change the name of your project</p>
	</div>
	<form
		class="mt-4 flex w-full max-w-2xl flex-col items-start justify-center gap-4 rounded-md border-2 border-neutral bg-base-300 p-4"
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
		<button
			disabled={projectTitle === "" || projectTitle === $project?.title}
			class="btn btn-primary w-24">update</button
		>
	</form>
</div>
