<script lang="ts">
	import { _, locale } from 'svelte-i18n'

	import { goto, params } from '@roxi/routify'

	import { session, user_object } from '../../lib/stores/session'
	import Button from '../Button.svelte'
	import Modal from '../Modal.svelte'
	import Group from '../Group.svelte'
	import NewGroup from '../NewGroup.svelte'
	import {
		get_groups,
		groups,
		groups_fetch_completed,
		langs,
		open_group,
		open_group_id,
	} from '../../lib/stores/app'
	import { fade } from 'svelte/transition'
	import { log_out } from '../../lib/auth'

	let container
	const limit = 1

	get_groups($session)

	let show_new_group_modal = false

	const group_created = () => {
		show_new_group_modal = false
		get_groups($session)
	}

	let scrolled = false
	let bottomed = false
	const on_scroll = () => {
		if (container) {
			scrolled = container.scrollTop > 0
			bottomed =
				container.scrollTop + container.clientHeight >= container.scrollHeight
		}
	}
</script>

<Modal
	show={show_new_group_modal}
	type="glass"
	on:close={() => {
		show_new_group_modal = false
	}}
>
	<NewGroup on:group_created={group_created} />
</Modal>

<div
	class="group-listings"
	class:scrolled
	class:bottomed
	bind:this={container}
	on:scroll={on_scroll}
>
	<div class="head">
		<div class="title">{$_('groups')}</div>
		{#if $groups.length > 0}
			<div class="count label">
				{$groups.length}
			</div>
		{/if}
		<div class="new-group">
			<Button
				on:click={() => {
					show_new_group_modal = true
				}}>{$_('new_group_button')}</Button
			>
		</div>
	</div>
	{#if $groups.length > 0}
		<div class="groups">
			{#each $groups as group}
				<Group {group} />
			{/each}
		</div>
	{:else if !$groups_fetch_completed}
		<div class="loading">Loading...</div>
	{:else}
		<div class="no_groups_found">{$_('no_groups_found')}</div>
	{/if}
	<div class="bottom-section">
		<Button on:click={log_out} variant="secondary">{$_('log_out')}</Button>
		{#each langs as lang}
			{#if lang !== $locale}
				<Button
					on:click={() => {
						locale.set(lang)
					}}
					variant="hollow">🌍</Button
				>
			{/if}
		{/each}
	</div>
	{#if $open_group}
		<div class="cover" transition:fade={{ duration: 200 }} />
	{/if}
</div>

<style>
	.group-listings {
		position: absolute;
		inset: 0;
		height: 100vh;
		overflow-y: auto;
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

	.head,
	.groups {
		padding: 1rem;
	}

	.head {
		position: sticky;
		top: 0;
		background-color: var(--gray-200);
		transition: box-shadow 0.25s ease-in-out;
		z-index: 1;
	}

	.scrolled .head {
		box-shadow: 0 0 2rem 2rem var(--gray-200);
	}

	.head .title,
	.head .count,
	.head .new-group {
		display: inline-block;
		vertical-align: middle;
	}

	.head .new-group {
		float: right;
	}
	.head .title {
		font-size: 1.5rem;
		color: var(--gray-700);
	}

	.bottom-section {
		position: sticky;
		bottom: 0;
		background-color: var(--gray-200);
		padding: 1rem;
		display: flex;
		justify-content: space-between;
	}

	.group-listings:not(.bottomed) .bottom-section {
		box-shadow: 0 0 1rem 1rem var(--gray-200);
	}

	.cover {
		position: fixed;
		inset: 0;
		background-color: var(--black-300);
		width: 100%;
		height: 100vh;
		pointer-events: none;
		z-index: 2;
	}
</style>
