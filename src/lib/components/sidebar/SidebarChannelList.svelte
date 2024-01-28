<script lang="ts">
	import { createChannel, readChannelListForProject } from "$lib/api/channel";
	import { project } from "$lib/stores/project";
	import { channel, channels } from "$lib/stores/channel";
	import type { ChannelPayload } from "$lib/types/channel/ChannelTypes";
	import { goto } from "$app/navigation";

	export let feedActive: boolean;

	let channelTitle = "";
	let modalRef: HTMLDialogElement;
	let error: string;

	async function fetchChannels() {
		const channelRes = await readChannelListForProject($project!.id);
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

		if (!$project) {
			error = "Select a project first";
			return;
		}
		const payload: ChannelPayload = { title: data.title as string, projectId: $project.id };
		const res = await createChannel(payload);
		if (res.status === 201) {
			await fetchChannels();
			modalRef.close();
		} else if (res.status === 409) {
			error = "Channel already exists";
		} else {
			error = "Something went wrong";
		}
	}
</script>

<div class="flex w-full flex-row items-center justify-between px-4">
	<h2 class="text-xl font-bold brightness-150">Channels</h2>
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
{#each $channels as channel_item}
	<button
		on:click={() => {
			channel.set(channel_item);
			goto("/");
		}}
		data-active={feedActive && channel_item === $channel}
		data-feed={channel_item === $channel && feedActive}
		class="btn btn-ghost flex w-full flex-grow flex-row flex-nowrap items-center justify-start gap-2 data-[active=true]:bg-base-content/20 data-[feed=true]:text-primary"
	>
		<svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 fill-current" viewBox="0 0 448 512"
			><!--!Font Awesome Free 6.5.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2023 Fonticons, Inc.--><path
				d="M181.3 32.4c17.4 2.9 29.2 19.4 26.3 36.8L197.8 128h95.1l11.5-69.3c2.9-17.4 19.4-29.2 36.8-26.3s29.2 19.4 26.3 36.8L357.8 128H416c17.7 0 32 14.3 32 32s-14.3 32-32 32H347.1L325.8 320H384c17.7 0 32 14.3 32 32s-14.3 32-32 32H315.1l-11.5 69.3c-2.9 17.4-19.4 29.2-36.8 26.3s-29.2-19.4-26.3-36.8l9.8-58.7H155.1l-11.5 69.3c-2.9 17.4-19.4 29.2-36.8 26.3s-29.2-19.4-26.3-36.8L90.2 384H32c-17.7 0-32-14.3-32-32s14.3-32 32-32h68.9l21.3-128H64c-17.7 0-32-14.3-32-32s14.3-32 32-32h68.9l11.5-69.3c2.9-17.4 19.4-29.2 36.8-26.3zM187.1 192L165.8 320h95.1l21.3-128H187.1z"
			/></svg
		>
		<p class="h-12 overflow-hidden text-ellipsis whitespace-nowrap font-bold leading-[3rem]">
			{channel_item.title}
		</p>
	</button>
{/each}

<dialog id="channel-modal" bind:this={modalRef} class="modal modal-bottom sm:modal-middle">
	<form class="modal-box" on:submit|preventDefault={handleNewChannel}>
		<h3 class="text-lg font-bold">Create new channel</h3>
		<p class="py-4">Create a new channel to start sending messages to it.</p>
		<input
			name="title"
			type="text"
			placeholder="Channel name"
			class="input input-bordered w-full"
			bind:value={channelTitle}
		/>
		{#if error}
			<p class="text-error">{error}</p>
		{/if}
		<div class="modal-action justify-start">
			<button disabled={channelTitle.length === 0} class="btn btn-primary" type="submit"
				>Create</button
			>
			<form method="dialog">
				<button class="btn btn-outline">Close</button>
			</form>
		</div>
	</form>
</dialog>
