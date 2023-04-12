<script lang="ts">
	export let type: "primary" | "hollow" = "primary";
	export let disabled: boolean = false;
	export let rounded: boolean = false;

	export let buttonProps = {
		class: [$$restProps.class],
	};
</script>

<button
	on:click
	{...buttonProps}
	class="button {type}"
	class:rounded
	class:disabled
>
	<slot />
</button>
