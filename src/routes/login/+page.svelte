<script>
	import LoginCard from '$lib/components/LoginCard.svelte';

	let successfulLogIn = true;
	let closeToast = false;

	function getMousePosition() {
		window.addEventListener('mousemove', (event) => {
			const eyesLeft = document.querySelector('.eyesLeft');
			if (eyesLeft) {
				const mouseX = event.clientX;
				const mouseY = event.clientY;
				const y = 5 + (mouseY * 2) / 100;
				const x = 75 - (mouseX * 4) / 100;
				eyesLeft.style.top = `${y}%`;
				eyesLeft.style.right = `${x}%`;
			}
			const eyesRight = document.querySelector('.eyesRight');
			if (eyesRight) {
				const mouseX = event.clientX;
				const mouseY = event.clientY;
				const y = 5 + (mouseY * 2) / 100;
				const x = 55 - (mouseX * 4) / 100;
				eyesRight.style.top = `${y}%`;
				eyesRight.style.right = `${x}%`;
			}

			const eyeR = document.querySelector('.eyeR');
			if (eyeR) {
				const mouseX = event.clientX;
				const mouseY = event.clientY;
				const y = 5 + (mouseY * 2) / 100;
				const x = 55 - (mouseX * 4) / 100;
				eyeR.style.top = `${y}%`;
				eyeR.style.right = `${x}%`;
			}

			const eyeL = document.querySelector('.eyeL');
			if (eyeL) {
				const mouseX = event.clientX;
				const mouseY = event.clientY;
				const y = 5 + (mouseY * 2) / 100;
				const x = 55 - (mouseX * 4) / 100;
				eyeL.style.top = `${y}%`;
				eyeL.style.right = `${x}%`;
			}
		});
	}

	getMousePosition();
</script>

{#if !successfulLogIn && !closeToast}
	<div class="flex justify-center mb-4">
		<div class="max-w-6xl alert alert-error flex justify-between">
			<span>Invalid e-mail or password.</span>
			<button
				class="btn btn-square btn-outline"
				style="color: black;"
				on:click={() => {
					closeToast = true;
				}}
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="h-6 w-6"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
					><path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M6 18L18 6M6 6l12 12"
					/></svg
				>
			</button>
		</div>
	</div>
{/if}
<div class="flex">
	<LoginCard bind:successfulLogIn bind:closeToast />
	<div class="hero min-h-screen bg-base-200">
		<div class="hero-content flex-col lg:flex-row">
			<div class="test">
				<div class="eyesLeft">
					<div class="eyeL"></div>
				</div>
				<div class="eyesRight">
					<div class="eyeR"></div>
				</div>
			</div>
			<div>
				<h1 class="text-5xl font-bold">Audit Trail!</h1>
				<p class="py-6">
					Provident cupiditate voluptatem et in. Quaerat fugiat ut assumenda excepturi
					exercitationem quasi. In deleniti eaque aut repudiandae et a id nisi.
				</p>
				<button class="btn btn-primary">Get Started</button>
			</div>
		</div>
	</div>
</div>

<style>
	.test {
		width: 10vw;
		height: 500px;
		border: solid;
		border-color: cyan;
		position: relative;
	}

	.eyesLeft {
		border-radius: 50%;
		height: 30px;
		width: 30px;
		border: solid;
		border-color: white;
		background-color: white;
		position: absolute;
	}

	.eyesRight {
		border-radius: 50%;
		height: 30px;
		width: 30px;
		border: solid;
		border-color: white;
		background-color: white;
		position: absolute;
	}

	.eyeR {
		border-radius: 50%;
		height: 15px;
		width: 15px;
		border: solid;
		border-color: black;
		background-color: black;
		position: absolute;
	}

	.eyeL {
		border-radius: 50%;
		height: 15px;
		width: 15px;
		border: solid;
		border-color: black;
		background-color: black;
		position: absolute;
	}
</style>
