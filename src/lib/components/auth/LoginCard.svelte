<script lang="ts">
	import { page } from '$app/stores';

	type EventType = 'emailFocus' | 'passwordFocus' | 'emailTouched' | 'passwordTouched';

	export let addToEventLog: (event: EventType) => void;

	let message: string;

	$: message = $page.url.searchParams.get('message') ?? '';

	function handleLogin(event: Event) {
		const form = event.target as HTMLFormElement;
		const formData = new FormData(form);
		const data = Object.fromEntries(formData.entries());
		console.log(data);
	}
</script>

<form
	class="flex flex-col items-center gap-4 rounded-3xl bg-base-300 px-4"
	on:submit|preventDefault={handleLogin}
>
	<div class="mb-20 mt-auto text-center">
		<h1 class="text-3xl font-bold">Welcome back!</h1>
		<p class="text-base-content/50">Please enter your details</p>
	</div>

	<div class="flex w-full flex-col items-center gap-2">
		<label class="form-control w-full max-w-xs">
			<input
				on:focusin={() => addToEventLog('emailFocus')}
				on:input={() => addToEventLog('emailTouched')}
				type="text"
				placeholder="Email"
				class="input w-full max-w-xs"
				required
			/>
		</label>

		<label class="form-control w-full max-w-xs">
			<input
				on:focusin={() => addToEventLog('passwordFocus')}
				on:input={() => addToEventLog('passwordTouched')}
				type="text"
				placeholder="Password"
				class="input w-full max-w-xs"
				required
			/>
		</label>

		<div class="form-control w-full max-w-xs">
			<label class="label cursor-pointer justify-start gap-2">
				<input type="checkbox" checked={true} class="checkbox checkbox-xs" />
				<span class="label-text">Remember me</span>
			</label>
		</div>
	</div>

	<button class="btn btn-primary w-full max-w-xs">Login</button>
	<p class="mb-4 mt-auto text-sm">
		Don't have an account? <a class="link-info" href="/register">Sign up</a>
	</p>
</form>
