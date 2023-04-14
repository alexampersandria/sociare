<script lang="ts">
	import { _ } from 'svelte-i18n'

	import { goto } from '@roxi/routify'
	import {
		session_is_valid,
		session_fetch_completed,
	} from '../lib/stores/session'
	import GroupListings from '../components/views/GroupListings.svelte'
	import GroupView from '../components/views/GroupView.svelte'

	session_is_valid.subscribe((is_valid) => {
		if (!is_valid && session_fetch_completed) {
			$goto('/login')
		}
	})

	import { open_group } from '../lib/stores/app'
</script>

<svelte:head>
	<title>{$_('page_title')} &mdash; {$_('app_page_title')}</title>
</svelte:head>
<div class="app">
	<GroupListings />
	<GroupView />
</div>

<style>
	.app {
		position: relative;
	}
</style>
