<script>
	import Card from '$lib/components/insight/Card.svelte';
</script>

<div class="flex flex-row justify-between border-b border-b-base-content/10 p-4">
	<h1 class="text-3xl font-bold brightness-150">insights</h1>
</div>

<div class="grid grid-cols-[1fr_1fr] gap-4 p-4">
	<Card title="Online" count="27" emoji="🟢" />
	<Card title="24h sales" count="$2,250" emoji="💲" />
	<Card title="orders in transit" count="12" emoji="🚚" />
	<Card title="orders processing" count="23" emoji="🏭" />
</div>
