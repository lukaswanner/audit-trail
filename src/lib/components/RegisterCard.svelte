<script lang="ts">
	let passwordType = 'password';
	let email = '';

	function switchType() {
		passwordType = passwordType === 'text' ? 'password' : 'text';
	}
</script>

<div class="flex flex-col items-center bg-base-100 min-w-[25vw] rounded-lg p-4">
	<label class="form-control w-full max-w-xs">
		<div class="label">
			<span class="label-text">First Name</span>
		</div>
		<input type="text" placeholder="Type here" class="input input-bordered w-full max-w-xs" />
	</label>
	<label class="form-control w-full max-w-xs">
		<div class="label">
			<span class="label-text">Last Name</span>
		</div>
		<input type="text" placeholder="Type here" class="input input-bordered w-full max-w-xs" />
	</label>
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
	</label>
	<label class="form-control w-full max-w-xs">
		<div class="label">
			<span class="label-text">Password</span>
		</div>

		<div class="relative">
			<input
				type={passwordType}
				placeholder="Type here"
				class="input input-bordered w-full max-w-xs"
			/>
			<span class="absolute inset-y-0 right-0 flex items-center pl-2">
				<label class="swap swap-rotate">
					<input type="checkbox" on:click={switchType} />
					<svg
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="1"
						stroke-linecap="round"
						stroke-linejoin="round"
						class="mr-1.5 swap-off w-7 h-7"
						><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path><circle
							cx="12"
							cy="12"
							r="3"
						></circle></svg
					>
					<svg
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="1"
						stroke-linecap="round"
						stroke-linejoin="round"
						class="mr-1 swap-on w-7 h-7"
						><path
							d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"
						></path><line x1="1" y1="1" x2="23" y2="23"></line></svg
					>
				</label>
			</span>
		</div>
	</label>
</div>
