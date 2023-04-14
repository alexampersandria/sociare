<script lang="ts">
	import { fade } from 'svelte/transition'
	import { teleport } from '../lib/teleport'
	import { createEventDispatcher } from 'svelte'

	export let show: boolean = false
	export let type: 'plain' | 'glass' = 'plain'
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
		transition:fade={{ duration: 100 }}
		on:click={wrapper_click}
		bind:this={wrapper}
	>
		<div class="modal {type}">
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
		background-color: var(--black-100);
		z-index: 480;
	}

	.modal {
		position: absolute;
		top: 50%;
		left: 50%;
		translate: -50% -50%;
		border-radius: 0.25em;
		padding: 2em;
		background-color: var(--white);
		min-width: min(50vw, 32rem);
		z-index: 490;
	}

	.modal.glass {
		background: var(--white-700);
		backdrop-filter: blur(2rem);
		box-shadow: INSET 0 0 0 2px var(--white-300),
			0 1rem 2rem 0.5rem var(--black-100);
	}
</style>
