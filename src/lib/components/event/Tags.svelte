<script lang="ts">
	export let tags: Record<string, unknown>[];
</script>

<div class="flex flex-col items-end justify-center">
	<div class="flex flex-row gap-2">
		{#if tags.length === 0}
			<div
				class="flex h-8 items-center justify-center rounded-md border border-primary bg-primary/50 px-2"
			>
				<p class="whitespace-nowrap">No tags</p>
			</div>
		{/if}
		{#if tags.length > 0}
			<div
				class="flex h-8 items-center justify-center rounded-md border border-primary bg-primary/50 px-2"
			>
				<p class="whitespace-nowrap">
					{Object.keys(tags[0])[0]}:
					<span class="brightness-150">
						{Object.values(tags[0])[0]}
					</span>
				</p>
			</div>
		{/if}
		{#if tags.length > 1}
			<div
				id="has-tooltip"
				class="flex h-8 w-fit items-center justify-center self-end rounded-md border border-primary bg-primary/50 px-2"
			>
				+{tags.length - 1}
				<span id="tooltip-wrapper">
					<div
						class="relative right-12 top-[3.5rem] z-10 flex h-fit w-fit flex-col gap-2 rounded-md border border-base-300 bg-base-300 p-4"
					>
						{#each tags.slice(1) as tag}
							<div
								class="flex h-8 items-center justify-center whitespace-nowrap rounded-md border border-primary bg-primary/50 px-2"
							>
								<p>
									{Object.keys(tag)[0]}:
									<span class="brightness-150">
										{Object.values(tag)[0]}
									</span>
								</p>
							</div>
						{/each}
					</div>
				</span>
			</div>
		{/if}
	</div>
</div>

<style>
	#tooltip-wrapper {
		position: absolute;
		visibility: hidden;
	}

	#has-tooltip:hover #tooltip-wrapper {
		visibility: visible;
	}
</style>
