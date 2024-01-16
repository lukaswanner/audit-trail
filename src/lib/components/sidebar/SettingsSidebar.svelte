<script lang="ts">
	import { page } from "$app/stores";
</script>

<h1 class="break-word h-16 text-2xl font-bold brightness-150">settings</h1>
<div class="flex flex-col items-start gap-4">
	<h2 class="text-lg brightness-150">project settings</h2>
	<a
		href="/settings/project"
		data-active={$page.url.pathname === "/settings/project"}
		class="btn btn-ghost flex w-full flex-row items-center justify-start gap-2 text-base-content transition-colors data-[active=true]:bg-base-content/20 data-[active=true]:text-accent"
	>
		<svg
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			stroke-linecap="round"
			stroke-linejoin="round"
			class="h-4 w-4"
			><path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"></path></svg
		>
		<p>project</p>
	</a>
	<a
		href="/settings/channels"
		data-active={$page.url.pathname === "/settings/channels"}
		class="btn btn-ghost flex w-full flex-row items-center justify-start gap-2 text-base-content transition-colors data-[active=true]:bg-base-content/20 data-[active=true]:text-accent"
	>
		<svg
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			stroke-linecap="round"
			stroke-linejoin="round"
			class="h-4 w-4"
			><path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"></path></svg
		>
		<p>channels</p>
	</a>
	<a
		href="/settings/actors"
		data-active={$page.url.pathname === "/settings/actors"}
		class="btn btn-ghost flex w-full flex-row items-center justify-start gap-2 text-base-content transition-colors data-[active=true]:bg-base-content/20 data-[active=true]:text-accent"
	>
		<svg
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			stroke-linecap="round"
			stroke-linejoin="round"
			class="h-4 w-4"
			><circle cx="12" cy="12" r="4"></circle><path
				d="M16 8v5a3 3 0 0 0 6 0v-1a10 10 0 1 0-3.92 7.94"
			></path></svg
		>
		<p>actors</p>
	</a>
	<h2 class="text-lg brightness-150">danger zone</h2>
	<a
		href="/settings/delete"
		data-active={$page.url.pathname === "/settings/delete"}
		class="btn btn-ghost flex w-full flex-row items-center justify-start gap-2 text-base-content transition-colors data-[active=true]:bg-base-content/20 data-[active=true]:text-accent"
	>
		<svg
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			stroke-linecap="round"
			stroke-linejoin="round"
			class="h-4 w-4"
			><circle cx="12" cy="12" r="10"></circle><line x1="15" y1="9" x2="9" y2="15"
			></line><line x1="9" y1="9" x2="15" y2="15"></line></svg
		>
		<p>delete project</p>
	</a>
</div>
