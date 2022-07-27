<script>
	export let board = "";
	export let validBoard;

	let rows = 2;
	let cols = 2;

	// $: splitBoard = board.split(" ");
	// $: letterGrid = getLetterGrid(splitBoard);
	// $: validBoard = checkValidBoard(splitBoard);
	$: gridEls = generateGridArray(rows, cols);
	$: outputBoard = generateBoardFromGrid(gridEls);
	$: console.log(gridEls);

	function generateGridArray(r, c) {
		const grid = [];
		Array.from(Array(r)).forEach((_) => {
			grid.push(
				Array.from(Array(c)).map((element) => ({
					element,
					value: "",
				})),
			);
		});
		return grid;
	}

	function generateBoardFromGrid(grid) {
		let boardString = "";
		grid.forEach((row, i) => {
			// iterate over each sub array, adding each value
			row.forEach(({ value }) => {
				boardString += value;
			});

			// only add space if its not the last row
			if (i !== grid.length - 1) boardString += " ";
		});
		return boardString;
	}

	// function getLetterGrid(boardRows) {
	// 	const grid = [];
	// 	if (!boardRows || !boardRows.length || boardRows.length === 0) return grid;
	// 	boardRows.forEach((row) => {
	// 		grid.push(Array.from(row));
	// 	});
	// 	return grid;
	// }

	// function checkValidBoard(boardRows) {
	// 	const rowLen = boardRows[0].length;
	// 	if (rowLen === 0) return false;
	// 	for (let i = 0; i < boardRows.length; i++) {
	// 		if (boardRows[i].length != rowLen) {
	// 			return false;
	// 		}
	// 	}
	// 	return true;
	// }

	function handleGridInput(val, i, j) {
		const width = gridEls[i].length,
			height = gridEls.length,
			prevCol = j - 1,
			nextCol = j + 1,
			prevRow = i - 1,
			nextRow = i + 1,
			left = 37,
			up = 38,
			right = 39,
			down = 40,
			tab = 9;

		// Preserve tab functionality,
		// loop to first input of next row
		// when end of row is reached or to
		// first row and column when end of
		// matrix is reached
		if (val.keyCode == tab) {
			nextCol != width
				? gridEls[i][nextCol].element.focus()
				: i != height - 1
				? gridEls[nextRow][0].element.focus()
				: gridEls[0][0].element.focus();
			return;
		}

		// Loop around single row with right and left arrows
		if (val.keyCode == right) {
			nextCol != width ? gridEls[i][nextCol].element.focus() : gridEls[i][0].element.focus();
			return;
		}
		if (val.keyCode == left) {
			j != 0 ? gridEls[i][prevCol].element.focus() : gridEls[i][width - 1].element.focus();
			return;
		}

		// loop around single column with up and down arrows
		if (val.keyCode == up) {
			i != 0 ? gridEls[prevRow][j].element.focus() : gridEls[height - 1][j].element.focus();
			return;
		}

		if (val.keyCode == down) {
			i != height - 1 ? gridEls[nextRow][j].element.focus() : gridEls[0][j].element.focus();
			return;
		}

		if ((val.keyCode >= 65 && val.keyCode <= 90) || (val.keyCode >= 97 && val.keyCode <= 122)) {
			gridEls[i][j].value = val.key;
			nextCol != width
				? gridEls[i][nextCol].element.focus()
				: i != height - 1
				? gridEls[nextRow][0].element.focus()
				: gridEls[0][0].element.focus();
			return true;
		}
		return false;
	}

	function addRow() {
		rows = rows + 1;
	}

	function removeRow() {
		rows = rows - 1;
	}

	function addCol() {
		cols = cols + 1;
	}

	function removeCol() {
		cols = cols - 1;
	}

	function hello(node, idx) {
		gridEls[idx.i][idx.j].element = node;
	}
</script>

<div class="grid-wrapper">
	<div class="grid">
		{#each gridEls as gridRow, i}
			<div class="row">
				{#each gridRow as gridCol, j}
					<input
						class="column"
						type="text"
						bind:value={gridEls[i][j].value}
						use:hello={{ i, j }}
						on:keydown|preventDefault={(e) => handleGridInput(e, i, j)}
					/>
				{/each}
			</div>
		{/each}
		<div class="row-controls">
			<button on:click={addRow}>+</button>
			<button on:click={removeRow}>-</button>
		</div>
	</div>

	<div class="col-controls" style="--num-rows: {rows}">
		<button on:click={addCol}>+</button>
		<button on:click={removeCol}>-</button>
	</div>
</div>

<br />

<input bind:value={outputBoard} type="text" style:width="300px" />

<style>
	:root {
		--grid-margin: 10px;
		--input-shape: 40px;
	}
	button {
		width: 100%;
	}

	input {
		width: var(--input-shape);
		height: var(--input-shape);
		margin: 0;
		border: none;
	}

	.grid-wrapper {
		display: flex;
		flex-direction: row;
		justify-content: center;
	}

	.grid {
		margin-right: var(--grid-margin);
	}

	.row {
		margin-bottom: var(--grid-margin);
		display: flex;
		flex-direction: row;
	}

	.column {
		margin-right: var(--grid-margin);
	}

	.column:last-of-type {
		margin: 0;
	}

	.col-controls {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 100%;
		width: var(--input-shape);
		float: right;
	}

	.col-controls button {
		/* get half the height of the grid not including the last margin */
		height: calc(
			(var(--num-rows) * (var(--input-shape) + var(--grid-margin)) - var(--grid-margin)) / 2
		);
	}

	.row-controls {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: center;
		width: 100%;
	}

	.row-controls button {
		height: var(--input-shape);
	}
</style>
