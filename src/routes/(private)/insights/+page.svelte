<script lang="ts">
	import Card from "$lib/components/insight/Card.svelte";
	import { readInsightList } from "$lib/api/insight";
	import { insights } from "$lib/stores/insight";
	import Loading from "$lib/layout/loading/Loading.svelte";
	import { project } from "$lib/stores/project";

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
<div id="bento" class="grid h-full grid-cols-3 gap-4 overflow-auto p-4">
	{#each $insights as insight, index}
		<div
			class={` rounded-xl border-2 border-neutral 
${index !== 0 && index % 7 === 0 ? "col-span-2" : "col-span-1"}
${index === 0 || index % 9 === 0 ? "col-span-2 row-span-2" : "row-span-1"}
`}
		>
			<Card title={insight.title} value={insight.value} icon={insight.icon} />
		</div>
	{/each}
</div>

<style>
	#bento {
		grid-template-rows: auto-fill;
	}
</style>
