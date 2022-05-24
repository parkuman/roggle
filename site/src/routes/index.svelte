<script>
	import { onMount } from "svelte";

	import init, { solve } from "../../node_modules/roggle/roggle";

	let board;
	let solving = false;
	let solutions;

	function solveBoard() {
		solutions = null;
		// TODO: validate board
		solving = true;
		const start = new Date();
		console.log("Solving...");
		solutions = solve(board);
		const end = new Date();
		console.log("completed in", (end - start) / 1000, "seconds");
		solving = false;
	}

	onMount(async () => {
		await init();
		console.log("Done loading roggle wasm module");
	});
</script>

<input type="text" bind:value={board} />
<button disabled={solving} on:click={solveBoard}>Solve</button>
{#if !solving && solutions}
	<p>{solutions}</p>
{:else if solving}
	<p>solving...</p>
{/if}
