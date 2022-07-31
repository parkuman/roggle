<script>
	import "../../static/styles/global.css";
	import { onMount } from "svelte";

	let wasmSupported = true;

	onMount(async () => {
		wasmSupported = !(typeof WebAssembly === "undefined");
		if (!wasmSupported) {
			error = "Sorry! Your browser does not support the features needed to run Roggle";
		}
	});
</script>

<header>
	<a href="/">
		<img src="/images/roggle.png" alt="roggle logo" width="100" />
	</a>
</header>
{#if !wasmSupported}
	<h1>Sorry! Your browser doesn't support WebAssembly which Roggle requires to work</h1>
{:else}
	<slot />
{/if}
<footer>
	<p>Want to know what's going on under the hood? Check out the <a href="/debug">debug page.</a></p>
	<p>
		Check out the <a href="https://prowe.ca/blog/wasm-rust-boggle-solver" target="_blank"
			>blog post about how Roggle was made!</a
		>
	</p>
	<p>Created by <a href="https://prowe.ca" target="_blank">Parker Rowe</a></p>
</footer>

<style>
	footer {
		width: 100%;
		border-top: solid 1px rgba(var(--color-primary-rgb), 0.4);
		padding: 50px 0;
		text-align: center;
	}

	footer p {
		margin-bottom: 10px;
	}
</style>
