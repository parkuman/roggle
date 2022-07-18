<script>
	import { onMount } from "svelte";

	import solver from "$lib/services/solver";
	import imageProcessing from "$lib/services/imageProcessing";
	import Camera from "$lib/Camera.svelte";
	import DebugImg from "$lib/DebugImg.svelte";

	let board;
	let solving = false;
	let solutions;
	let error;
	let wasmSupported = true;
	let loadingWorker = true;
	let outputLog = []; // array containing all the outputted images from the worker

	let letterCanvases = Array.from(Array(16));
	let letterCanvasDimensions = Array.from(Array(16)).map((element) => ({
		width: 100,
		height: 100,
	}));

	let canvasEl;
	let inputImgEl;
	let useCamera = false;
	const canvas = {
		width: 480,
		height: 480,
	};

	async function debugProcessImage() {
		outputLog = [];

		// override the usual image processing class' message handling
		imageProcessing.worker.onmessage = (e) => {
			console.log("[DEBUG] Received a message from worker: ", e);

			if (e.data.msg === "imageProcessing") {
				board = e.data.payload;
			}

			outputLog = [
				...outputLog,
				{ msg: e.data.msg, payload: e.data.payload, imageData: e.data.imageData },
			];
		};

		var memCanvas = document.createElement("canvas");
		memCanvas.height = canvas.height;
		memCanvas.width = canvas.width;
		var memCanvasCtx = memCanvas.getContext("2d");
		memCanvasCtx.drawImage(inputImgEl, 0, 0, canvas.width, canvas.height);

		const image = memCanvasCtx.getImageData(0, 0, canvas.width, canvas.height);
		imageProcessing.worker.postMessage({ msg: "imageProcessing", data: image, debug: true });
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
	});
</script>

<!-- <header>
	<img src="/images/roggle.png" alt="roggle logo" width="200" />
</header> -->
<main class="split">
	<section class="left">
		<div class="cam-canvas">
			{#if useCamera}
				<Camera bind:context={inputImgEl} width={canvas.width} height={canvas.height} />
			{:else}
				<img
					bind:this={inputImgEl}
					src="/images/board5.jpg"
					width={canvas.width}
					height={canvas.height}
					alt="a boggle board"
				/>
			{/if}
		</div>
		<label>
			Use Camera
			<input type="checkbox" bind:checked={useCamera} />
		</label>

		<button on:click={debugProcessImage} disabled={loadingWorker}>process image</button>

		<form on:submit|preventDefault={solveBoard}>
			<p>Please input the N x M board as rows separated by spaces. For qu tile just put q.</p>
			<input type="text" style:margin-bottom="20px" bind:value={board} />
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
		{#each outputLog as log}
			<div class="log-item">
				<p><strong>{log.msg}</strong></p>
				{#if log.payload}
					<p>{log.payload}</p>
				{/if}
				{#if log.imageData}
					<DebugImg
						imageData={log.imageData}
						width={log.imageData.width}
						height={log.imageData.height}
					/>
				{/if}
			</div>
		{/each}
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

	.left,
	.right {
		height: 100%;
		overflow: auto;
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.left {
		width: 40%;
		background-color: rgb(227, 227, 227);
	}

	.right {
		width: 60%;
		margin: 0 10px;
	}

	.log-item {
		margin-bottom: 50px;
	}
</style>
