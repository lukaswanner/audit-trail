<script>
	import Navbar from '$lib/layout/navigation/Navbar.svelte';
	const list = ['Register', 'Projects', 'Account'];
</script>

<div class="flex h-screen flex-col">
	<Navbar {list} />
	<div class="flex h-full flex-col overflow-hidden p-4">
		<slot />
	</div>
</div>
