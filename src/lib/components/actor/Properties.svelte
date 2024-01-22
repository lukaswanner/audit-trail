<script lang="ts">
	type Property = Map<string, string>;
	type Step = "key" | "value";

	export let properties: Property;
	export let add: (key: string, value: string) => void;
	export let remove: (key: string) => void;

	let key = "";
	let value = "";
	let step: Step = "key";

	function addProperty() {
		add(key, value);
		key = "";
		value = "";
		step = "key";
	}

	function removeProperty(key: string) {
		remove(key);
	}
</script>

<div class="mb-8 flex w-full flex-col gap-2">
	{#if !properties || properties.size === 0}
		<p class="text-base-content/50">no properties</p>
	{/if}
	{#each properties.keys() as mapKey}
		<div
			class="flex max-w-md flex-row items-center justify-between rounded-lg border border-base-content/20 bg-base-100 outline-base-content/20"
		>
			<div class="flex flex-row gap-2 px-4">
				<p>{mapKey}:</p>
				<p>{properties.get(mapKey)}</p>
			</div>

			<button
				type="button"
				on:click={() => {
					removeProperty(mapKey);
				}}
				class="btn btn-error rounded-bl-none rounded-tl-none"
			>
				<svg
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
					class="h-4 w-4"
				>
					<circle cx="12" cy="12" r="10" />
					<line x1="15" y1="9" x2="9" y2="15" />
					<line x1="9" y1="9" x2="15" y2="15" />
				</svg>
			</button>
		</div>
	{/each}
</div>

<h2 class="mb-2">New property</h2>
<div class="w-full">
	{#if step === "key"}
		<div
			class="input-wrapper flex max-w-md flex-row rounded-lg border border-base-content/20 bg-base-100 outline-base-content/20"
		>
			<input
				type="text"
				placeholder="actor properties"
				class="input w-full rounded-br-none rounded-tr-none focus-within:border-none focus:border-none focus:outline-none focus:focus-within:outline-none"
				bind:value={key}
			/>
			<button
				type="button"
				disabled={key.length === 0}
				on:click={() => (step = "value")}
				class="animate-bounce-x group btn btn-primary rounded-bl-none rounded-tl-none"
			>
				<svg
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
					class="h-4 w-4"
				>
					<line x1="5" y1="12" x2="19" y2="12" />
					<polyline points="12 5 19 12 12 19" />
				</svg>
			</button>
		</div>
	{:else}
		<div
			class="input-wrapper flex max-w-md flex-row rounded-lg border border-base-content/20 bg-base-100 outline-base-content/20"
		>
			<div class="flex flex-row items-center pl-4 pr-2">
				<p class="whitespace-nowrap text-accent">{key}:</p>
			</div>
			<input
				type="text"
				placeholder="actor properties"
				class="input w-full rounded-none pl-2 focus-within:border-none focus:border-none focus:outline-none focus:focus-within:outline-none"
				bind:value
			/>
			<button
				type="button"
				disabled={value.length === 0}
				on:click={() => {
					addProperty();
				}}
				class="animate-bounce-x group btn btn-primary rounded-bl-none rounded-tl-none"
			>
				<svg
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
					class="h-4 w-4"
				>
					<line x1="5" y1="12" x2="19" y2="12" />
					<polyline points="12 5 19 12 12 19" />
				</svg>
			</button>
		</div>
	{/if}
</div>

<style>
	.input-wrapper:focus,
	.input-wrapper:focus-within {
		outline-style: solid;
		outline-width: 2px;
		outline-offset: 2px;
	}

	@keyframes bounce {
		0%,
		100% {
			transform: none;
			animation-timing-function: cubic-bezier(0.8, 0, 1, 1);
		}

		50% {
			transform: translateX(25%);
			animation-timing-function: cubic-bezier(0, 0, 0.2, 1);
		}
	}

	.animate-bounce-x:hover svg {
		animation: bounce 1s infinite;
	}
</style>
