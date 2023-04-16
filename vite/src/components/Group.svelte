<script lang="ts">
	import { format_currency } from '../lib/econ'
	import type { GroupListing } from '../lib/types/GroupListing'
	export let group: GroupListing
	import { _ } from 'svelte-i18n'
	import { open_group_id } from '../lib/stores/app'
	import { user_object } from '../lib/stores/session'
	import { gravatar } from '../lib/gravatar'
	import GroupEvent from './GroupEvent.svelte'

	const on_click = () => {
		open_group_id.set(group.group.id)
	}
	const on_keydown = (event) => {
		if (event.key === 'Enter') {
			on_click()
		}
	}

	const users_to_show = 4

	let your_balance = 0
	$: {
		your_balance = 0
		group.debts.forEach((debt) => {
			if (debt.from_id === $user_object.id) {
				your_balance -= debt.amount
			} else if (debt.to_id === $user_object.id) {
				your_balance += debt.amount
			}
		})
	}
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
			<GroupEvent {event} {group} inline concise />
		{/each}
	</div>
	<div class="bottom">
		<div class="users">
			{#each group.users.slice(0, users_to_show) as user}
				<div class="gravatar">
					<img class="round" src={gravatar(user.email, 48)} alt={user.name} />
				</div>
			{/each}
			{#if group.users.length > users_to_show}
				<div class="more">
					<div class="inner">
						+{Math.abs(group.users.length - users_to_show)}
					</div>
				</div>
			{/if}
		</div>
		<div class="balance">
			{#if your_balance < 0}
				<div class="balance-inner you-owe">
					<div class="text">
						{$_('you_owe')}
					</div>
					<div class="amount">
						{format_currency(-your_balance, group.group.currency)}
					</div>
				</div>
			{:else if your_balance > 0}
				<div class="balance-inner you-are-owed">
					<div class="text">{$_('you_are_owed')}</div>
					<div class="amount">
						{format_currency(your_balance, group.group.currency)}
					</div>
				</div>
			{/if}
		</div>
	</div>
</a>

<style>
	.group {
		display: block;
		background-color: var(--gray-50);
		border-radius: 0.25em;
		padding: 1em;
		cursor: pointer;
		transition: background-color 0.25s ease-in-out, box-shadow 0.25s ease-in-out;
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
	.group:hover {
		background-color: var(--gray-100);
	}

	.users {
		position: relative;
		top: 0.25rem;
	}

	.group:hover .users .gravatar img,
	.group:hover .users .more {
		border-color: var(--gray-100);
	}

	.group:active {
		background-color: var(--white);
	}

	.bottom {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-top: 0.5rem;
	}

	.balance-inner .text,
	.balance-inner .amount {
		display: inline-flex;
	}

	.balance-inner .text {
		color: var(--gray-400);
	}

	.balance-inner .amount {
		border-radius: 0.25em;
		padding: 0.125em 0.33em;
	}

	.you-owe .amount {
		color: var(--red-500);
		background-color: var(--red-100);
		border: 1px solid var(--red-200);
	}

	.you-are-owed .amount {
		color: var(--green-500);
		background-color: var(--green-100);
		border: 1px solid var(--green-200);
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
		transition: border-color 0.25s ease-in-out;
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

		.group.active :global(.event),
		.group.active .balance-inner .text {
			color: var(--pink-400);
		}
	}
</style>
