<script lang="ts">
	import { _ } from 'svelte-i18n'
	import { createEventDispatcher } from 'svelte'
	import { Close } from 'carbon-icons-svelte'

	import { open_group, open_group_id, time_since } from '../../lib/stores/app'
	import { format_currency } from '../../lib/econ'
	import { gravatar } from '../../lib/gravatar'
	import UserSearch from '../UserSearch.svelte'
	import { session } from '../../lib/stores/session'

	const dispatch = createEventDispatcher()

	const on_close = () => {
		dispatch('close')
	}

	const add_user_to_group = async (user_id: string) => {
		if (user_id) {
			const res = await fetch(
				`${import.meta.env.VITE_API_URL}/api/v1/groups/${$open_group_id}/add`,
				{
					method: 'POST',
					headers: {
						'Content-Type': 'application/json',
						Authorization: `Bearer ${$session}`,
					},
					body: JSON.stringify({
						user_id,
					}),
				}
			)
			const data = await res.json()
			if (data.error) {
				alert(data.error)
			} else {
				dispatch('added_user', data)
			}
		}
	}
</script>

<div class="head">
	<div class="back">
		<a class="none" href="javascript:void(0)" on:click={on_close}
			><Close size={24} /></a
		>
	</div>
	<div class="name">{$open_group.group.name}</div>
	<div class="total">
		{format_currency($open_group.group.total, $open_group.group.currency)}
	</div>
	<div class="created">
		{$_('created_time_since')}
		{time_since($open_group.group.created_at)}
	</div>
	<div class="users">
		<div class="add-user">
			<label for="add-user-to-group">{$_('add_user_to_group_label')}</label>
			<UserSearch
				name="add-user-to-group"
				on:select={(event) => {
					add_user_to_group(event.detail.id)
				}}
			/>
		</div>
		{#each $open_group.users as user}
			<div class="user">
				<div class="gravatar">
					<img src={gravatar(user.email, 48)} alt={user.name} class="round" />
				</div>
				<div class="info">
					<div class="name">
						{user.nickname ? user.nickname : user.name}
						{#if user.is_admin}<div class="label admin">
								{$_('group_user_is_admin_label')}
							</div>{/if}
					</div>
					<div class="username">@{user.username}</div>
				</div>
				<div class="actions">
					<a href="#">mock action</a>
					<a href="#">another</a>
				</div>
			</div>
		{/each}
	</div>
</div>

<h2>raw JSON</h2>
<pre><code>{JSON.stringify($open_group, undefined, 2)}</code></pre>

<style>
	.add-user {
		margin-bottom: 1rem;
	}

	.users .user {
		display: flex;
		padding: 0.5em 0.5em 0.25em 0.5em;
		color: var(--gray-500);
	}

	.users .user .gravatar img {
		width: 2.5rem;
		height: 2.5rem;
	}

	.users .user .info {
		margin-left: 0.5em;
	}

	.users .user .info .label {
		position: relative;
		top: -0.125em;
		display: inline-block;
	}

	.users .user .info .name {
		color: var(--gray-700);
	}

	.users .user .info .username {
		font-size: 0.8em;
	}

	.users .user .actions {
		margin-left: auto;
	}
</style>
