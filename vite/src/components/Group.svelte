<script lang="ts">
	import { format_currency } from '../lib/econ'
	import {
		get_user_name_by_id,
		type GroupListing,
	} from '../lib/types/GroupListing'
	export let group: GroupListing
	import { _, locale } from 'svelte-i18n'
	import { open_group_id, time_since } from '../lib/stores/app'
	import { user_object } from '../lib/stores/session'
	import { gravatar } from '../lib/gravatar'

	const on_click = () => {
		open_group_id.set(group.group.id)
	}
	const on_keydown = (event) => {
		if (event.key === 'Enter') {
			on_click()
		}
	}

	const users_to_show = 4
</script>

<a
	href="javascript:void(0);"
	class="group none"
	class:active={$open_group_id == group.group.id}
	on:click={on_click}
	on:keydown={on_keydown}
>
	<div class="head">
		<div class="name">{group.group.name}</div>
		<div class="total">
			{format_currency(group.group.total, group.group.currency)}
		</div>
	</div>
	<div class="events">
		{#each group.events as event}
			<div class="event">
				<div class="action">
					<span class="user">
						{get_user_name_by_id(
							group,
							event.event.user_id,
							$user_object.id
						) === 'user_name_self'
							? $_(
									get_user_name_by_id(
										group,
										event.event.user_id,
										$user_object.id
									)
							  )
							: get_user_name_by_id(
									group,
									event.event.user_id,
									$user_object.id
							  )}</span
					>{#if event.message}<span class="message"
							>: {event.message.content}</span
						>{:else}<span class="type">&nbsp;{$_(event.event.event)}</span>
					{/if}
				</div>
				<div class="time">
					{time_since(event.event.created_at, $locale, true)}
				</div>
			</div>
		{/each}
	</div>
	<div class="users">
		{#each group.users.slice(0, users_to_show) as user}
			<div class="gravatar">
				<img class="round" src={gravatar(user.email, 48)} alt={user.name} />
			</div>
		{/each}
		{#if group.users.length > users_to_show}
			<div class="more">
				<div class="inner">+{Math.abs(group.users.length - users_to_show)}</div>
			</div>
		{/if}
	</div>
</a>

<style>
	.group {
		display: block;
		background-color: var(--gray-50);
		border-radius: 0.25em;
		padding: 1em;
		cursor: pointer;
		transition: background-color 0.25s;
		overflow: hidden;
	}

	.group:not(:last-child) {
		margin-bottom: 1em;
	}

	.group .head {
		display: flex;
		justify-content: space-between;
	}

	.group .head .name,
	.group .head .total {
		white-space: nowrap;
		color: var(--gray-700);
	}

	.group .head .name {
		position: relative;
		top: -0.5rem;
		font-size: 1.5em;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.events .event {
		color: var(--gray-400);
		display: flex;
		justify-content: space-between;
		white-space: nowrap;
	}

	.events .event .action {
		justify-content: space-between;
		overflow: hidden;
		text-overflow: ellipsis;
	}
	.group:hover {
		background-color: var(--gray-100);
	}

	.group:hover .users .gravatar img,
	.group:hover .users .more {
		border-color: var(--gray-100);
	}

	.group:active {
		background-color: var(--white);
	}

	.users {
		margin-top: 0.5rem;
	}

	.users .gravatar,
	.users .more {
		display: inline-block;
		width: 1rem;
	}

	.users .gravatar img,
	.users .more {
		width: 1.5rem;
		height: 1.5rem;
		overflow: hidden;
		border: 2px solid var(--gray-50);
	}

	.users .more {
		background-color: var(--gray-300);
		border-radius: 50%;
		color: var(--gray-500);
		text-align: center;
		font-size: 0.8rem;
	}

	.users .more .inner {
		position: relative;
		top: 0.25rem;
	}

	@media (min-width: 920px) {
		.group.active {
			background-color: var(--pink-100);
			box-shadow: 0 0 0 2px var(--pink-500);
		}

		.group.active .users .gravatar img,
		.group.active .users .more {
			border-color: var(--pink-100);
		}

		.group.active .events .event {
			color: var(--pink-400);
		}
	}
</style>
