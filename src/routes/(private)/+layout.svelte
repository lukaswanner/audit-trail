<script lang="ts">
	import Sidebar from '$lib/components/sidebar/Sidebar.svelte';
	import SidebarMenu from '$lib/components/sidebar/SidebarMenu.svelte';
	import type { LayoutData } from '../$types';

	export let data: LayoutData;
	let { projects, channels } = data;
</script>

<div class="grid h-screen grid-cols-[100px_250px_6fr]">
	<div class="border-r border-r-base-content/10 p-4">
		<Sidebar {projects} />
	</div>
	<div class="border-r border-r-base-content/10 p-4">
		<SidebarMenu {channels} />
	</div>
	<div>
		<slot />
	</div>
</div>
