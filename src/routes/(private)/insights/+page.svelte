<script lang="ts">
	import Card from '$lib/components/insight/Card.svelte';
	import { readInsightList } from '$lib/api/insight';
	import { insights } from '$lib/stores/insight';
	import Loading from '$lib/layout/loading/Loading.svelte';
	import { project } from '$lib/stores/project';

	let loading = true;
	async function fetchInsights(projectTitle: string): Promise<void> {
		const res = await readInsightList(projectTitle);
		if (res.status == 200) {
			const data = await res.json();
			insights.set(data);
		} else {
			insights.set([]);
		}
		loading = false;
	}

	$: if ($project) {
		fetchInsights($project.title);
	}
</script>

<div class="flex flex-row justify-between border-b border-b-base-content/10 p-4">
	<h1 class="text-3xl font-bold brightness-150">insights</h1>
</div>

{#if loading}
	<div class="relative">
		<Loading />
	</div>
{/if}
<div class="grid grid-cols-[1fr_1fr] gap-4 p-4">
	{#each $insights as insight}
		<Card title={insight.title} value={insight.value} icon={insight.icon} />
	{/each}
</div>
