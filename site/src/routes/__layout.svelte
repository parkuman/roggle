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

<style>
</style>
