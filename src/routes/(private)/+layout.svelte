<script lang="ts">
	import Sidebar from '$lib/components/sidebar/Sidebar.svelte';
	import SidebarMenu from '$lib/components/sidebar/SidebarMenu.svelte';

	let menuItems = ['ma', 'qu', 'ch', 'gi'];
	let channels = ['signups', 'general', 'chat'];
</script>

<div class="grid grid-cols-[1fr_3fr_6fr] py-4">
	<Sidebar {menuItems} />
	<SidebarMenu {channels} />
	<slot />
</div>
