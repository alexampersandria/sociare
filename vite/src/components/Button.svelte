<script lang="ts">
	export let type: 'primary' | 'hollow' = 'primary';
	export let disabled: boolean = false;
	export let rounded: boolean = false;
</script>

<button class={type} class:rounded class:disabled>
	<slot />
</button>

<style>
	button {
		border: 0;
		background: #000;
		color: #fff;
		font-size: 1rem;
		padding: 0.75em 1.5em;
		border-radius: 0.25em;
		cursor: pointer;
		transition: background 0.25s ease-in-out;
	}

	button.primary {
		background: #000;
		color: #fff;
	}

	button.primary:not(.disabled):hover {
		background: var(--neutral-800);
	}

	button.hollow {
    position: relative;
		background: transparent;
		color: #000;
	}

	button.hollow::before {
		content: '';
		position: absolute;
		inset: 0;
		border-radius: 0.25em;
		border: 0.15em solid #000;
		clip-path: polygon(
			0% 0%,
			0% 100%,
			0.5em 100%,
			0.5em 0,
			calc(100% - 0.5em) 0,
			calc(100% - 0.5em) 100%,
			100% 100%,
			100% 0%
		);
		transition: clip-path 0.3s cubic-bezier(0.5, 0.05, 0.5, 0.95);
	}

	button.hollow:not(.disabled):hover::before {
		clip-path: polygon(0% 0%, 0% 100%, 50% 100%, 50% 0, 50% 0, 50% 100%, 100% 100%, 100% 0%);
	}

	button.disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	button.rounded {
		border-radius: 9999em;
	}
</style>
