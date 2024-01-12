<script lang="ts">
	import { page } from '$app/stores';
	import { readChannelListForProject } from '$lib/api/channel';
	import { readProjectList } from '$lib/api/project';
	import SettingsSidebar from '$lib/components/sidebar/SettingsSidebar.svelte';
	import Sidebar from '$lib/components/sidebar/Sidebar.svelte';
	import SidebarMenu from '$lib/components/sidebar/SidebarMenu.svelte';
	import Loading from '$lib/layout/loading/Loading.svelte';
	import { channel, channels } from '$lib/stores/channel';
	import { project, projects } from '$lib/stores/project';
	import { onMount } from 'svelte';

	let loading = true;

	async function readProjects() {
		const projectRes = await readProjectList();
		if (projectRes.status === 200) {
			const newProjects = await projectRes.json();
			projects.set(newProjects);
			if (newProjects.length !== 0) {
				project.set(newProjects[0]);
			}
		} else {
			projects.set([]);
		}
	}

	async function readChannels(projectId: number) {
		const channelRes = await readChannelListForProject(projectId);
		if (channelRes.status === 200) {
			const newChannels = await channelRes.json();
			channels.set(newChannels);
			if (newChannels.length !== 0) {
				channel.set(newChannels[0]);
			}
		} else {
			channels.set([]);
		}
	}

	onMount(async () => {
		try {
			await readProjects();
			if ($project) {
				await readChannels($project.id);
			}
		} catch (e) {
			console.error(e);
		} finally {
			loading = false;
		}
	});

	$: if ($project) {
		readChannels($project.id);
	}

	$: settingsActive = $page.url.pathname.includes('/settings');
</script>

{#if loading}
	<Loading />
{:else}
	<div
		class="grid h-screen grid-rows-[auto_200px_1fr] md:grid-cols-[125px_250px_6fr] md:grid-rows-none"
	>
		<div class="overflow-auto p-4 md:border-r md:border-r-base-content/10">
			<Sidebar />
		</div>
		{#if settingsActive}
			<div class="overflow-auto border-r border-r-base-content/10 p-4">
				<SettingsSidebar />
			</div>
		{:else}
			<div class="overflow-auto border-r border-r-base-content/10 p-4">
				<SidebarMenu />
			</div>
		{/if}
		<div class="flex flex-col overflow-hidden">
			<slot />
		</div>
	</div>
{/if}
