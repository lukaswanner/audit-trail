<script lang="ts">
	import { project } from "$lib/stores/project";
	import { page } from "$app/stores";
	import SidebarChannelList from "./SidebarChannelList.svelte";
	import SidebarMenuSelection from "./SidebarMenuSelection.svelte";
	import SidebarActorList from "./SidebarActorList.svelte";

	$: feedActive = $page.url.pathname === "/";
	$: searchActive = $page.url.pathname === "/search";
	$: insightsActive = $page.url.pathname === "/insights";
</script>

<div class="flex flex-col items-start gap-4">
	<div class="flex w-full flex-row items-center justify-between px-2 brightness-150">
		<h1 class="break-word text-2xl font-bold">
			{$project?.title || "No project selected"}
		</h1>
	</div>
	<SidebarMenuSelection {feedActive} {searchActive} {insightsActive} />
	<SidebarChannelList {feedActive} />
	<SidebarActorList />
</div>
