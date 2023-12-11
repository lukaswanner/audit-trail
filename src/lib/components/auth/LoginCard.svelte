<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { login } from '$lib/api/auth';

	let password = '';
	let email = '';
	let rememberAccount = false;
	let emptyEmail = false;
	let emptyPassword = false;
	export let closeToast: boolean;
	export let successfulLogIn: boolean;

	async function logIn() {
		emptyPassword = password === '';
		emptyEmail = email === '';
		closeToast = false;
		const creds = {
			email,
			password,
			rememberMe: rememberAccount
		};

		const res = await login(creds);
		successfulLogIn = res.status === 200;
		if (res.status === 200) {
			const redirectTo = $page.url.searchParams.get('redirectTo');
			if (redirectTo) {
				goto(redirectTo);
			} else {
				goto('/');
			}
		}
	}
</script>

<div class="flex min-w-[25vw] flex-col items-center rounded-lg bg-base-100 p-4">
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
			<label class="label cursor-pointer">
				<input type="checkbox" bind:checked={rememberAccount} class="checkbox-info checkbox" />
				<span class="label-text">Remember me</span>
			</label>
		</div>
		<button class="btn btn-info btn-active mt-4" on:click={logIn}>Log In</button>
		<hr class="mt-4" />
		<div>
			No account yet?
			<a href="/register" class="link link-info">Click here to register</a>
		</div>
	</label>
</div>
