<script>
	import { onMount } from "svelte";

	export let imageData;
	export let width;
	export let height;
	let canvasEl;

	onMount(() => {
		const canvasCtx = canvasEl.getContext("2d");
		canvasCtx.putImageData(imageData, 0, 0);
	});
</script>

<canvas bind:this={canvasEl} {width} {height} />
