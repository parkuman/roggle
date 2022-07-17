<script>
	import { onMount, onDestroy } from "svelte";

	export let width;
	export let height;
	export let context;

	let videoEl;
	let errorMessage;
	let stream;

	onMount(async () => {
		try {
			const videoOptions = {
				audio: false,
				video: {
					facingMode: "environment",
				},
			};
			stream = await navigator.mediaDevices.getUserMedia(videoOptions);
			videoEl.srcObject = stream;
			videoEl.play();
		} catch (e) {
			console.error(e, "camera access denied");
			errorMessage = e;
		}
	});

	onDestroy(() => {
		// stop all input streams (specifically we're just stopping the camera)
		if (stream) {
			stream.getTracks().forEach((track) => {
				if (track.readyState == "live") {
					track.stop();
				}
			});
		}
	});

	$: context = videoEl;
</script>

<main>
	<!-- svelte-ignore a11y-media-has-caption -->
	<video bind:this={videoEl} {width} {height} />

	{#if errorMessage}
		<p style="color: red;">{errorMessage}</p>
	{/if}
</main>
