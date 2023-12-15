<script lang="ts">
	import { goto } from '$app/navigation';
	import { project } from '$lib/stores/project';

	export let feedActive: boolean;
	export let chartsActive: boolean;
	export let insightsActive: boolean;
</script>

<div class="flex w-full flex-col gap-2">
	<button
		class="btn btn-ghost flex flex-row items-center justify-start gap-2 text-base-content transition-colors data-[active=true]:bg-base-content/20 data-[active=true]:text-primary"
		disabled={!$project}
		data-active={feedActive}
		on:click={() => goto('/')}
	>
		<svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 fill-current" viewBox="0 0 512 512"
			><!--!Font Awesome Free 6.5.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2023 Fonticons, Inc.--><path
				d="M121 32C91.6 32 66 52 58.9 80.5L1.9 308.4C.6 313.5 0 318.7 0 323.9V416c0 35.3 28.7 64 64 64H448c35.3 0 64-28.7 64-64V323.9c0-5.2-.6-10.4-1.9-15.5l-57-227.9C446 52 420.4 32 391 32H121zm0 64H391l48 192H387.8c-12.1 0-23.2 6.8-28.6 17.7l-14.3 28.6c-5.4 10.8-16.5 17.7-28.6 17.7H195.8c-12.1 0-23.2-6.8-28.6-17.7l-14.3-28.6c-5.4-10.8-16.5-17.7-28.6-17.7H73L121 96z"
			/></svg
		>
		<p>feed</p>
	</button>
	<button
		class="btn btn-ghost flex flex-row items-center justify-start gap-2 text-base-content transition-colors data-[active=true]:bg-base-content/20 data-[active=true]:text-secondary"
		disabled={!$project}
		data-active={chartsActive}
		on:click={() => goto('/charts')}
	>
		<svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 fill-current" viewBox="0 0 512 512"
			><!--!Font Awesome Free 6.5.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2023 Fonticons, Inc.--><path
				d="M3.9 54.9C10.5 40.9 24.5 32 40 32H472c15.5 0 29.5 8.9 36.1 22.9s4.6 30.5-5.2 42.5L320 320.9V448c0 12.1-6.8 23.2-17.7 28.6s-23.8 4.3-33.5-3l-64-48c-8.1-6-12.8-15.5-12.8-25.6V320.9L9 97.3C-.7 85.4-2.8 68.8 3.9 54.9z"
			/></svg
		>
		<p>charts</p>
	</button>
	<button
		class="btn btn-ghost flex flex-row items-center justify-start gap-2 text-base-content transition-colors data-[active=true]:bg-base-content/20 data-[active=true]:text-accent"
		disabled={!$project}
		data-active={insightsActive}
		on:click={() => goto('/insights')}
	>
		<svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 fill-current" viewBox="0 0 384 512"
			><!--!Font Awesome Free 6.5.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2023 Fonticons, Inc.--><path
				d="M272 384c9.6-31.9 29.5-59.1 49.2-86.2l0 0c5.2-7.1 10.4-14.2 15.4-21.4c19.8-28.5 31.4-63 31.4-100.3C368 78.8 289.2 0 192 0S16 78.8 16 176c0 37.3 11.6 71.9 31.4 100.3c5 7.2 10.2 14.3 15.4 21.4l0 0c19.8 27.1 39.7 54.4 49.2 86.2H272zM192 512c44.2 0 80-35.8 80-80V416H112v16c0 44.2 35.8 80 80 80zM112 176c0 8.8-7.2 16-16 16s-16-7.2-16-16c0-61.9 50.1-112 112-112c8.8 0 16 7.2 16 16s-7.2 16-16 16c-44.2 0-80 35.8-80 80z"
			/></svg
		>
		<p>insights</p>
	</button>
</div>
