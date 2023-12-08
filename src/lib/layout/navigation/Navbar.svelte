<script lang="ts">
	export let list: string[];
</script>

<div class="navbar bg-base-100 mb-3">
	<div class="navbar-start">
		<div class="dropdown">
			<div tabindex="0" role="button" class="btn btn-ghost btn-circle">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="h-5 w-5"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
					><path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M4 6h16M4 12h16M4 18h7"
					/></svg
				>
			</div>
			<ul
				tabindex="0"
				class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52"
			>
				{#each list as item}
					<li><a href="/register"> {item}</a></li>
				{/each}
			</ul>
		</div>
	</div>
	<div class="navbar-center">
		<a class="btn btn-ghost text-xl" href="/">Audit Trail</a>
	</div>
	<div class="navbar-end">
		<a class="btn btn-outline btn-primary mr-3" href="/login">Log In</a>
		<label class="cursor-pointer grid place-items-center mr-3">
			<input
				type="checkbox"
				value="light"
				class="toggle theme-controller bg-base-content row-start-1 col-start-1 col-span-2"
			/>
			<svg
				class="col-start-1 row-start-1 stroke-base-100 fill-base-100"
				xmlns="http://www.w3.org/2000/svg"
				width="14"
				height="14"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
				><circle cx="12" cy="12" r="5" /><path
					d="M12 1v2M12 21v2M4.2 4.2l1.4 1.4M18.4 18.4l1.4 1.4M1 12h2M21 12h2M4.2 19.8l1.4-1.4M18.4 5.6l1.4-1.4"
				/></svg
			>
			<svg
				class="col-start-2 row-start-1 stroke-base-100 fill-base-100"
				xmlns="http://www.w3.org/2000/svg"
				width="14"
				height="14"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
				><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path></svg
			>
		</label>
	</div>
</div>
