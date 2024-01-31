<script lang="ts">
	import { createNotification, readNotificationList } from "$lib/api/notification";
	import CreateNotificationUserModal from "$lib/components/notification/CreateNotificationUserModal.svelte";
	import ListItem from "$lib/components/settings/ListItem.svelte";
	import Loading from "$lib/layout/loading/Loading.svelte";
	import { channels } from "$lib/stores/channel";
	import { project } from "$lib/stores/project";
	import type {
		Notification,
		UpdateNotificationPayload
	} from "$lib/types/notification/notificationTypes";
	import { onMount } from "svelte";

	let loading = false;
	let notifications: Notification[] = [];
	let modalRef: HTMLDialogElement;

	let channelError = "";
	let nameError = "";
	let phoneError = "";
	let uploadError = "";

	async function handleNewNotificationUser(event: Event) {
		const form = event.target as HTMLFormElement;
		const formData = new FormData(form);

		const id = formData.get("channelId");
		const name = formData.get("name") as string;
		const phoneNumber = formData.get("phoneNumber") as string;

		if (!id) {
			channelError = "Please select a channel";
			return;
		}
		if (!name) {
			nameError = "Please enter a name";
			return;
		}
		if (!phoneNumber) {
			phoneError = "Please enter a phone number";
			return;
		}

		const payload: UpdateNotificationPayload = {
			channelId: parseInt(id as string),
			name,
			phoneNumber
		};

		const res = await createNotification(payload);
		if (res.status === 201) {
			modalRef.close();
		} else {
			uploadError = "Something went wrong";
		}
	}

	async function fetchNotificationList() {
		loading = true;
		const res = await readNotificationList();
		if (res.status !== 200) {
			loading = false;
			return;
		}
		try {
			const data = await res.json();
			notifications = data;
		} catch (error) {
			console.log(error);
		}
		loading = false;
	}

	$: $project, fetchNotificationList();

	onMount(() => {
		fetchNotificationList();
	});
</script>

<div class="flex flex-row justify-between border-b border-b-base-content/10 p-4">
	<h1 class="text-3xl font-bold brightness-150">notification</h1>
</div>

{#if loading}
	<Loading />
{/if}

<div class="w-full overflow-auto">
	<div class="mb-8 flex h-full max-w-2xl flex-col items-start gap-4 p-4">
		<div class="w-full">
			<div
				class="flex w-full flex-row items-center justify-start gap-4 rounded-tl-md rounded-tr-md border-l-2 border-r-2 border-t-2 border-neutral bg-base-300 p-4"
			>
				<div class="flex items-center justify-center rounded-lg bg-primary/10 p-4">
					<svg
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
						class="h-4 w-4"
					>
						<path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9" />
						<path d="M13.73 21a2 2 0 0 1-3.46 0" />
					</svg>
				</div>
				<h1 class="text-2xl brightness-150">your notifications</h1>
			</div>
			<div
				class="flex w-full flex-col items-start justify-center gap-1 rounded-bl-md rounded-br-md border-b-2 border-l-2 border-r-2 border-t-2 border-neutral bg-base-300 p-4"
			>
				<p>notifications help you stay on top of your project</p>
				<p>you can create notifications for any channel in your project</p>
				<button class="btn btn-primary mt-8" on:click={() => modalRef.showModal()}>
					+ create notification</button
				>
			</div>
		</div>

		{#each $channels as channel}
			<div class="w-full">
				<div
					class="flex w-full flex-col items-start justify-center gap-4 border-l-2 border-r-2 border-t-2 border-neutral bg-base-300 p-4 data-[last=true]:rounded-bl-md data-[last=true]:rounded-br-md data-[last=true]:border-b-2"
				>
					<p class="text-xl text-secondary">#{channel.title}</p>
				</div>
				{#if notifications.filter((notification) => notification.channelId === channel.id).length === 0}
					<div
						class="flex w-full flex-col items-start justify-center rounded-bl-md rounded-br-md border-2 border-neutral bg-base-300 p-4"
					>
						<h2 class="text-lg">
							No notification in {channel.title}
						</h2>
					</div>
				{:else}
					{#each notifications.filter((notification) => notification.channelId === channel.id) as user, index}
						<div
							data-last={index ===
								notifications.filter(
									(notification) => notification.channelId === channel.id
								).length -
									1}
							class="flex w-full flex-col items-start justify-center gap-4 border-l-2 border-r-2 border-t-2 border-neutral bg-base-300 p-4 data-[last=true]:rounded-bl-md data-[last=true]:rounded-br-md data-[last=true]:border-b-2"
						>
							<ListItem
								name={user.name}
								hrefLink={`/settings/notifications/${user.id}`}
							/>
						</div>
					{/each}
				{/if}
			</div>
		{/each}
	</div>
</div>
<CreateNotificationUserModal
	bind:modalRef
	{handleNewNotificationUser}
	{uploadError}
	{channelError}
	{nameError}
	{phoneError}
/>
