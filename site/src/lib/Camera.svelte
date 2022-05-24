<script>
	import { onMount } from "svelte";
	let videoEl;
	let errorMessage;

	onMount(async () => {
		try {
			const stream = await navigator.mediaDevices.getUserMedia({ video: true });
			videoEl.srcObject = stream;
			videoEl.play();
		} catch (e) {
			console.error(e, "camera access denied");
			errorMessage = e;
		}
	});
</script>

<main>
	<!-- svelte-ignore a11y-media-has-caption -->
	<video bind:this={videoEl} width="600" height="480" />

	{#if errorMessage}
		<h2 style="color: red;">{errorMessage}</h2>
	{/if}
</main>
