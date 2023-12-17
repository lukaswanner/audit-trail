<script lang="ts">
	export let title: string;
	export let value: string;
	export let icon: string;
</script>

<div class="flex flex-col items-start rounded-2xl border-2 border-neutral-content/50 p-4">
	<p class="h-8 w-8 self-end text-3xl">{icon}</p>
	<p class="text-xl font-bold brightness-150">{value}</p>
	<p class="text-md">{title}</p>
</div>
