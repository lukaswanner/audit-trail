<script lang="ts">
	let password = '';
	let email = '';
	let rememberAccount = false;
	let emptyEmail = false;
	let emptyPassword = false;
	let correctCredentials = true;
	export let closeToast: boolean;
	export let successfulLogIn: boolean;

	function logIn() {
		emptyPassword = password === '';
		emptyEmail = email === '';
		correctCredentials = !emptyEmail && !emptyPassword;
		closeToast = false;
		(async () => {
			const response = await fetch('http://localhost:3000/auth/login', {
				method: 'POST',
				headers: {
					Accept: 'application/json',
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({ email: email, password: password })
			});
			successfulLogIn = correctCredentials && response.status === 200;
			if (successfulLogIn) {
				console.log('You are now logged in and taken back to the landing page');
			} else {
				console.log('Error trying to log in');
			}
		})();
	}
</script>

<div class="flex flex-col items-center bg-base-100 min-w-[25vw] rounded-lg p-4">
	<label class="form-control w-full max-w-xs">
		<div class="label">
			<span class="label-text">E-Mail</span>
		</div>
		<input
			bind:value={email}
			type="text"
			placeholder="Type here"
			class="input input-bordered w-full max-w-xs"
		/>
		{#if emptyEmail}
			<span class="label-text-alt" style="color: red;">This field is necessary</span>
		{/if}
	</label>
	<label class="form-control w-full max-w-xs">
		<div class="label">
			<span class="label-text">Password</span>
		</div>

		<div class="relative">
			<input
				type="password"
				placeholder="Type here"
				bind:value={password}
				class="input input-bordered w-full max-w-xs"
			/>
			<span class="absolute inset-y-0 right-0 flex items-center pl-2"> </span>
		</div>
		{#if emptyPassword}
			<span class="label-text-alt" style="color: red;">This field is necessary</span>
		{/if}
		<div class="form-control mt-4">
			<label class="cursor-pointer label">
				<input type="checkbox" bind:checked={rememberAccount} class="checkbox checkbox-info" />
				<span class="label-text">Remember me</span>
			</label>
		</div>
		<button class="btn btn-active btn-info mt-4" on:click={logIn}>Log In</button>
		<hr class="mt-4" />
		<div>
			No account yet?
			<a href="/register" class="link link-info">Click here to register</a>
		</div>
	</label>
</div>
