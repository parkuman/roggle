<script>
	import solver from "$lib/services/solver";
	import imageProcessing from "$lib/services/imageProcessing";
	import { onMount } from "svelte";

	const maxVideoSize = 200;

	let board;
	let solving = false;
	let extractingBoard = false;
	let solutions;
	let error;
	let wasmSupported = true;
	let loadingWorker = true;

	let letterCanvases = Array.from(Array(16));
	let letterCanvasDimensions = Array.from(Array(16)).map((element) => ({
		width: 100,
		height: 100,
	}));

	let canvasEl;
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
		const processedImage = await imageProcessing.imageProcessing(image, true);
		const payload = processedImage.data.payload;

		if (typeof payload === "string" && payload.toLowerCase().includes("error")) {
			error = processedImage.data.payload;
			return;
		}

		extractingBoard = false;
		board = payload;
	}

	async function processImage() {
		const canvasCtx = canvasEl.getContext("2d");

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

		canvasCtx.putImageData(payload, 0, 0);
	}

	async function getLetters() {
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

		payload.forEach((letter, idx) => {
			letterCanvasDimensions[idx].width = letter.width;
			letterCanvasDimensions[idx].height = letter.height;
			const canvasCtx = letterCanvases[idx].getContext("2d");
			canvasCtx.putImageData(letter, 0, 0);
		});
	}

	async function solveBoard() {
		solutions = null;
		solving = true;
		error = null;
		// TODO: validate board
		console.log("Solving...");
		const start = new Date();

		try {
			const res = await solver.solve(board);

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

		loadingWorker = true;
		await imageProcessing.load();
		loadingWorker = false;

		// try {
		// 	const stream = await navigator.mediaDevices.getUserMedia({ video: true });
		// 	videoEl.srcObject = stream;
		// 	videoEl.play();
		// } catch (e) {
		// 	console.error(e, "camera access denied");
		// }
	});
</script>

<!-- <header>
	<img src="/images/roggle.png" alt="roggle logo" width="200" />
</header> -->
<main class="split">
	<section class="left">
		left:

		<div class="cam-canvas">
			<!-- <video bind:this={videoEl} width={canvas.width} height={canvas.height} /> -->
			<img
				bind:this={videoEl}
				src="/images/board5.jpg"
				width={canvas.width}
				height={canvas.height}
			/>
		</div>

		<button on:click={processImage} disabled={loadingWorker}>process image</button>

		<div class="letter-canvas">
			{#each Array.from(Array(16).keys()) as idx}
				<canvas
					bind:this={letterCanvases[idx]}
					width={letterCanvasDimensions[idx].width}
					height={letterCanvasDimensions[idx].height}
				/>
				{#if (idx + 1) % 4 === 0}
					<br />
				{/if}
			{/each}
		</div>
		<button on:click={getLetters} disabled={loadingWorker}>get letters</button>

		<form on:submit|preventDefault={solveBoard}>
			<p>Please input the N x M board as rows separated by spaces. For qu tile just put q.</p>
			<input type="text" style:margin-bottom="20px" bind:value={board} />
			<button
				on:click={extractBoard}
				type="button"
				disabled={extractingBoard || loadingWorker || !wasmSupported}
				>Extract board from image</button
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
	</section>
	<section class="right">
		right:
		<canvas bind:this={canvasEl} width={canvas.width} height={canvas.height} />
	</section>
</main>

<style>

	.split {
		display: flex;
		flex-direction: row;
		height: 100vh;
		width: 100vw;
		justify-content: space-between;
		align-items: center;
	}

	.left {
		width: 40%;
		height: 100%;
		overflow: auto;
		display: flex; 
		flex-direction: column;
		align-items: center;
		background-color: rgb(227, 227, 227);
		box-shadow: 3px 0px 3px rgba(0, 0, 0, 0.292)
	}

	.right {
		width: 60%;
		height: 100%;
		overflow: auto;

	}


	.cam-canvas {
		width: 95%;
	}
</style>
