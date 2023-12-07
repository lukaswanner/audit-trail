<script lang="ts">
	import LoginCard from '$lib/components/LoginCard.svelte';

	let successfulLogIn = true;
	let closeToast = false;
	let eyesLeft: HTMLDivElement;
	let eyesRight: HTMLDivElement;
	let eyeR: HTMLDivElement;
	let eyeL: HTMLDivElement;
	let blacknose: HTMLDivElement;
	let eyesLeftRed: HTMLDivElement;
	let eyesRightRed: HTMLDivElement;
	let eyeRRed: HTMLDivElement;
	let eyeLRed: HTMLDivElement;
	let redMouth: HTMLDivElement;

	function getMousePosition(mouseEvent: MouseEvent) {
		const mouseX = mouseEvent.clientX;
		const mouseY = mouseEvent.clientY;

		let y = 5 + (mouseY * 2) / 100;
		let x = 75 - (mouseX * 2) / 100;
		eyesLeft.style.top = `${y}%`;
		eyesLeft.style.right = `${x}%`;

		y = 5 + (mouseY * 2) / 100;
		x = 55 - (mouseX * 2) / 100;
		eyesRight.style.top = `${y}%`;
		eyesRight.style.right = `${x}%`;

		y = 5 + (mouseY * 2) / 100;
		x = 55 - (mouseX * 4) / 100;
		eyeR.style.top = `${y}%`;
		eyeR.style.right = `${x}%`;

		y = 5 + (mouseY * 2) / 100;
		x = 55 - (mouseX * 4) / 100;
		eyeL.style.top = `${y}%`;
		eyeL.style.right = `${x}%`;

		y = 15 + (mouseY * 2) / 100;
		x = 75 - (mouseX * 2) / 100;
		blacknose.style.top = `${y}%`;
		blacknose.style.right = `${x}%`;

		y = 5 + (mouseY * 2) / 100;
		x = 75 - (mouseX * 2) / 100;
		eyesLeftRed.style.top = `${y}%`;
		eyesLeftRed.style.right = `${x}%`;

		y = 5 + (mouseY * 2) / 100;
		x = 55 - (mouseX * 2) / 100;
		eyesRightRed.style.top = `${y}%`;
		eyesRightRed.style.right = `${x}%`;

		y = 5 + (mouseY * 2) / 100;
		x = 55 - (mouseX * 4) / 100;
		eyeRRed.style.top = `${y}%`;
		eyeRRed.style.right = `${x}%`;

		y = 5 + (mouseY * 2) / 100;
		x = 55 - (mouseX * 4) / 100;
		eyeLRed.style.top = `${y}%`;
		eyeLRed.style.right = `${x}%`;

		y = 25 + (mouseY * 4) / 100;
		x = 75 - (mouseX * 4) / 100;
		redMouth.style.top = `${y}%`;
		redMouth.style.right = `${x}%`;
	}
</script>

<svelte:window on:mousemove={getMousePosition} />
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
	<LoginCard {successfulLogIn} {closeToast} />
	<div class="hero min-h-screen bg-base-200">
		<div class="hero-content flex-col items-end gap-0 lg:flex-row">
			<div class="figureBlack">
				<div bind:this={eyesLeft} class="eyesLeft">
					<div bind:this={eyeL} class="eyeL"></div>
				</div>
				<div bind:this={eyesRight} class="eyesRight">
					<div bind:this={eyeR} class="eyeR"></div>
				</div>
				<div bind:this={blacknose} class="nose"></div>
			</div>
			<div class="figureRed mr-4">
				<div bind:this={eyesLeftRed} class="eyesLeftRed">
					<div bind:this={eyeLRed} class="eyeLRed"></div>
				</div>
				<div bind:this={eyesRightRed} class="eyesRightRed">
					<div bind:this={eyeRRed} class="eyeRRed"></div>
				</div>
				<div bind:this={redMouth} class="redMouth"></div>
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
		border-top-left-radius: 5em;
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

	.redMouth {
		position: absolute;
		background-color: black;
		height: 3px;
		width: 30px;
	}

	.figureBlack {
		width: 10vw;
		height: 40vh;
		background-color: black;
		position: relative;
	}

	.nose {
		position: absolute;
		background-color: white;
		height: 40px;
		width: 3px;
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
