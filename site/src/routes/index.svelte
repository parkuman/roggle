<script>
	import { onMount } from "svelte";
	import init, { solve } from "../../node_modules/roggle/roggle";

	let board;
	let solving = false;
	let solutions;
	let error;

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
	$: console.log(board);

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

	function solveBoard() {
		try {
			solutions = null;
			solving = true;
			// TODO: validate board
			console.log("Solving...");
			const start = new Date();
			solutions = solve(board)
				.split(", ")
				.sort()
				.map((word) => {
					let points;
					switch (word.length) {
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
			const end = new Date();
			console.log("completed in", (end - start) / 1000, "seconds");
		} catch (e) {
			error =
				"Error while solving, please make sure the board is N x M and contains only english alphabet characters";
			console.error(e, "error while solving!");
		}
		solving = false;
	}

	onMount(async () => {
		await init();
		console.log("Done loading roggle wasm module");
	});
</script>

<main>
	<header>
		<img src="https://prowe.ca/images/projects/roggle/roggle.png" alt="roggle logo" width="200" />
	</header>
	<form on:submit|preventDefault={solveBoard}>
		<pre>Please input the N x M board as rows separated by spaces. For qu tile just put q.</pre>
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

		<button disabled={solving} type="submit">{solving ? "Solving..." : "Solve"}</button>
	</form>

	{#if error}
		<p style:color="red">{error}</p>
	{/if}
	{#if solving}
		<p>solving...</p>
	{/if}
	{#if !solving && solutions}
		<tbody>
			{#each solutions as { word, points }}
				<tr>
					<p>{word} ({points} pt{points > 1 ? "s" : ""})</p>
				</tr>
			{/each}
		</tbody>
	{/if}
</main>

<style>
	button {
		cursor: pointer;
	}
	main {
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
