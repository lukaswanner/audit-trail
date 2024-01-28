<script lang="ts">
	export let handleNewNotificationUser: (event: Event) => void;
	export let modalRef: HTMLDialogElement;

	let userName = "";
	let phoneNumber = "";

	let error = "";
	let errorNumber = "";
</script>

<dialog id="channel-id" bind:this={modalRef} class="modal modal-bottom sm:modal-middle">
	<form class="modal-box" on:submit|preventDefault={handleNewNotificationUser}>
		<h1 class="text-lg font-bold">Create new notification user</h1>
		<p class="py-4">Create a user who should be notified when things go south.</p>
		<div class="flex flex-col gap-4">
			<div>
				<input
					name="name"
					type="text"
					placeholder="User name"
					class="input input-bordered w-full"
					bind:value={userName}
				/>
				{#if error}
					<p class="text-error">{error}</p>
				{/if}
			</div>
			<div>
				<input
					name="phoneNumber"
					placeholder="Phone number"
					type="text"
					class="input input-bordered w-full"
					bind:value={phoneNumber}
				/>
				{#if errorNumber}
					<p class="text-error">{errorNumber}</p>
				{/if}
			</div>
		</div>
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
