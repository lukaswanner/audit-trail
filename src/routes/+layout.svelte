<script>
	import '../app.css';
	import Navbar from '$lib/layout/navigation/Navbar.svelte';
</script>

<div class="flex flex-col h-screen bg-base-300">
	<Navbar />
	<div class="flex flex-1 flex-col p-4">
		<slot />
	</div>
</div>
