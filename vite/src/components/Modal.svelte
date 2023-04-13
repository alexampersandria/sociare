<script lang="ts">
	import { fade } from 'svelte/transition'
	import { teleport } from '../lib/teleport'
	import { createEventDispatcher } from 'svelte'

	export let show: boolean = false
	let wrapper

	const dispatch = createEventDispatcher()

	const wrapper_click = (event: MouseEvent) => {
		const target = event.target as HTMLElement
		if (target === wrapper) {
			dispatch('close')
		}
	}

	$: {
		if (show && wrapper) {
			const first_focusable_in_wrapper = wrapper.querySelector(
				'a, button, input, textarea, select'
			)
			if (first_focusable_in_wrapper) {
				first_focusable_in_wrapper.focus()
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
		class="modal-wrapper"
		use:teleport={'body'}
		in:fade
		out:fade
		on:click={wrapper_click}
		bind:this={wrapper}
	>
		<div class="modal">
			<slot />
		</div>
	</div>
{/if}

<style>
	.modal-wrapper {
		position: fixed;
		inset: 0;
		width: 100vw;
		height: 100vh;
		background-color: var(--grey-300);
		z-index: 480;
	}

	.modal {
		position: absolute;
		top: 50%;
		left: 50%;
		translate: -50% -50%;
		background: var(--white);
		filter: drop-shadow(0.5em 0.5em 0 var(--black));
		padding: 2em;
		min-width: min(50vw, 32rem);
		z-index: 490;
	}
</style>
