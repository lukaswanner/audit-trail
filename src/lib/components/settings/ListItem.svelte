<script lang="ts">
	export let name: string;
	export let hrefLink: string;
</script>

<div class="flex w-full flex-row items-center justify-between">
	<p class="text-base-content">{name}</p>
	<div class="flex flex-row items-center gap-2">
		<a href={hrefLink} class="transition-colors hover:text-secondary">
			<svg
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
				class="h-4 w-4"
				><path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"></path><path
					d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"
				></path></svg
			>
		</a>
		<a href={`${hrefLink}/danger`} class="transition-colors hover:text-secondary">
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
		</a>
	</div>
</div>
