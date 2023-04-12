<script lang="ts">
	export let variant:
		| 'pink'
		| 'blue'
		| 'green'
		| 'red'
		| 'yellow'
		| 'violet'
		| 'hollow'
		| 'primary' = 'primary'
	export let disabled: boolean = false
	export let rounded: boolean = false

	export let buttonProps = {
		class: [$$restProps.class],
	}
</script>

<button
	on:click
	{...buttonProps}
	class="button {variant}"
	class:rounded
	class:disabled
>
	<slot />
</button>
