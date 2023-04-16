<script lang="ts">
	import { format_currency, type CurrencyCode, currencies } from '../lib/econ'
	import { createEventDispatcher } from 'svelte'
	const dispatch = createEventDispatcher()

	export let name: string
	export let currency: CurrencyCode | string
	export let with_prefix_suffix: boolean = false
	export let show_label: boolean = true

	let self
	let value = 0

	const raw = (): number => {
		let c = currencies[currency]
		let only_0_to_9_comma_and_dot = self.value
			.replace(/[^0-9.,]/g, '')
			.replace(c.thousandsSeparator, '')
			.replace(c.decimalSeparator, '.')
		return Math.round(
			parseFloat(only_0_to_9_comma_and_dot) * 10 ** c.decimalPlaces
		)
	}

	const on_blur = () => {
		value = raw()
		dispatch('blur', { raw: value })
		if (value > 0) {
			self.value = format_currency(value, currency, with_prefix_suffix)
		} else {
			self.value = ''
		}
	}

	const on_input = () => {
		dispatch('input', { raw: raw() })
	}
</script>

<div class="currency-input">
	{#if show_label && currencies[currency].prefix}
		<div class="currency-label prefix">
			{currencies[currency].prefix}
		</div>
	{/if}
	<input
		bind:this={self}
		type="text"
		{name}
		on:blur={on_blur}
		on:input={on_input}
	/>
	{#if show_label && currencies[currency].suffix}
		<div class="currency-label suffix">
			{currencies[currency].suffix}
		</div>
	{/if}
</div>

<style>
	.currency-input {
		position: relative;
		display: flex;
		align-items: center;
	}

	.currency-input input,
	.currency-input .currency-label {
		border-radius: 0;
		margin: 0;
	}

	.currency-input input {
		position: relative;
		z-index: 1;
	}

	.currency-label {
		background: var(--gray-300);
		color: var(--gray-500);
		padding: 0.5em 1em;
	}

	.currency-input :first-child {
		border-top-left-radius: 0.25rem;
		border-bottom-left-radius: 0.25rem;
	}

	.currency-input :last-child {
		border-top-right-radius: 0.25rem;
		border-bottom-right-radius: 0.25rem;
	}
</style>
