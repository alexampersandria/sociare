<script lang="ts">
	import { _ } from 'svelte-i18n'

	import { goto } from '@roxi/routify'
	import { session_is_valid, session_fetch_completed } from '../lib/stores'
	import Groups from '../components/views/Groups.svelte'

	session_is_valid.subscribe((is_valid) => {
		if (!is_valid && session_fetch_completed) {
			$goto('/login')
		}
	})
</script>

<svelte:head>
	<title>{$_('page_title')} &mdash; {$_('app_page_title')}</title>
</svelte:head>
<div class="app">
	<Groups />
</div>

<style>
	.app {
		display: flex;
	}
</style>
