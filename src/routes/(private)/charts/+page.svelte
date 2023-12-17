<script lang="ts">
	import Chart, { type ChartItem } from 'chart.js/auto';
	import { onMount } from 'svelte';

	let chartRef: HTMLCanvasElement;
	let flowRef: HTMLCanvasElement;

	onMount(() => {
		new Chart(chartRef as ChartItem, {
			type: 'bar',
			data: {
				labels: ['January', 'February', 'March'],
				datasets: [
					{
						label: 'My First Dataset',
						data: [65, 59, 80, 81, 56, 55, 40],
						backgroundColor: ['#a38cf3', '#dcd6fb', '#a38cf3'],
						borderRadius: 12
					}
				]
			},
			options: {
				backgroundColor: 'transparent',
				scales: {
					y: {
						grid: {
							display: true,
							color: 'rgba(178, 204, 214, 0.1)',
							drawTicks: false
						},
						ticks: { display: false }
					},
					x: {
						grid: { display: false, drawTicks: true },
						ticks: { display: false }
					}
				},
				plugins: { title: { display: false }, legend: { display: false } }
			}
		});
		new Chart(flowRef as ChartItem, {
			type: 'bar',
			data: {
				labels: ['January', 'February', 'March'],
				datasets: [
					{
						label: 'My First Dataset',
						data: [65, 59, 80, 81, 56, 55, 40],
						backgroundColor: ['#a38cf3', '#dcd6fb', '#a38cf3'],
						borderRadius: 12
					}
				]
			},
			options: {
				backgroundColor: 'transparent',
				scales: {
					y: {
						grid: {
							display: true,
							color: 'rgba(178, 204, 214, 0.1)',
							drawTicks: false
						},
						ticks: { display: false }
					},
					x: {
						grid: { display: false, drawTicks: true },
						ticks: { display: false }
					}
				},
				plugins: { title: { display: false }, legend: { display: false } }
			}
		});
	});
</script>

<div class="flex flex-row justify-between border-b border-b-base-content/10 p-4">
	<h1 class="text-3xl font-bold brightness-150">charts</h1>
</div>

<div class="mt-4 flex h-full w-full flex-col items-center gap-4 p-4">
	<div class="flex w-full flex-col">
		<h2 class="text-3xl font-bold text-base-content brightness-150">Account Created</h2>
		<div class="flex flex-row items-center">
			<p>
				<span class="font-bold">115 </span>
				in the past 30 days
			</p>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				viewBox="0 0 24 24"
				fill="none"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
				class="mx-2 h-4 w-4 self-center stroke-green-500"
				><line x1="7" y1="17" x2="17" y2="7"></line><polyline points="7 7 17 7 17 17"
				></polyline></svg
			>
			<!-- <svg
				xmlns="http://www.w3.org/2000/svg"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
				class="h-6 w-6 stroke-red-500"
				><line x1="7" y1="7" x2="17" y2="17"></line><polyline points="17 7 17 17 7 17"
				></polyline></svg
			> -->

			<span class="text-green-500">15%</span>
			<div class="divider divider-horizontal m-0 h-2/4 self-center" />
			<p>115</p>
		</div>
		<canvas bind:this={chartRef} />
	</div>
	<div class="flex w-full flex-col">
		<h2 class="text-3xl font-bold text-base-content brightness-150">Order Placed</h2>
		<div class="flex flex-row items-center">
			<p>
				<span class="font-bold">11% </span>
				conversion from Account Created
			</p>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
				class="h-6 w-6 stroke-red-500"
				><line x1="7" y1="7" x2="17" y2="17"></line><polyline points="17 7 17 17 7 17"
				></polyline></svg
			>

			<span class="text-red-500">7%</span>
			<div class="divider divider-horizontal m-0 h-2/4 self-center" />
			<p>11%</p>
		</div>
		<canvas bind:this={flowRef} />
	</div>
</div>
