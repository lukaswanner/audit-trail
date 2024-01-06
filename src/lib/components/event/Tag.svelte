<script lang="ts">
	export let tag: Record<string, unknown>;

	const entries = Object.entries(tag)[0];
</script>

<div class="flex h-fit w-fit flex-row items-center rounded-lg bg-base-300 px-2 py-1">
	<p class="text-base-content">
		{entries[0]}:
		<span class="text-base-content brightness-150">
			{entries[1]}
		</span>
	</p>
</div>
