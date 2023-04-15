<script lang="ts">
	import { fade } from 'svelte/transition'
	import { teleport } from '../lib/teleport'
	import { createEventDispatcher } from 'svelte'

	export let show: boolean = false
	export let type: 'plain' | 'glass' = 'plain'
	let overlay

	const dispatch = createEventDispatcher()

	$: {
		if (show && overlay) {
			const first_focusable_in_overlay = overlay.querySelector(
				'a, button, input, textarea, select'
			)
			if (first_focusable_in_overlay) {
				first_focusable_in_overlay.focus()
			}
		}
	}
</script>

<svelte:body
	on:keydown={(event) => {
		if (event.key === 'Escape' && show) {
			dispatch('close')
		}
	}}
/>
{#if show}
	<div
		class="overlay {type}"
		use:teleport={'body'}
		transition:fade={{ duration: 100 }}
		bind:this={overlay}
	>
		<slot />
	</div>
{/if}

<style>
	.overlay {
		position: fixed;
		inset: 0;
		width: 100vw;
		height: 100vh;
		background-color: var(--black-100);
		z-index: 320;
		background-color: var(--white);
		padding: 2rem 0;
	}

	.overlay.glass {
		background: var(--white-500);
		backdrop-filter: blur(2rem);
	}
</style>
