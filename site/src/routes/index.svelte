<script>
	import roggle from "$lib/services/roggle";
	import imageProcessing from "$lib/services/imageProcessing";
	import { onMount } from "svelte";

	let board;
	let solving = false;
	let extractingBoard = false;
	let solutions;
	let error;
	let wasmSupported = true;
	let loadingWorker = true;

	let videoEl;
	const canvas = {
		width: 480,
		height: 480,
	};

	async function extractBoard() {
		extractingBoard = true;
		error = null;

		var memCanvas = document.createElement("canvas");
		memCanvas.height = canvas.height;
		memCanvas.width = canvas.width;
		var memCanvasCtx = memCanvas.getContext("2d");
		memCanvasCtx.drawImage(videoEl, 0, 0, canvas.width, canvas.height);

		const image = memCanvasCtx.getImageData(0, 0, canvas.width, canvas.height);
		const processedImage = await imageProcessing.imageProcessing(image);
		const payload = processedImage.data.payload;

		if (typeof payload === "string" && payload.toLowerCase().includes("error")) {
			error = processedImage.data.payload;
			return;
		}

		extractingBoard = false;
		board = payload;
	}

	async function solveBoard() {
		solutions = null;
		solving = true;
		error = null;
		// TODO: validate board
		console.log("Solving...");
		const start = new Date();

		try {
			const res = await roggle.solve(board);

			if (res.data.solutions === "") {
				solutions = [];
			} else {
				solutions = res.data.solutions
					.split(", ")
					.sort()
					.map((word) => {
						let points;
						switch (word.length) {
							case 0:
								points = 0;
								word = "";
								break;
							case 3:
							case 4:
								points = 1;
								break;
							case 5:
								points = 2;
								break;
							case 6:
								points = 3;
								break;
							case 7:
								points = 5;
								break;
							default: // 8 or higher
								points = 11;
						}

						return { word, points };
					});
			}
		} catch (err) {
			console.error("ERROR:", err);
			error = err;
		}

		const end = new Date();
		console.log("completed in", (end - start) / 1000, "seconds");

		solving = false;
	}

	onMount(async () => {
		wasmSupported = !(typeof WebAssembly === "undefined");
		if (!wasmSupported) {
			error = "Sorry! Your browser does not support the features needed to run Roggle";
		}

		try {
			const stream = await navigator.mediaDevices.getUserMedia({ video: true });
			videoEl.srcObject = stream;
			videoEl.play();
		} catch (e) {
			console.error(e, "camera access denied");
		}

		loadingWorker = true;
		await imageProcessing.load();
		loadingWorker = false;
	});
</script>

<main>
	<header>
		<img src="/images/roggle.png" alt="roggle logo" width="200" />
	</header>

	<div class="cam-canvas">
		<video bind:this={videoEl} width={canvas.width} height={canvas.height} />
		<!-- <img bind:this={videoEl} src="/images/board1.jpg" width={canvas.width} height={canvas.height} /> -->
	</div>

	<form on:submit|preventDefault={solveBoard}>
		<p>Please input the N x M board as rows separated by spaces. For qu tile just put q.</p>
		<input type="text" style:margin-bottom="20px" bind:value={board} />
		<button
			on:click={extractBoard}
			type="button"
			disabled={extractingBoard || loadingWorker || !wasmSupported}>Extract board from image</button
		>
		<button disabled={solving || !wasmSupported} type="submit"
			>{solving ? "Solving..." : "Solve"}</button
		>
	</form>

	{#if error}
		<p style:color="red">{error}</p>
	{/if}
	{#if !solving && solutions}
		<tbody>
			{#if solutions.length === 0}
				<p>no solutions!</p>
			{:else}
				{#each solutions as { word, points }}
					<tr>
						<p>{word} ({points} pt{points > 1 ? "s" : ""})</p>
					</tr>
				{/each}
			{/if}
		</tbody>
	{/if}
</main>

<style>
	.cam-canvas {
		width: 100%;
		display: flex;
		flex-direction: row;
		justify-content: center;
		align-items: center;
	}

	button {
		cursor: pointer;
	}
	main {
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	form {
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	header {
		margin-bottom: 100px;
	}
</style>
