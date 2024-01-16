<script lang="ts">
	import { goto } from "$app/navigation";
	import { page } from "$app/stores";
	import { createProject, readProjectList } from "$lib/api/project";
	import { actors } from "$lib/stores/actor";
	import { channels } from "$lib/stores/channel";
	import { events } from "$lib/stores/event";
	import { insights } from "$lib/stores/insight";
	import { project, projects } from "$lib/stores/project";
	import type { Project, ProjectPayload } from "$lib/types/project/ProjectTypes";

	let projectTitle = "";
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

		if (!data.title || data.title === "") {
			error = "Project name is required";
			return;
		}

		if ($projects.find((el) => el.title === data.title)) {
			error = "Project name already assigned";
			return;
		}

		const res = await createProject(data as ProjectPayload);
		if (res.status === 201) {
			await fetchProjects();
			modalRef.close();
		} else if (res.status === 409) {
			error = "Project already exists";
		} else {
			error = "Something went wrong";
		}
	}

	function handleProjectSwitch(newProject: Project) {
		if (newProject === $project) {
			goto("/");
			return;
		}
		project.set(newProject);
		channels.set([]);
		actors.set([]);
		events.set([]);
		insights.set([]);
	}

	$: settingsActive = $page.url.pathname === "/settings";
</script>

<div class="flex h-full items-center gap-2 md:flex-col">
	<button
		on:click={() => goto("/")}
		class="btn btn-ghost h-14 w-14 rounded-full fill-base-content brightness-150 transition-colors hover:fill-primary hover:brightness-100"
	>
		<svg xmlns="http://www.w3.org/2000/svg" class="h-full w-full" viewBox="0 0 576 512"
			><!--!Font Awesome Free 6.5.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2023 Fonticons, Inc.--><path
				d="M575.8 255.5c0 18-15 32.1-32 32.1h-32l.7 160.2c0 2.7-.2 5.4-.5 8.1V472c0 22.1-17.9 40-40 40H456c-1.1 0-2.2 0-3.3-.1c-1.4 .1-2.8 .1-4.2 .1H416 392c-22.1 0-40-17.9-40-40V448 384c0-17.7-14.3-32-32-32H256c-17.7 0-32 14.3-32 32v64 24c0 22.1-17.9 40-40 40H160 128.1c-1.5 0-3-.1-4.5-.2c-1.2 .1-2.4 .2-3.6 .2H104c-22.1 0-40-17.9-40-40V360c0-.9 0-1.9 .1-2.8V287.6H32c-18 0-32-14-32-32.1c0-9 3-17 10-24L266.4 8c7-7 15-8 22-8s15 2 21 7L564.8 231.5c8 7 12 15 11 24z"
			/></svg
		>
	</button>
	<div class="flex flex-row items-center gap-2 md:flex-col">
		{#each $projects as project_item}
			<button
				on:click={() => handleProjectSwitch(project_item)}
				data-active={project_item === $project}
				class="btn btn-circle flex h-12 w-12 items-center justify-center overflow-hidden rounded-full border-transparent bg-transparent hover:border-transparent hover:bg-transparent hover:text-accent
		data-[active=true]:border-2 data-[active=true]:border-accent"
			>
				<p class="text-ellipsis text-lg font-bold brightness-150">
					{project_item.title.slice(0, 2)}
				</p>
			</button>
		{/each}
	</div>
	<div class="my-0 hidden w-1/2 !self-center md:divider" />
	<button
		class="btn btn-ghost ml-auto h-12 w-12 rounded-full fill-base-content brightness-150 transition-colors hover:fill-primary hover:brightness-100 md:mr-auto"
		on:click={() => modalRef.showModal()}
	>
		<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" viewBox="0 0 448 512"
			><!--!Font Awesome Free 6.5.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2023 Fonticons, Inc.--><path
				d="M256 80c0-17.7-14.3-32-32-32s-32 14.3-32 32V224H48c-17.7 0-32 14.3-32 32s14.3 32 32 32H192V432c0 17.7 14.3 32 32 32s32-14.3 32-32V288H400c17.7 0 32-14.3 32-32s-14.3-32-32-32H256V80z"
			/></svg
		>
	</button>
	<a data-active={settingsActive} class="mt-auto hover:text-primary" href="/settings/project">
		<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 fill-current" viewBox="0 0 512 512"
			><!--!Font Awesome Free 6.5.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2023 Fonticons, Inc.--><path
				d="M495.9 166.6c3.2 8.7 .5 18.4-6.4 24.6l-43.3 39.4c1.1 8.3 1.7 16.8 1.7 25.4s-.6 17.1-1.7 25.4l43.3 39.4c6.9 6.2 9.6 15.9 6.4 24.6c-4.4 11.9-9.7 23.3-15.8 34.3l-4.7 8.1c-6.6 11-14 21.4-22.1 31.2c-5.9 7.2-15.7 9.6-24.5 6.8l-55.7-17.7c-13.4 10.3-28.2 18.9-44 25.4l-12.5 57.1c-2 9.1-9 16.3-18.2 17.8c-13.8 2.3-28 3.5-42.5 3.5s-28.7-1.2-42.5-3.5c-9.2-1.5-16.2-8.7-18.2-17.8l-12.5-57.1c-15.8-6.5-30.6-15.1-44-25.4L83.1 425.9c-8.8 2.8-18.6 .3-24.5-6.8c-8.1-9.8-15.5-20.2-22.1-31.2l-4.7-8.1c-6.1-11-11.4-22.4-15.8-34.3c-3.2-8.7-.5-18.4 6.4-24.6l43.3-39.4C64.6 273.1 64 264.6 64 256s.6-17.1 1.7-25.4L22.4 191.2c-6.9-6.2-9.6-15.9-6.4-24.6c4.4-11.9 9.7-23.3 15.8-34.3l4.7-8.1c6.6-11 14-21.4 22.1-31.2c5.9-7.2 15.7-9.6 24.5-6.8l55.7 17.7c13.4-10.3 28.2-18.9 44-25.4l12.5-57.1c2-9.1 9-16.3 18.2-17.8C227.3 1.2 241.5 0 256 0s28.7 1.2 42.5 3.5c9.2 1.5 16.2 8.7 18.2 17.8l12.5 57.1c15.8 6.5 30.6 15.1 44 25.4l55.7-17.7c8.8-2.8 18.6-.3 24.5 6.8c8.1 9.8 15.5 20.2 22.1 31.2l4.7 8.1c6.1 11 11.4 22.4 15.8 34.3zM256 336a80 80 0 1 0 0-160 80 80 0 1 0 0 160z"
			/></svg
		>
	</a>

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
			bind:value={projectTitle}
		/>
		{#if error}
			<p class="text-error">{error}</p>
		{/if}
		<div class="modal-action justify-start">
			<button disabled={projectTitle.length === 0} class="btn btn-success" type="submit"
				>Create</button
			>
			<form method="dialog">
				<button class="btn btn-outline">Close</button>
			</form>
		</div>
	</form>
</dialog>
