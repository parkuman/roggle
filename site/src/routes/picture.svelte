<script>
	import imageProcessing from "$lib/services/imageProcessing";
	import Camera from "$lib/Camera.svelte";
	import { onMount } from "svelte";

	let board;
	let error;
	let extractingBoard = false;
	let loadingWorker = true;
	let pictureTaken = false;

	let inputImgEl;
	const canvas = {
		width: 480,
		height: 480,
	};

	async function extractBoard() {
		pictureTaken = false;
		extractingBoard = true;
		error = null;

		var memCanvas = document.createElement("canvas");
		memCanvas.height = canvas.height;
		memCanvas.width = canvas.width;
		var memCanvasCtx = memCanvas.getContext("2d");
		memCanvasCtx.drawImage(inputImgEl, 0, 0, canvas.width, canvas.height);

		const image = memCanvasCtx.getImageData(0, 0, canvas.width, canvas.height);
		const processedImage = await imageProcessing.imageProcessing(image);
		const payload = processedImage.data.payload;

		extractingBoard = false;
		if (typeof payload === "string" && payload.toLowerCase().includes("error")) {
			error = processedImage.data.payload;
			return;
		}

		board = payload;
		pictureTaken = true;
	}

	function dismissConfirmScreen() {
		pictureTaken = false;
	}

	onMount(async () => {
		loadingWorker = true;
		await imageProcessing.load();
		loadingWorker = false;
	});

	$: isLoading = extractingBoard || loadingWorker;
</script>

<main>
	<div class="cam-wrapper" style="width: {canvas.width}px; height: {canvas.height}px; ">
		<Camera bind:context={inputImgEl} width={canvas.width} height={canvas.height} />
	</div>
	<button
		type="button"
		on:click={extractBoard}
		id="capture-btn"
		disabled={isLoading}
		style:cursor={isLoading ? "not-allowed" : "pointer"}
	>
		<svg width="120" viewBox="0 0 120 120" fill="none" xmlns="http://www.w3.org/2000/svg">
			<circle cx="60" cy="60" r="50" fill="white" />
			<circle cx="60" cy="60" r="58.5" stroke="white" stroke-width="3" />
		</svg>
	</button>

	{#if pictureTaken}
		<div class="confirm-board">
			<h1>Is this your board?</h1>
			<input disabled type="text" style:margin-bottom="20px" bind:value={board} />
			<button class="btn-big" on:click={dismissConfirmScreen}>No, Retake</button>
			<a href={`/solve?board=${board.trim().replace(" ", "%20")}`}>
				<button class="btn-big" style:background-color="green" on:click={dismissConfirmScreen}
					>Close Enough!</button
				></a
			>
		</div>
	{/if}
	{#if error}
		<p>{error}</p>
	{/if}
</main>

<style>
	.cam-wrapper {
		overflow: hidden;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		max-height: 66vh;
	}

	#capture-btn {
		margin: 20px 0;
		cursor: pointer;
		background: none;
		border: none;
	}

	#capture-btn svg {
		display: relative;
		max-width: 3rem;
	}

	#capture-btn:hover svg circle:first-of-type {
		fill: #d7d7d7;
	}

	.confirm-board {
		width: 100%;
		height: 100%;
		position: absolute;
		top: var(--header-height);
		left: 0;
		display: flex;
		flex-direction: column;
		align-items: center;
		background-color: var(--color-bg);
	}
</style>
