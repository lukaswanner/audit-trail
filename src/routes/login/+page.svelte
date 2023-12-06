<script>
	import LoginCard from '$lib/components/LoginCard.svelte';

	let successfulLogIn = true;
	let closeToast = false;
</script>

{#if !successfulLogIn && !closeToast}
	<div class="flex justify-center mb-4">
		<div class="max-w-6xl alert alert-error flex justify-between">
			<span>Invalid e-mail or password.</span>
			<button
				class="btn btn-square btn-outline"
				style="color: black;"
				on:click={() => {
					closeToast = true;
				}}
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="h-6 w-6"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
					><path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M6 18L18 6M6 6l12 12"
					/></svg
				>
			</button>
		</div>
	</div>
{/if}
<div class="flex">
	<LoginCard bind:successfulLogIn bind:closeToast />
	<div class="hero min-h-screen bg-base-200">
		<div class="hero-content flex-col lg:flex-row">
			<img
				src="https://daisyui.com/images/stock/photo-1635805737707-575885ab0820.jpg"
				class="max-w-sm rounded-lg shadow-2xl"
			/>
			<div>
				<h1 class="text-5xl font-bold">Audit Trail!</h1>
				<p class="py-6">
					Provident cupiditate voluptatem et in. Quaerat fugiat ut assumenda excepturi
					exercitationem quasi. In deleniti eaque aut repudiandae et a id nisi.
				</p>
				<button class="btn btn-primary">Get Started</button>
			</div>
		</div>
	</div>
</div>
