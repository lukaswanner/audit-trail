<script lang="ts">
	import { goto } from "$app/navigation";
	import { page } from "$app/stores";
	import { login } from "$lib/api/auth";
	import type { UserCredentialsLogin } from "$lib/types/account/AccountTypes";
	import type { EventType } from "$lib/types/login/login";
	import { handleSuccessfulRedirect } from "$lib/utils/redirectTo";

	export let addToEventLog: (event: EventType) => void;

	let timer: number;

	async function handleLogin(event: Event) {
		const form = event.target as HTMLFormElement;
		const formData = new FormData(form);
		const data = Object.fromEntries(formData.entries());
		const { email, password, rememberMe } = data;
		const creds: UserCredentialsLogin = {
			email: email as string,
			password: password as string,
			rememberMe: rememberMe === "on" ? true : false
		};
		const res = await login(creds);
		if (res.status === 200) {
			addToEventLog("loginSuccess");
			setTimeout(() => {
				goto(handleSuccessfulRedirect($page.url));
			}, 1000);
		} else {
			addToEventLog("loginFailure");
		}
	}

	function handleCheckbox(event: Event) {
		const { checked } = event.target as HTMLInputElement;
		if (checked) {
			addToEventLog("rememberMe");
		} else {
			addToEventLog("DontRememberMe");
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
	on:submit|preventDefault={handleLogin}
>
	<div class="mb-10 mt-auto text-center">
		<h1 class="text-3xl font-bold">Welcome back!</h1>
		<p class="text-base-content/50">Please enter your details</p>
	</div>

	<div class="flex w-full flex-col items-center gap-2">
		<label class="form-control w-full max-w-sm">
			<input
				on:input={() => debounceInput("emailTouched")}
				type="email"
				name="email"
				placeholder="Email"
				class="input w-full max-w-sm"
				required
			/>
		</label>

		<label class="form-control w-full max-w-sm">
			<input
				on:input={() => debounceInput("passwordTouched")}
				type="text"
				name="password"
				placeholder="Password"
				class="input w-full max-w-sm"
				required
			/>
		</label>

		<div class="form-control w-full max-w-sm">
			<label class="label cursor-pointer justify-start gap-2">
				<input
					on:change={handleCheckbox}
					name="rememberMe"
					type="checkbox"
					checked={true}
					class="checkbox checkbox-xs"
				/>
				<span class="label-text">Remember me</span>
			</label>
		</div>
	</div>

	<button class="btn btn-primary w-full max-w-sm">Login</button>
	<p class="mb-4 mt-auto text-sm">
		Don't have an account? <a class="link-info" href="/register">Sign up</a>
	</p>
</form>
