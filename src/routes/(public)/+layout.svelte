<script>
	import Navbar from '$lib/layout/navigation/Navbar.svelte';
	const list = ['Register', 'Projects', 'Account'];
</script>

<div class="flex h-screen flex-col">
	<Navbar {list} />
	<div class="flex flex-1 flex-col p-4">
		<slot />
	</div>
</div>
