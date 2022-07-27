<script>
	export let board;
	export let validBoard;
	export let rows = 4;
	export let cols = 4;

	let modifiedGrid = false;

	grid = generateGridArray(rows, cols);
	$: prevGrid = grid;
	$: grid = generateGridArray(rows, cols);
	$: board = generateBoardFromGrid(grid);
	$: validBoard = checkValidBoard(board);

	function generateGridArray(r, c) {
		const tmpGrid = [];
		Array.from(Array(r)).forEach((_, i) => {
			tmpGrid.push(
				Array.from(Array(c)).map((_, j) => {
					if (
						prevGrid !== undefined &&
						prevGrid[i] !== undefined &&
						prevGrid[i][j] !== undefined &&
						prevGrid[i][j].value !== undefined
					) {
						return {
							value: prevGrid[i][j].value,
						};
					}

					return {
						value: "",
					};
				}),
			);
		});

		// if we received a board and its valid, apply it to the grid
		if (!modifiedGrid && board !== undefined && checkValidBoard(board)) {
			const splitBoardStr = board.split(" ");
			rows = splitBoardStr.length;
			cols = splitBoardStr[0].length;

			splitBoardStr.forEach((row, i) => {
				Array.from(row).forEach((letter, j) => {
					tmpGrid[i][j].value = letter;
				});
			});
		}

		return tmpGrid;
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

	function checkValidBoard(boardStr) {
		const splitBoardStr = boardStr.split(" ");
		const rowLen = splitBoardStr[0].length;
		if (rowLen === 0) return false;
		for (let i = 0; i < splitBoardStr.length; i++) {
			if (splitBoardStr[i].length != rowLen) {
				return false;
			}
		}
		return true;
	}

	function handleGridInput(val, i, j) {
		const width = grid[i].length,
			height = grid.length,
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
				? grid[i][nextCol].ref.focus()
				: i != height - 1
				? grid[nextRow][0].ref.focus()
				: grid[0][0].ref.focus();
			return;
		}

		// Loop around single row with right and left arrows
		if (val.keyCode == right) {
			nextCol != width ? grid[i][nextCol].ref.focus() : grid[i][0].ref.focus();
			return;
		}
		if (val.keyCode == left) {
			j != 0 ? grid[i][prevCol].ref.focus() : grid[i][width - 1].ref.focus();
			return;
		}

		// loop around single column with up and down arrows
		if (val.keyCode == up) {
			i != 0 ? grid[prevRow][j].ref.focus() : grid[height - 1][j].ref.focus();
			return;
		}

		if (val.keyCode == down) {
			i != height - 1 ? grid[nextRow][j].ref.focus() : grid[0][j].ref.focus();
			return;
		}

		if ((val.keyCode >= 65 && val.keyCode <= 90) || (val.keyCode >= 97 && val.keyCode <= 122)) {
			grid[i][j].value = val.key;
			nextCol != width
				? grid[i][nextCol].ref.focus()
				: i != height - 1
				? grid[nextRow][0].ref.focus()
				: grid[0][0].ref.focus();
			return true;
		}
		return false;
	}

	function addRow() {
		modifiedGrid = true;
		rows = rows + 1;
	}

	function removeRow() {
		modifiedGrid = true;
		rows = rows - 1;
	}

	function addCol() {
		modifiedGrid = true;
		cols = cols + 1;
	}

	function removeCol() {
		modifiedGrid = true;
		cols = cols - 1;
	}
</script>

<div class="grid-wrapper">
	<div class="grid">
		{#each grid as gridRow, i}
			<div class="row">
				{#each gridRow as gridCol, j (gridCol)}
					<input
						class="column"
						type="text"
						bind:value={gridCol.value}
						bind:this={gridCol.ref}
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
