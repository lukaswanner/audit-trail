<script lang="ts">
	export let title: string;
	export let count: string;
	export let emoji: string;
</script>

<div class="flex flex-col items-start rounded-2xl bg-neutral-content/20 border border-neutral-content/50 p-4">
	<p class="h-8 w-8 self-end text-3xl">{emoji}</p>
	<p class="text-xl font-bold brightness-150">{count}</p>
	<p class="text-md">{title}</p>
</div>
