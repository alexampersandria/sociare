<script lang="ts">
	export let variant: 'danger' | 'hollow' | 'black' | 'primary' | 'secondary' =
		'primary'
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
