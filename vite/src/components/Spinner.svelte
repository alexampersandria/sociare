<script lang="ts">
	export let size: number = 1
	export let speed: number = 3
	export let center: boolean = false
</script>

<div
	class="spinner"
	style="--spinner-size: {size}rem; --spinner-speed: {10 / speed}s"
	class:center
/>

<style>
	.spinner::after {
		content: ' ';
		display: block;
		border-radius: 50%;
		width: 0;
		height: 0;
		margin: 8px;
		box-sizing: border-box;
		border: var(--spinner-size) solid var(--white);
		border-color: var(--neutral-400) transparent var(--neutral-400) transparent;
		animation: spinner var(--spinner-speed) infinite linear;
	}
	.spinner.center::after {
		position: absolute;
		inset: 50%;
		translate: -50% -50%;
	}
	@keyframes spinner {
		0% {
			transform: rotate(0);
		}
		50% {
			transform: rotate(900deg);
		}
		100% {
			transform: rotate(1800deg);
		}
	}
</style>
