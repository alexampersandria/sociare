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

	const on_click = () => {
		open_group_id.set(group.group.id)
	}
	const on_keydown = (event) => {
		if (event.key === 'Enter') {
			on_click()
		}
	}
</script>

<div
	class="group"
	class:active={$open_group_id == group.group.id}
	on:click={on_click}
	on:keydown={on_keydown}
	tabindex="0"
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
</div>

<style>
	.group {
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

	.group:active {
		background-color: var(--white);
	}

	@media (min-width: 920px) {
		.group.active {
			background-color: var(--pink-100);
			box-shadow: 0 0 0 2px var(--pink-500);
		}

		.group.active .events .event {
			color: var(--pink-400);
		}
	}
</style>
