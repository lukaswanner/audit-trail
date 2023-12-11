<script lang="ts">
	import { login } from '$lib/api/auth';
	import { project, projects } from '$lib/stores/project';
</script>

<div class="flex flex-col items-center gap-2">
	<button
		on:click={async () => {
			await login({
				email: 'lukas.wanner@google.com',
				password: 'some-secure-password',
				rememberMe: false
			});
		}}
		class="btn btn-ghost h-16 w-16 rounded-full fill-base-content transition-colors hover:fill-primary"
	>
		<a href="/">
			<svg xmlns="http://www.w3.org/2000/svg" class=" h-8 w-8" viewBox="0 0 576 512"
				><!--!Font Awesome Free 6.5.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2023 Fonticons, Inc.--><path
					d="M575.8 255.5c0 18-15 32.1-32 32.1h-32l.7 160.2c0 2.7-.2 5.4-.5 8.1V472c0 22.1-17.9 40-40 40H456c-1.1 0-2.2 0-3.3-.1c-1.4 .1-2.8 .1-4.2 .1H416 392c-22.1 0-40-17.9-40-40V448 384c0-17.7-14.3-32-32-32H256c-17.7 0-32 14.3-32 32v64 24c0 22.1-17.9 40-40 40H160 128.1c-1.5 0-3-.1-4.5-.2c-1.2 .1-2.4 .2-3.6 .2H104c-22.1 0-40-17.9-40-40V360c0-.9 0-1.9 .1-2.8V287.6H32c-18 0-32-14-32-32.1c0-9 3-17 10-24L266.4 8c7-7 15-8 22-8s15 2 21 7L564.8 231.5c8 7 12 15 11 24z"
				/></svg
			>
		</a>
	</button>
	{#each $projects as project_item}
		<button
			on:click={() => project.set(project_item)}
			data-active={project_item === $project}
			class="btn btn-circle flex h-12 w-12 items-center justify-center overflow-hidden rounded-full border-transparent bg-transparent hover:border-transparent hover:bg-transparent data-[active=true]:border-2
		data-[active=true]:border-accent"
		>
			<p class="text-ellipsis text-lg font-bold">
				{project_item.title.slice(0, 2)}
			</p>
		</button>
	{/each}
	<div class="divider my-0 w-1/2 self-center" />
	<button
		class="btn btn-ghost h-12 w-12 rounded-full fill-base-content transition-colors hover:fill-primary"
	>
		<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" viewBox="0 0 448 512"
			><!--!Font Awesome Free 6.5.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2023 Fonticons, Inc.--><path
				d="M256 80c0-17.7-14.3-32-32-32s-32 14.3-32 32V224H48c-17.7 0-32 14.3-32 32s14.3 32 32 32H192V432c0 17.7 14.3 32 32 32s32-14.3 32-32V288H400c17.7 0 32-14.3 32-32s-14.3-32-32-32H256V80z"
			/></svg
		>
	</button>
</div>
