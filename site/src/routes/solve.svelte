<script>
	import solver from "$lib/services/solver";
	import BoardGrid from "$lib/BoardGrid.svelte";

	let board;
	let solutions;
	let error;
	let solving = false;
	let validBoard;

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

	$: canSolve = validBoard && !solving;
</script>

<main>
	<BoardGrid {board} bind:validBoard />

	<form on:submit|preventDefault={solveBoard}>
		<p>Alternatively you can input the N x M board below as rows separated by spaces. For qu tile just put q.</p>
		<input type="text" style:margin-bottom="20px" bind:value={board} />
		<button
			class="btn-big"
			disabled={!canSolve}
			style:cursor={canSolve ? "pointer" : "not-allowed"}
			style:background={canSolve ? "purple" : "black"}
			type="submit">{solving ? "Solving..." : "Solve"}</button
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
	form {
		display: flex;
		flex-direction: column;
		align-items: center;
	}
</style>
