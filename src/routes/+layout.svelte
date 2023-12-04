<script>
	import '../app.css';
	import Navbar from '$lib/layout/navigation/Navbar.svelte';
	const list = ['Register', 'Projects', 'Account'];
</script>

<div class="flex flex-col h-screen bg-base-300">
	<Navbar {list} />
	<div class="flex flex-1 flex-col p-4">
		<slot />
	</div>
</div>
