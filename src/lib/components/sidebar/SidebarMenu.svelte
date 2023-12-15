<script lang="ts">
	import { project } from '$lib/stores/project';
	import { channel, channels } from '$lib/stores/channel';
	import { page } from '$app/stores';
	import { createChannel, readChannelListForProject } from '$lib/api/channel';
	import type { ChannelPayload } from '$lib/types/channel/ChannelTypes';
	import SidebarChannelList from './SidebarChannelList.svelte';
	import SidebarMenuSelection from './SidebarMenuSelection.svelte';

	let modalRef: HTMLDialogElement;
	let configModalRef: HTMLDialogElement;
	let error: string;

	async function fetchChannels() {
		const channelRes = await readChannelListForProject($project.title);
		if (channelRes.status === 200) {
			const newChannels = await channelRes.json();
			channels.set(newChannels);
			channel.set(newChannels[newChannels.length - 1]);
		}
	}

	async function handleNewChannel(e: Event) {
		const form = e.target as HTMLFormElement;
		const formData = new FormData(form);
		const data = Object.fromEntries(formData.entries());

		const payload: ChannelPayload = { title: data.title as string, projectId: $project.id };
		const res = await createChannel(payload);
		if (res.status === 201) {
			await fetchChannels();
			modalRef.close();
		} else if (res.status === 409) {
			error = 'Channel already exists';
		} else {
			error = 'Something went wrong';
		}
	}

	$: feedActive = $page.url.pathname === '/';
	$: chartsActive = $page.url.pathname === '/charts';
	$: insightsActive = $page.url.pathname === '/insights';
</script>

<div class="flex flex-col items-start gap-4">
	<div class="flex w-full flex-row items-center justify-between px-2">
		<h1 class="text-2xl font-bold">
			{$project?.title || 'No project selected'}
		</h1>
		<button on:click={() => configModalRef.showModal()} disabled={!$project} class="btn btn-ghost">
			<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 fill-current" viewBox="0 0 512 512"
				><!--!Font Awesome Free 6.5.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2023 Fonticons, Inc.--><path
					d="M495.9 166.6c3.2 8.7 .5 18.4-6.4 24.6l-43.3 39.4c1.1 8.3 1.7 16.8 1.7 25.4s-.6 17.1-1.7 25.4l43.3 39.4c6.9 6.2 9.6 15.9 6.4 24.6c-4.4 11.9-9.7 23.3-15.8 34.3l-4.7 8.1c-6.6 11-14 21.4-22.1 31.2c-5.9 7.2-15.7 9.6-24.5 6.8l-55.7-17.7c-13.4 10.3-28.2 18.9-44 25.4l-12.5 57.1c-2 9.1-9 16.3-18.2 17.8c-13.8 2.3-28 3.5-42.5 3.5s-28.7-1.2-42.5-3.5c-9.2-1.5-16.2-8.7-18.2-17.8l-12.5-57.1c-15.8-6.5-30.6-15.1-44-25.4L83.1 425.9c-8.8 2.8-18.6 .3-24.5-6.8c-8.1-9.8-15.5-20.2-22.1-31.2l-4.7-8.1c-6.1-11-11.4-22.4-15.8-34.3c-3.2-8.7-.5-18.4 6.4-24.6l43.3-39.4C64.6 273.1 64 264.6 64 256s.6-17.1 1.7-25.4L22.4 191.2c-6.9-6.2-9.6-15.9-6.4-24.6c4.4-11.9 9.7-23.3 15.8-34.3l4.7-8.1c6.6-11 14-21.4 22.1-31.2c5.9-7.2 15.7-9.6 24.5-6.8l55.7 17.7c13.4-10.3 28.2-18.9 44-25.4l12.5-57.1c2-9.1 9-16.3 18.2-17.8C227.3 1.2 241.5 0 256 0s28.7 1.2 42.5 3.5c9.2 1.5 16.2 8.7 18.2 17.8l12.5 57.1c15.8 6.5 30.6 15.1 44 25.4l55.7-17.7c8.8-2.8 18.6-.3 24.5 6.8c8.1 9.8 15.5 20.2 22.1 31.2l4.7 8.1c6.1 11 11.4 22.4 15.8 34.3zM256 336a80 80 0 1 0 0-160 80 80 0 1 0 0 160z"
				/></svg
			>
		</button>
	</div>
	<div class="flex w-full flex-row items-center justify-between px-4">
		<h2 class="text-xl font-bold">Channels</h2>
		<button
			disabled={!$project}
			on:click={() => modalRef.showModal()}
			class="btn btn-ghost rounded-full fill-base-content transition-colors hover:fill-primary"
		>
			<svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 448 512"
				><!--!Font Awesome Free 6.5.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2023 Fonticons, Inc.--><path
					d="M256 80c0-17.7-14.3-32-32-32s-32 14.3-32 32V224H48c-17.7 0-32 14.3-32 32s14.3 32 32 32H192V432c0 17.7 14.3 32 32 32s32-14.3 32-32V288H400c17.7 0 32-14.3 32-32s-14.3-32-32-32H256V80z"
				/></svg
			>
		</button>
	</div>
	<SidebarMenuSelection {feedActive} {chartsActive} {insightsActive} />
	<SidebarChannelList {feedActive} {chartsActive} {insightsActive} />
</div>
<dialog id="channel-modal" bind:this={modalRef} class="modal modal-bottom sm:modal-middle">
	<form class="modal-box" on:submit|preventDefault={handleNewChannel}>
		<h3 class="text-lg font-bold">Create new channel</h3>
		<p class="py-4">Create a new channel to start sending messages to it.</p>
		<input
			name="title"
			type="text"
			placeholder="Channel name"
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
<dialog
	id="project-config-modal"
	bind:this={configModalRef}
	class="modal modal-bottom sm:modal-middle"
>
	<div class="modal-box">
		<h3 class="text-lg font-bold">{$project.title}</h3>
		<p class="py-4">Manage your project.</p>
		<div class="modal-action">
			<button class="btn btn-error">Delete</button>
			<form method="dialog">
				<button class="btn btn-outline">Close</button>
			</form>
		</div>
	</div>
</dialog>
