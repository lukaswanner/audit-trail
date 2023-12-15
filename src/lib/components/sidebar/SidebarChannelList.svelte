<script lang="ts">
	import { channel, channels } from '$lib/stores/channel';

	export let feedActive: boolean;
	export let chartsActive: boolean;
	export let insightsActive: boolean;
</script>

{#each $channels as channel_item}
	<button
		on:click={() => channel.set(channel_item)}
		data-active={channel_item === $channel}
		data-feed={channel_item === $channel && feedActive}
		data-charts={channel_item === $channel && chartsActive}
		data-insights={channel_item === $channel && insightsActive}
		class="btn btn-ghost flex w-full flex-grow flex-row flex-nowrap items-center justify-start gap-2 data-[active=true]:bg-base-content/20 data-[charts=true]:text-secondary data-[feed=true]:text-primary data-[insights=true]:text-accent"
	>
		<svg
			xmlns="http://www.w3.org/2000/svg"
			class="h-6 w-6 flex-shrink-0 fill-current"
			viewBox="0 0 448 512"
			><!--!Font Awesome Free 6.5.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2023 Fonticons, Inc.--><path
				d="M181.3 32.4c17.4 2.9 29.2 19.4 26.3 36.8L197.8 128h95.1l11.5-69.3c2.9-17.4 19.4-29.2 36.8-26.3s29.2 19.4 26.3 36.8L357.8 128H416c17.7 0 32 14.3 32 32s-14.3 32-32 32H347.1L325.8 320H384c17.7 0 32 14.3 32 32s-14.3 32-32 32H315.1l-11.5 69.3c-2.9 17.4-19.4 29.2-36.8 26.3s-29.2-19.4-26.3-36.8l9.8-58.7H155.1l-11.5 69.3c-2.9 17.4-19.4 29.2-36.8 26.3s-29.2-19.4-26.3-36.8L90.2 384H32c-17.7 0-32-14.3-32-32s14.3-32 32-32h68.9l21.3-128H64c-17.7 0-32-14.3-32-32s14.3-32 32-32h68.9l11.5-69.3c2.9-17.4 19.4-29.2 36.8-26.3zM187.1 192L165.8 320h95.1l21.3-128H187.1z"
			/></svg
		>
		<p class="h-12 overflow-hidden text-ellipsis whitespace-nowrap font-bold leading-[3rem]">
			{channel_item.title}
		</p>
	</button>
{/each}
