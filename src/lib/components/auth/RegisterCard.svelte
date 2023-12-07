<script lang="ts">
	let password = '';
	let email = '';
	let firstName = '';
	let lastName = '';
</script>

<div class="flex flex-col items-center bg-base-100 min-w-[25vw] rounded-lg p-4">
	<label class="form-control w-full max-w-xs">
		<div class="label">
			<span class="label-text">First Name</span>
		</div>
		<input
			bind:value={firstName}
			type="text"
			placeholder="Type here"
			class="input input-bordered w-full max-w-xs"
		/>
	</label>
	<label class="form-control w-full max-w-xs">
		<div class="label">
			<span class="label-text">Last Name</span>
		</div>
		<input
			bind:value={lastName}
			type="text"
			placeholder="Type here"
			class="input input-bordered w-full max-w-xs"
		/>
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
				type="password"
				placeholder="Type here"
				bind:value={password}
				class="input input-bordered w-full max-w-xs"
			/>
		</div>
	</label>
</div>
