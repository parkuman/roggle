<script>
	import solver from "$lib/services/solver";
	import BoardGrid from "$lib/BoardGrid.svelte";
	import { page } from "$app/stores";
import Head from "../lib/Head.svelte";

	let board = $page.url.searchParams.get("board")
		? $page.url.searchParams.get("board").toString().replace("%20", " ")
		: "";

	const splitBoardStr = board.split(" ");
	let rows = board.length > 1 ? splitBoardStr.length : 4;
	let cols = board.length > 1 ? splitBoardStr[0].length : 4;
	let solutions;
	let error;
	let solving = false;
	let validBoard;
	let totalPts = 0;

	async function solveBoard() {
		solutions = null;
		solving = true;
		error = null;
		totalPts = 0;

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

						totalPts += points;

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

<Head title="Solve Board | Roggle" />
<main>
	<BoardGrid bind:board bind:validBoard bind:rows bind:cols />

	<form on:submit|preventDefault={solveBoard}>
		<p>
			Alternatively you can input the N x M board below as rows separated by spaces. For qu tile
			just put q.
		</p>
		<input type="text" style:margin-bottom="20px" bind:value={board} />
		<button
			class="btn-big"
			disabled={!canSolve}
			style:cursor={canSolve ? "pointer" : "not-allowed"}
			style:border-color={!canSolve ? "red" : ""}
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
				<p>total points: {totalPts}</p>
				<br />
				{#each solutions as { word, points }}
					<tr>
						<td
							style:background={() => {
								if (points === 2) return "lightgreen";
								if (points === 3) return "lightblue";
								if (points === 5) return "lightyellow";
								if (points === 11) return "lightred";
							}}
						>
							<p>{word} ({points} pt{points > 1 ? "s" : ""})</p>
						</td>
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
		text-align: center;
		margin-top: 20px;
	}

	form > p {
		max-width: 90%;
	}

	input {
		width: 90%;
	}

	tbody {
		margin-top: 50px;
	}
</style>
