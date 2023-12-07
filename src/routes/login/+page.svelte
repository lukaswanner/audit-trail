<script>
	import LoginCard from '$lib/components/LoginCard.svelte';

	let successfulLogIn = true;
	let closeToast = false;

	function getMousePosition() {
		if (typeof window !== 'undefined') {
			window.addEventListener('mousemove', (event) => {
				const eyesLeft = document.querySelector('.eyesLeft');
				if (eyesLeft) {
					const mouseX = event.clientX;
					const mouseY = event.clientY;
					const y = 5 + (mouseY * 2) / 100;
					const x = 75 - (mouseX * 2) / 100;
					eyesLeft.style.top = `${y}%`;
					eyesLeft.style.right = `${x}%`;
				}
				const eyesRight = document.querySelector('.eyesRight');
				if (eyesRight) {
					const mouseX = event.clientX;
					const mouseY = event.clientY;
					const y = 5 + (mouseY * 2) / 100;
					const x = 55 - (mouseX * 2) / 100;
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

				const eyesLeftRed = document.querySelector('.eyesLeftRed');
				if (eyesLeft) {
					const mouseX = event.clientX;
					const mouseY = event.clientY;
					const y = 5 + (mouseY * 2) / 100;
					const x = 75 - (mouseX * 2) / 100;
					eyesLeftRed.style.top = `${y}%`;
					eyesLeftRed.style.right = `${x}%`;
				}
				const eyesRightRed = document.querySelector('.eyesRightRed');
				if (eyesRight) {
					const mouseX = event.clientX;
					const mouseY = event.clientY;
					const y = 5 + (mouseY * 2) / 100;
					const x = 55 - (mouseX * 2) / 100;
					eyesRightRed.style.top = `${y}%`;
					eyesRightRed.style.right = `${x}%`;
				}

				const eyeRRed = document.querySelector('.eyeRRed');
				if (eyeRRed) {
					const mouseX = event.clientX;
					const mouseY = event.clientY;
					const y = 5 + (mouseY * 2) / 100;
					const x = 55 - (mouseX * 4) / 100;
					eyeRRed.style.top = `${y}%`;
					eyeRRed.style.right = `${x}%`;
				}

				const eyeLRed = document.querySelector('.eyeLRed');
				if (eyeLRed) {
					const mouseX = event.clientX;
					const mouseY = event.clientY;
					const y = 5 + (mouseY * 2) / 100;
					const x = 55 - (mouseX * 4) / 100;
					eyeLRed.style.top = `${y}%`;
					eyeLRed.style.right = `${x}%`;
				}
			});
		}
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
		<div class="hero-content flex-col items-end gap-0 lg:flex-row">
			<div class="figureBlack">
				<div class="eyesLeft">
					<div class="eyeL"></div>
				</div>
				<div class="eyesRight">
					<div class="eyeR"></div>
				</div>
			</div>
			<div class="figureRed mr-4">
				<div class="eyesLeftRed">
					<div class="eyeLRed"></div>
				</div>
				<div class="eyesRightRed">
					<div class="eyeRRed"></div>
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
	.figureRed {
		width: 14vw;
		height: 25vh;
		background-color: darkred;
		position: relative;
		border-top-right-radius: 5em;
		border-top-left-radius: 5em
	}

	.eyesLeftRed {
		border-radius: 50%;
		height: 3vh;
		width: 3vh;
		border: solid;
		border-color: white;
		background-color: white;
		position: absolute;
	}

	.eyesRightRed {
		border-radius: 50%;
		height: 3vh;
		width: 3vh;
		border: solid;
		border-color: white;
		background-color: white;
		position: absolute;
		margin-left: 30px;
	}

	.eyeRRed {
		border-radius: 50%;
		height: 15px;
		width: 15px;
		border: solid;
		border-color: black;
		background-color: black;
		position: absolute;
	}

	.eyeLRed {
		border-radius: 50%;
		height: 15px;
		width: 15px;
		border: solid;
		border-color: black;
		background-color: black;
		position: absolute;
	}

	.figureBlack {
		width: 10vw;
		height: 40vh;
		background-color: black;
		position: relative;
	}

	.eyesLeft {
		border-radius: 50%;
		height: 3vh;
		width: 3vh;
		border: solid;
		border-color: white;
		background-color: white;
		position: absolute;
	}

	.eyesRight {
		border-radius: 50%;
		height: 3vh;
		width: 3vh;
		border: solid;
		border-color: white;
		background-color: white;
		position: absolute;
		margin-left: 30px;
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
