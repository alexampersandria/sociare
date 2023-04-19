<script lang="ts">
	import SvelteMarkdown from 'svelte-markdown'

	import Heading from './renderers/Heading.svelte'
	import Code from './renderers/Code.svelte'
	import Image from './renderers/Image.svelte'
	import Link from './renderers/Link.svelte'

	export let source
	export let isInline = false

	let endtag = 'none'
	$: {
		if (source) {
			let sections = source.split(' ')
			let last_section = sections[sections.length - 1]
			if (last_section.startsWith('/')) {
				endtag = last_section.replace('/', '')
			}
		}
	}
</script>

<div class="messagedown endtag-{endtag}" class:isInline>
	<SvelteMarkdown
		{source}
		{isInline}
		renderers={{
			heading: Heading,
			code: Code,
			image: Image,
			link: Link,
		}}
		options={{
			breaks: true,
			tables: true,
			gfm: true,
			sanitize: true,
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
	.messagedown :global(pre),
	.messagedown :global(blockquote) {
		margin: 0;
	}
	.messagedown :global(code) {
		padding: 0;
		background: transparent;
		color: var(--gray-600);
	}

	.messagedown.endtag-s {
		font-family: 'Comic Sans MS', 'Comic Sans', cursive;
	}
</style>
