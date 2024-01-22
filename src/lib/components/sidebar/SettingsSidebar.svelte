<script lang="ts">
	import { goto } from "$app/navigation";
	import { page } from "$app/stores";
	import { logout } from "$lib/api/auth";
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
		>
			<path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" />
		</svg>
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
		>
			<line x1="4" y1="9" x2="20" y2="9" />
			<line x1="4" y1="15" x2="20" y2="15" />
			<line x1="10" y1="3" x2="8" y2="21" />
			<line x1="16" y1="3" x2="14" y2="21" />
		</svg>
		<p>channels</p>
	</a>
	<a
		href="/settings/actors"
		data-active={$page.url.pathname.includes("/settings/actors")}
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
	<a
		href="/settings/api"
		data-active={$page.url.pathname.includes("/settings/api")}
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
		>
			<polygon points="12 2 2 7 12 12 22 7 12 2" />
			<polyline points="2 17 12 22 22 17" />
			<polyline points="2 12 12 17 22 12" />
		</svg>
		<p>api</p>
	</a>

	<h2 class="text-lg brightness-150">profile settings</h2>
	<a
		href="/settings/profile"
		data-active={$page.url.pathname === "/settings/profile"}
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
		>
			<path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" />
			<circle cx="12" cy="7" r="4" />
		</svg>

		<p>profile</p>
	</a>

	<h2 class="text-lg brightness-150">danger zone</h2>
	<button
		on:click={async () => {
			const res = await logout();
			if (res.status === 200) {
				goto("/login");
			}
		}}
		class="btn btn-ghost flex w-full flex-row items-center justify-start gap-2 text-base-content transition-colors"
	>
		<svg
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			stroke-linecap="round"
			stroke-linejoin="round"
			class="h-4 w-4"
		>
			<path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" />
			<polyline points="16 17 21 12 16 7" />
			<line x1="21" y1="12" x2="9" y2="12" />
		</svg>

		<p>logout</p>
	</button>
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
		>
			<circle cx="12" cy="12" r="10" />
			<line x1="15" y1="9" x2="9" y2="15" />
			<line x1="9" y1="9" x2="15" y2="15" />
		</svg>
		<p>delete project</p>
	</a>
</div>
