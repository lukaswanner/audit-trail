<script lang="ts">
	import Sidebar from '$lib/components/sidebar/Sidebar.svelte';
	import SidebarMenu from '$lib/components/sidebar/SidebarMenu.svelte';

	let menuItems = ['ma', 'qu', 'ch', 'gi'];
	let channels = ['signups', 'general', 'chat'];
</script>

<div class="grid h-screen grid-cols-[1fr_3fr_6fr]">
	<div class="border-r border-r-base-content/10 p-4">
		<Sidebar {menuItems} />
	</div>
	<div class="border-r border-r-base-content/10 p-4">
		<SidebarMenu {channels} />
	</div>
	<div class="p-4">
		<slot />
	</div>
</div>
