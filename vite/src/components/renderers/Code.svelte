<script>
	import Highlight from 'svelte-highlight'
	import github from 'svelte-highlight/styles/github'

	import * as languages from 'svelte-highlight/languages'
	import { language_alias } from '../../lib/language_alias'

	export let text
	export let lang

	let language = languages['plaintext']

	$: {
		if (languages[lang]) {
			language = languages[lang]
		} else if (language_alias[lang]) {
			language = languages[language_alias[lang]]
		}
	}
</script>

<svelte:head>
	{@html github}
</svelte:head>

<div class="messagedown-code">
	<Highlight {language} code={text} />
</div>

<style>
	.messagedown-code {
		display: inline;
	}
</style>
