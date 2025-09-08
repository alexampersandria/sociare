<script lang="ts">
	import { _ } from 'svelte-i18n'
</script>

<svelte:head>
	<title>{$_('page_title')} &mdash; {$_('_fallback_page_title')}</title>
</svelte:head>

<div class="container align-center">
	<div class="huge-emoji">😖</div>
	<h1>Page not found</h1>
	<p>🤷‍♂️ oopsie woopsie 🤷‍♀️</p>
	<a href="/" class="button red">go to root @ <code>'/'</code></a>
</div>

<style>
	.huge-emoji {
		margin-top: 4rem;
		font-size: 6rem;
		animation: wiggle 0.5s ease-in-out infinite;
	}

	@keyframes wiggle {
		0%,
		100% {
			transform: rotate(-6deg);
		}
		50% {
			transform: rotate(6deg);
		}
	}
</style>
