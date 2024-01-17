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
</script>

<div
	class="flex h-16 w-full max-w-2xl flex-shrink-0 flex-row items-center justify-between overflow-hidden rounded-xl border border-neutral pl-8 pr-2"
>
	<p
		class="overflow-hidden text-ellipsis whitespace-nowrap blur-sm transition-all hover:blur-none"
	>
		<span>{apikey.projectTitle} - </span>
		{apikey.token}
	</p>
	<p></p>
	<div class="flex flex-row items-center gap-2">
		<button class="btn btn-circle btn-ghost transition-colors hover:text-primary">
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
