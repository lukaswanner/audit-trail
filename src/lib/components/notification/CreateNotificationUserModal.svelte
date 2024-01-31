<script lang="ts">
	import { channels } from "$lib/stores/channel";

	export let handleNewNotificationUser: (event: Event) => void;
	export let modalRef: HTMLDialogElement;

	export let nameError: string;
	export let phoneError: string;
	export let channelError: string;
	export let uploadError: string;


	let userName = "";
	let phoneNumber = "";
</script>

<dialog id="channel-id" bind:this={modalRef} class="modal modal-bottom sm:modal-middle">
	<form class="modal-box" on:submit|preventDefault={handleNewNotificationUser}>
		<h1 class="text-lg font-bold">Create new notification user</h1>
		<p class="py-4">Create a user who should be notified when things go south.</p>
		<div class="flex flex-col gap-4">
			<label class="form-control w-full">
				<div class="label">
					<span class="label-text">Name of the person to be notified</span>
				</div>
				<input
					name="name"
					type="text"
					placeholder="User name"
					class="input input-bordered w-full"
					bind:value={userName}
				/>
				{#if nameError}
					<p class="text-error">{nameError}</p>
				{/if}
			</label>
			<label class="form-control w-full">
				<div class="label">
					<span class="label-text">Phone number to be notified</span>
				</div>
				<input
					name="phoneNumber"
					placeholder="Phone number"
					type="text"
					class="input input-bordered w-full"
					bind:value={phoneNumber}
				/>
				{#if phoneError}
					<p class="text-error">{phoneError}</p>
				{/if}
			</label>
			<label class="form-control w-full">
				<div class="label">
					<span class="label-text">Choose notification channel for actor</span>
				</div>
				<select name="channelId" class="select select-bordered text-[1rem]">
					{#each $channels as channel}
						<option class="text-[1rem]" value={channel.id}>#{channel.title}</option>
					{/each}
				</select>
				{#if channelError}
					<p class="text-error">{channelError}</p>
				{/if}
			</label>
		</div>
		{#if uploadError}
			<p class="text-error">{uploadError}</p>
		{/if}
		<div class="modal-action justify-start">
			<button
				disabled={userName.length === 0 || phoneNumber.length === 0}
				class="btn btn-primary"
				type="submit">Create</button
			>
			<form method="dialog">
				<button class="btn btn-outline">Close</button>
			</form>
		</div>
	</form>
</dialog>
