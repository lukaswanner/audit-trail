<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { register } from '$lib/api/auth';
	import type { UserCredentials } from '$lib/types/account/AccountTypes';
	import type { EventType } from '$lib/types/login/login';
	import { handleSuccessfulRedirect } from '$lib/utils/redirectTo';

	export let addToEventLog: (event: EventType) => void;

	let timer: number;

	async function handleRegister(event: Event) {
		const form = event.target as HTMLFormElement;
		const formData = new FormData(form);
		const data = Object.fromEntries(formData.entries());
		const { email, password } = data;
		const creds: UserCredentials = {
			email: email as string,
			password: password as string
		};
		const res = await register(creds);
		if (res.status === 200) {
			addToEventLog('registerSuccess');
			setTimeout(() => {
				goto(handleSuccessfulRedirect($page.url));
			}, 1000);
		} else {
			addToEventLog('registerFailure');
		}
	}

	function debounceInput(msg: EventType) {
		clearTimeout(timer);
		timer = setTimeout(() => {
			addToEventLog(msg);
		}, 1000);
	}
</script>

<form
	class="flex flex-col items-center gap-4 rounded-3xl bg-base-300 px-4"
	on:submit|preventDefault={handleRegister}
>
	<div class="mb-10 mt-auto text-center">
		<h1 class="text-3xl font-bold">Welcome, nice to meet you!</h1>
		<p class="text-base-content/50">Please enter your details</p>
	</div>

	<div class="flex w-full flex-col items-center gap-2">
		<label class="form-control w-full max-w-sm">
			<input
				on:input={() => debounceInput('emailTouched')}
				type="email"
				name="email"
				placeholder="Email"
				class="input w-full max-w-sm"
				required
			/>
		</label>

		<label class="form-control w-full max-w-sm">
			<input
				on:input={() => debounceInput('passwordTouched')}
				type="text"
				name="password"
				placeholder="Password"
				class="input w-full max-w-sm"
				required
			/>
		</label>
	</div>

	<button class="btn btn-primary w-full max-w-sm">Create account</button>
	<p class="mb-4 mt-auto text-sm">
		Already have an account? <a class="link-info" href="/login">Sign in</a>
	</p>
</form>
