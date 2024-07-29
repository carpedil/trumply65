<script lang="ts">
	import { tweened } from 'svelte/motion';
	import { cubicInOut } from 'svelte/easing';
	import { onMount } from 'svelte';

	export let duration: number;
	const progress = tweened(0, {
		duration: 10000,
		easing: cubicInOut,
		delay: 1500
	});

	onMount(() => {
		progress.set(duration);
	});
</script>

<div class="outer">
	<div class="inner" style="width: {$progress * 100}%;"></div>
</div>

<style>
	.inner,
	.outer {
		height: 1px;
		border-radius: 10px;
	}

	.outer {
		width: 80vw;
		margin: 10px auto;
		background-color: transparent;
		border: 1px solid #222;
	}

	.inner {
		width: 10%;
		background-color: tomato;
	}
</style>
