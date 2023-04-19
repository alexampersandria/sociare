<script lang="ts">
	import SvelteMarkdown from 'svelte-markdown'

	import Heading from './renderers/Heading.svelte'
	import Code from './renderers/Code.svelte'
	import Image from './renderers/Image.svelte'
	import Link from './renderers/Link.svelte'

	export let source
	export let isInline = false
</script>

<div class="messagedown" class:isInline>
	<SvelteMarkdown
		{source}
		{isInline}
		renderers={{
			heading: Heading,
			code: Code,
			image: Image,
			link: Link,
		}}
	/>
</div>

<style>
	.messagedown {
		display: inline;
	}

	.messagedown.isInline :global(*) {
		display: inline;
	}

	.messagedown :global(p),
	.messagedown :global(ul),
	.messagedown :global(ol),
	.messagedown :global(pre) {
		margin: 0;
	}

	.messagedown :global(code) {
		padding: 0;
		background: transparent;
		color: var(--gray-600);
	}
</style>
