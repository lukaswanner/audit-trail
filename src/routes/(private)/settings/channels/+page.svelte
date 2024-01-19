<script lang="ts">
	import { readChannelListForProject } from "$lib/api/channel";
	import Loading from "$lib/layout/loading/Loading.svelte";
	import { channels } from "$lib/stores/channel";
	import { project } from "$lib/stores/project";
	import { onMount } from "svelte";

	let loading = true;

	async function updateChannelList() {
		loading = true;
		let res = await readChannelListForProject($project!.id);
		if (res.status === 200) {
			try {
				const updatedChannels = await res.json();
				channels.set(updatedChannels);
			} catch (e) {
				console.error(e);
			}
		}
		loading = false;
	}

	$: $project, updateChannelList();

	onMount(async () => {
		updateChannelList();
	});
</script>

<div class="flex flex-row justify-between border-b border-b-base-content/10 p-4">
	<h1 class="text-3xl font-bold brightness-150">channels</h1>
</div>

{#if loading}
	<div class="relative">
		<Loading />
	</div>
{/if}
<div class="flex max-w-xl flex-col items-start p-4">
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
				><circle cx="12" cy="12" r="4"></circle><path
					d="M16 8v5a3 3 0 0 0 6 0v-1a10 10 0 1 0-3.92 7.94"
				></path></svg
			>
		</div>
		<h1 class="text-2xl brightness-150">channels for {$project?.title}</h1>
	</div>
	{#if $channels.length === 0}
		<div
			class="flex w-full flex-col items-start justify-center gap-4 rounded-bl-md rounded-br-md border-b-2 border-l-2 border-r-2 border-t-2 border-neutral bg-base-300 p-4"
		>
			<p class="text-base-content">no channels yet created</p>
		</div>
	{/if}
	{#each $channels as channel, index}
		<div
			data-last={index === $channels.length - 1}
			class="flex w-full flex-col items-start justify-center gap-4 border-l-2 border-r-2 border-t-2 border-neutral bg-base-300 p-4 data-[last=true]:rounded-bl-md data-[last=true]:rounded-br-md data-[last=true]:border-b-2"
		>
			<div class="flex w-full flex-row items-center justify-between">
				<p class="text-base-content">{channel.title}</p>
				<div class="flex flex-row items-center gap-2">
					<a
						href={`/settings/channels/${channel.title}`}
						class="transition-colors hover:text-secondary"
					>
						<svg
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
							class="h-4 w-4"
							><path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"
							></path><path
								d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"
							></path></svg
						>
					</a>
					<a
						href={`/settings/channels/${channel.title}/danger`}
						class="transition-colors hover:text-secondary"
					>
						<svg
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
							class="h-4 w-4"
							><circle cx="12" cy="12" r="10"></circle><line
								x1="15"
								y1="9"
								x2="9"
								y2="15"
							></line><line x1="9" y1="9" x2="15" y2="15"></line></svg
						>
					</a>
				</div>
			</div>
		</div>
	{/each}
</div>
