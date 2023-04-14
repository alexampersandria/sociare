<script lang="ts">
	import { _ } from 'svelte-i18n'

	import { goto } from '@roxi/routify'
	import {
		session,
		session_is_valid,
		session_fetch_completed,
	} from '../lib/stores'

	import { writable } from 'svelte/store'

	session_is_valid.subscribe((is_valid) => {
		if (!is_valid && session_fetch_completed) {
			$goto('/login')
		}
	})

	const groups = writable([])
	const groups_fetch_completed = writable(false)

	const get_groups = () => {
		groups_fetch_completed.set(false)
		fetch(`${import.meta.env.VITE_API_URL}/api/v1/groups`, {
			headers: {
				Authorization: `Bearer ${$session}`,
			},
		})
			.then((res) => res.json())
			.then((data) => {
				if (data.error) {
					alert(data.error)
				} else {
					groups.set(data)
				}
				groups_fetch_completed.set(true)
			})
	}

	get_groups()
</script>

<svelte:head>
	<title>{$_('page_title')} &mdash; {$_('app_page_title')}</title>
</svelte:head>
<div class="app">
	<div class="group-container">
		{#if $groups.length > 0}
			{#each $groups as group}
				<div class="group">
					<h2>{group.name}</h2>
				</div>
			{/each}
		{:else if !$groups_fetch_completed}
			<div class="loading">Loading...</div>
		{:else}
			<div class="no_groups_found">{$_('no_groups_found')}</div>
		{/if}
	</div>
</div>

<style>
	.app {
		display: flex;
	}

	.group-container {
		display: relative;
		height: 100vh;
		overflow-y: scroll;
		background-color: var(--gray-200);
		min-width: 10rem;
		flex: 1;
	}

	.loading,
	.no_groups_found {
		position: absolute;
		top: 50%;
		left: 50%;
		translate: -50% -50%;
		white-space: nowrap;
		color: var(--gray-400);
	}
</style>
