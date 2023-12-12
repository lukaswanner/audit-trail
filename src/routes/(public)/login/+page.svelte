<script lang="ts">
	import LoginCard from '$lib/components/auth/LoginCard.svelte';
	import { formatRelative } from 'date-fns';
	import { fade } from 'svelte/transition';

	type EventType = 'emailFocus' | 'passwordFocus' | 'emailTouched' | 'passwordTouched';

	type PseudoEvent = {
		icon: string;
		title: string;
		timeStamp: number;
	};

	let events: PseudoEvent[] = [];

	function addToEventLog(event: EventType) {
		let newEvent: PseudoEvent;
		switch (event) {
			case 'emailFocus':
				newEvent = {
					icon: '📧',
					title: 'Email input focused',
					timeStamp: Date.now()
				};
				break;
			case 'passwordFocus':
				newEvent = {
					icon: '🔑',
					title: 'Password input focused',
					timeStamp: Date.now()
				};
				break;
			case 'emailTouched':
				newEvent = {
					icon: '📧',
					title: 'New Email input',
					timeStamp: Date.now()
				};
				break;
			case 'passwordTouched':
				newEvent = {
					icon: '🔑',
					title: 'New Password input',
					timeStamp: Date.now()
				};
				break;
		}
		events = [...events, newEvent];
	}

	let clear: number;
	$: {
		clearInterval(clear);
		clear = setInterval(() => {
			events.pop();
			events = [...events];
		}, 2500);
	}
</script>

<div class="grid h-full w-full grid-cols-1 lg:grid-cols-[2fr_4fr]">
	<LoginCard {addToEventLog} />
	<div class="overflow-hiden hidden flex-col items-center justify-center gap-4 lg:flex">
		{#each events as event}
			<div
				transition:fade
				class="grid w-full max-w-md grid-cols-[auto_1fr] gap-4 rounded-3xl border-2 border-secondary p-4"
			>
				<div
					class="flex aspect-square w-16 flex-col items-center justify-center rounded-3xl bg-primary/30"
				>
					<div class="text-3xl">{event.icon}</div>
				</div>
				<div class="flex flex-col justify-between">
					<p class="text-bold text-xl">{event.title}</p>
					<div class="flex w-full flex-row justify-between gap-2">
						<p>Website</p>
						<p>{formatRelative(event.timeStamp, new Date())}</p>
					</div>
				</div>
			</div>
		{/each}
	</div>
</div>
