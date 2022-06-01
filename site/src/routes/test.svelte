<script>
	import roggle from "$lib/services/roggle";
	import cv from "$lib/services/cv";
	import { onMount } from "svelte";

	const maxVideoSize = 200;

	let board;
	let solving = false;
	let solutions;
	let error;
	let wasmSupported = true;

	let canvasEl;
	let videoEl;
	const canvas = {
		width: 480,
		height: 480
	};

	let rows = 4;
	let cols = 4;
	let boxes = [
		["", "", "", ""],
		["", "", "", ""],
		["", "", "", ""],
		["", "", "", ""]
	];

	// $: console.log(boxes);
	// $: board = [].concat(boxes.map((row) => row.join("") + " ")).join("");
	// $: console.log(board);

	function rebuildBoard() {
		boxes = Array(rows).fill(Array(cols).fill(""));
	}

	function isValid(val, i, j) {
		console.log(i, j);
		if ((val.keyCode >= 65 && val.keyCode <= 90) || (val.keyCode >= 97 && val.keyCode <= 122)) {
			boxes[i][j] = val.key;
			return true;
		}
		return false;
	}

	async function loadOpenCV() {
		console.log("Loading OpenCV...");
		await cv.load();
		console.log("Done loading OpenCV!");
	}

	async function processImage() {
		const canvasCtx = canvasEl.getContext("2d");

		var memCanvas = document.createElement("canvas");
		memCanvas.height = canvas.height;
		memCanvas.width = canvas.width;
		var memCanvasCtx = memCanvas.getContext("2d");
		memCanvasCtx.drawImage(videoEl, 0, 0, canvas.width, canvas.height);

		const image = memCanvasCtx.getImageData(0, 0, canvas.width, canvas.height);
		const processedImage = await cv.imageProcessing(image);

		canvasCtx.putImageData(processedImage.data.payload, 0, 0);
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

		await cv.load();

		try {
			const stream = await navigator.mediaDevices.getUserMedia({ video: true });
			videoEl.srcObject = stream;
			videoEl.play();
		} catch (e) {
			console.error(e, "camera access denied");
		}
	});
</script>

<main>
	<header>
		<img src="/images/roggle.png" alt="roggle logo" width="200" />
	</header>

	<div class="cam-canvas">
		<video bind:this={videoEl} width={canvas.width} height={canvas.height} />
		<canvas bind:this={canvasEl} width={canvas.width} height={canvas.height} />
	</div>

	<!-- <button on:click={loadOpenCV}>load opencv</button> -->
	<button on:click={processImage}>process image</button>

	<form on:submit|preventDefault={solveBoard}>
		<p>Please input the N x M board as rows separated by spaces. For qu tile just put q.</p>
		<input type="text" style:margin-bottom="20px" bind:value={board} />

		<!-- <div class="board">
			<div class="grid">
				{#each boxes as row, i}
					<div class="row">
						{#each row as column, j}
							<input
								class="grid-input"
								type="text"
								bind:value={boxes[i][j]}
								on:keydown|preventDefault={(e) => isValid(e, i, j)}
							/>
						{/each}
					</div>
				{/each}
				<div class="row-buttons">
					<button
						type="button"
						on:click={() => {
							rows = rows - 1;
							rebuildBoard();
						}}>-</button
					>
					<button
						type="button"
						on:click={() => {
							rows = rows + 1;
							rebuildBoard();
						}}>+</button
					>
				</div>
			</div>
			<div class="col-buttons">
				<button
					type="button"
					on:click={() => {
						cols = cols - 1;
						rebuildBoard();
					}}>-</button
				>
				<button
					type="button"
					on:click={() => {
						cols = cols + 1;
						rebuildBoard();
					}}>+</button
				>
			</div>
		</div> -->

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
		justify-content: space-evenly;
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

	.row-buttons {
		display: flex;
		flex-direction: row;
	}

	.row-buttons button {
		width: 100%;
	}

	.col-buttons {
		margin-left: 5px;
		display: flex;
		flex-direction: column;
	}

	.col-buttons button {
		height: 100%;
	}

	input.grid-input {
		max-width: 1rem;
	}

	.board {
		display: flex;
		flex-direction: row;
		margin-bottom: 20px;
	}

	button.right-plus {
		margin-left: 5px;
	}

	input.grid-input:not(:last-child) {
		margin: 0 5px 5px 0;
	}
</style>
