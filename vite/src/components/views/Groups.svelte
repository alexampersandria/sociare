<script lang="ts">
	import { _ } from 'svelte-i18n'

	import { session } from '../../lib/stores'

	import { writable, type Writable } from 'svelte/store'
	import Button from '../Button.svelte'
	import Modal from '../Modal.svelte'

	import type { GroupListing } from '../../lib/types/GroupListing'
	import Group from '../Group.svelte'
	import NewGroup from '../NewGroup.svelte'

	const groups: Writable<GroupListing[] | []> = writable([])
	const groups_fetch_completed = writable(false)

	let container

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
					data.sort((a, b) => {
						return a.events[0].event.created_at < b.events[0].event.created_at
							? 1
							: -1
					})
					groups.set(data)
				}
				groups_fetch_completed.set(true)
			})
	}

	get_groups()

	let show_new_group_modal = false

	const group_created = () => {
		show_new_group_modal = false
		get_groups()
	}

	let scrolled = false
	const on_scroll = () => {
		scrolled = container.scrollTop > 0
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
	class="group-container"
	class:scrolled
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
</div>

<style>
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

	.head,
	.groups {
		padding: 1rem;
	}

	.head {
		position: sticky;
		top: 0;
		background-color: var(--gray-200);
		transition: box-shadow 0.25s ease-in-out;
	}

	.scrolled .head {
		box-shadow: 0 0 3rem 3rem var(--gray-200);
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
</style>
