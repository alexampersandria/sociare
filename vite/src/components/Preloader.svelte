<script>
	import { fade } from 'svelte/transition'
	import Spinner from './Spinner.svelte'

	export let show = false
</script>

{#if show}
	<div class="preloader" out:fade>
		<Spinner center={true} size={2} speed={2} />
	</div>
{/if}

<style>
	.preloader {
		position: fixed;
		inset: 0;
		width: 100vw;
		height: 100vh;
		background-color: var(--neutral-300);
		z-index: 1000;
	}
</style>
