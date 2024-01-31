<script lang="ts">
	import { goto } from "$app/navigation";
	import { page } from "$app/stores";
	import { readChannelListForProject, updateChannel } from "$lib/api/channel";
	import { channels } from "$lib/stores/channel";
	import { project } from "$lib/stores/project";

	let error: string;
	let channelId = decodeURIComponent($page.url.pathname.split("/")[3]);
	let channelName = $channels.find((channel) => channel.id === Number(channelId))?.title;

	let inputName = "";

	async function readChannelList() {
		let res = await readChannelListForProject($project!.id);
		if (res.status === 200) {
			try {
				const updatedChannels = await res.json();
				channels.set(updatedChannels);
			} catch (e) {
				console.error(e);
			}
		}
		error = "";
	}

	async function handleNameChange() {
		// make sure we have the latest actor list
		await readChannelList();
		const currentChannel = $channels.find((channel) => channel.title === channelName);
		if (!$project || !$project.id) {
			error = "no project selected";
			return;
		}
		if (!currentChannel) {
			error = "channel not found";
			return;
		}
		const payload = {
			id: currentChannel.id,
			title: inputName,
			projectId: $project.id
		};

		const res = await updateChannel(payload);
		if (res.status === 200) {
			await readChannelList();
			goto("/settings/channels");
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
	<h1 class="text-3xl font-bold brightness-150">update #{channelName}</h1>
</div>

<div class="w-full max-w-2xl p-4">
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
				<line x1="4" y1="9" x2="20" y2="9" />
				<line x1="4" y1="15" x2="20" y2="15" />
				<line x1="10" y1="3" x2="8" y2="21" />
				<line x1="16" y1="3" x2="14" y2="21" />
			</svg>
		</div>
		<h1 class="text-2xl">
			<span class="brightness-150"> settings for </span>
			{#if channelName}
				<span class="brightness-150">
					#{channelName}
				</span>
			{:else}
				<span class="text-error">channel not found</span>
			{/if}
		</h1>
	</div>
	<div
		class="flex w-full flex-col items-start justify-center gap-4 rounded-bl-md rounded-br-md border-b-2 border-l-2 border-r-2 border-t-2 border-neutral bg-base-300 p-4"
	>
		<p class="text-base-content">change the name of this channel</p>
	</div>
</div>
<div class="flex w-full max-w-2xl flex-col items-start gap-4 p-4">
	<form
		class="flex w-full flex-col items-start justify-center gap-4 rounded-md border-2 border-neutral bg-base-300 p-4"
		on:submit|preventDefault={handleNameChange}
	>
		channel name
		<input
			type="text"
			name="name"
			class="input input-bordered w-full max-w-md"
			placeholder="channel title"
			bind:value={inputName}
		/>
		{#if error}
			<p class="text-xs text-error">{error}</p>
		{/if}
		<button disabled={inputName.trim() === ""} class="btn btn-primary w-24">update</button>
	</form>
</div>
