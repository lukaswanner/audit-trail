<script lang="ts">
	import { goto } from '$app/navigation';
	import { createProject, readProjectList } from '$lib/api/project';
	import { project, projects } from '$lib/stores/project';
	import type { ProjectPayload } from '$lib/types/project/ProjectTypes';

	let modalRef: HTMLDialogElement;
	let error: string;

	async function fetchProjects() {
		const projectRes = await readProjectList();
		if (projectRes.status === 200) {
			const newProjects = await projectRes.json();
			projects.set(newProjects);
			project.set(newProjects[newProjects.length - 1]);
		}
	}

	async function handleNewProject(e: Event) {
		const form = e.target as HTMLFormElement;
		const formData = new FormData(form);
		const data = Object.fromEntries(formData.entries());

		const res = await createProject(data as ProjectPayload);
		if (res.status === 201) {
			await fetchProjects();
			modalRef.close();
		} else if (res.status === 409) {
			error = 'Project already exists';
		} else {
			error = 'Something went wrong';
		}
	}
</script>

<div class="flex items-center gap-2 md:flex-col">
	<button
		on:click={() => goto('/')}
		class="btn btn-ghost h-16 w-16 rounded-full fill-base-content brightness-150 transition-colors hover:fill-primary"
	>
		<svg xmlns="http://www.w3.org/2000/svg" class=" h-8 w-8" viewBox="0 0 576 512"
			><!--!Font Awesome Free 6.5.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2023 Fonticons, Inc.--><path
				d="M575.8 255.5c0 18-15 32.1-32 32.1h-32l.7 160.2c0 2.7-.2 5.4-.5 8.1V472c0 22.1-17.9 40-40 40H456c-1.1 0-2.2 0-3.3-.1c-1.4 .1-2.8 .1-4.2 .1H416 392c-22.1 0-40-17.9-40-40V448 384c0-17.7-14.3-32-32-32H256c-17.7 0-32 14.3-32 32v64 24c0 22.1-17.9 40-40 40H160 128.1c-1.5 0-3-.1-4.5-.2c-1.2 .1-2.4 .2-3.6 .2H104c-22.1 0-40-17.9-40-40V360c0-.9 0-1.9 .1-2.8V287.6H32c-18 0-32-14-32-32.1c0-9 3-17 10-24L266.4 8c7-7 15-8 22-8s15 2 21 7L564.8 231.5c8 7 12 15 11 24z"
			/></svg
		>
	</button>
	<div class="flex flex-row items-center gap-2 md:flex-col">
		{#each $projects as project_item}
			<button
				on:click={() => project.set(project_item)}
				data-active={project_item === $project}
				class="btn btn-circle flex h-12 w-12 items-center justify-center overflow-hidden rounded-full border-transparent bg-transparent hover:border-transparent hover:bg-transparent data-[active=true]:border-2
		data-[active=true]:border-accent"
			>
				<p class="text-ellipsis text-lg font-bold brightness-150">
					{project_item.title.slice(0, 2)}
				</p>
			</button>
		{/each}
	</div>
	<div class="my-0 hidden w-1/2 !self-center md:divider" />
	<button
		class="btn btn-ghost ml-auto h-12 w-12 rounded-full fill-base-content brightness-150 transition-colors hover:fill-primary md:mr-auto"
		on:click={() => modalRef.showModal()}
	>
		<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" viewBox="0 0 448 512"
			><!--!Font Awesome Free 6.5.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2023 Fonticons, Inc.--><path
				d="M256 80c0-17.7-14.3-32-32-32s-32 14.3-32 32V224H48c-17.7 0-32 14.3-32 32s14.3 32 32 32H192V432c0 17.7 14.3 32 32 32s32-14.3 32-32V288H400c17.7 0 32-14.3 32-32s-14.3-32-32-32H256V80z"
			/></svg
		>
	</button>
	<!-- Open the modal using ID.showModal() method -->
</div>
<dialog id="channel-id" bind:this={modalRef} class="modal modal-bottom sm:modal-middle">
	<form class="modal-box" on:submit|preventDefault={handleNewProject}>
		<h3 class="text-lg font-bold">Create new project</h3>
		<p class="py-4">Create a new project to start working on it.</p>
		<input
			name="title"
			type="text"
			placeholder="Project name"
			class="input input-bordered w-full"
		/>
		{#if error}
			<p class="text-error">{error}</p>
		{/if}
		<div class="modal-action">
			<button class="btn btn-primary" type="submit">Create</button>
			<form method="dialog">
				<button class="btn btn-outline">Close</button>
			</form>
		</div>
	</form>
</dialog>
