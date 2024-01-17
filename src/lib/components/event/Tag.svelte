<script lang="ts">
	export let tag: Record<string, unknown>;

	const entries = Object.entries(tag)[0];
</script>

<div class="rounded-md border border-primary bg-primary/20 px-2 py-1">
	<p class="text-base-content">
		{entries[0]}:
		<span class="brightness-150">
			{entries[1]}
		</span>
	</p>
</div>
