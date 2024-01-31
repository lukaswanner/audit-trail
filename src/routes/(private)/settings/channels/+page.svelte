<script lang="ts">
	import { readChannelListForProject } from "$lib/api/channel";
	import ListItem from "$lib/components/settings/ListItem.svelte";
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
				<line x1="4" y1="9" x2="20" y2="9" />
				<line x1="4" y1="15" x2="20" y2="15" />
				<line x1="10" y1="3" x2="8" y2="21" />
				<line x1="16" y1="3" x2="14" y2="21" />
			</svg>
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
			<ListItem name={channel.title} hrefLink={`/settings/channels/${channel.id}`} />
		</div>
	{/each}
</div>
