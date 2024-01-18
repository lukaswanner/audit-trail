<script lang="ts">
	import { deleteApikey } from "$lib/api/apikey";
	import { apikeys } from "$lib/stores/apikey";
	import type { Apikey } from "$lib/types/apikey/ApikeyTypes";

	export let apikey: Apikey;

	async function handleDelete() {
		const res = await deleteApikey(apikey.id);
		if (res.status === 204) {
			apikeys.update((apikeys) => apikeys.filter((a) => a.id !== apikey.id));
		}
	}

	function handleCopy() {
		navigator.clipboard.writeText(apikey.token);
	}
</script>

<div class="flex w-full max-w-xl flex-col gap-2">
	<span>{apikey.projectTitle} - {new Date(apikey.createdAt).toLocaleDateString()}</span>
	<div
		class="min-h-16 flex flex-row flex-wrap items-center justify-between overflow-hidden rounded-xl border border-neutral pl-4 pr-2 transition-colors hover:border-neutral-600"
	>
		<p class="group overflow-hidden text-ellipsis whitespace-nowrap rounded px-1">
			<span class=" font-mono transition-all group-hover:blur-none md:blur-sm">
				{apikey.token}</span
			>
		</p>
		<div class="flex flex-row items-center gap-2">
			<button
				on:click={handleCopy}
				class="btn btn-circle btn-ghost transition-colors hover:text-accent"
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
					<rect x="9" y="9" width="13" height="13" rx="2" ry="2" />
					<path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" />
				</svg>
			</button>
			<button
				on:click={handleDelete}
				class="btn btn-circle btn-ghost transition-colors hover:text-error"
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
			</button>
		</div>
	</div>
</div>
