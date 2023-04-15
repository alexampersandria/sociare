<script lang="ts">
	import { _ } from 'svelte-i18n'
	import type { GroupEvent } from '../lib/types/GroupEvent'

	export let event: GroupEvent

	import { get_user_name_by_id } from '../lib/types/GroupListing'
	import { open_group } from '../lib/stores/app'
	import moment from 'moment'
	import { user_object } from '../lib/stores/session'
</script>

<div class="wrapper" class:self={event.event.user_id === $user_object.id}>
	<div class="above">
		<div class="name">
			{get_user_name_by_id(
				$open_group,
				event.event.user_id,
				$user_object.id
			) === 'user_name_self'
				? $_(
						get_user_name_by_id(
							$open_group,
							event.event.user_id,
							$user_object.id
						)
				  )
				: get_user_name_by_id(
						$open_group,
						event.event.user_id,
						$user_object.id
				  )}
		</div>

		<div class="time">
			&mdash; {moment(event.event.created_at).fromNow()}
		</div>
	</div>
	<div class="message-wrapper">
		<div class="message">
			{#if event.message}
				{event.message.content}
			{:else if event.receipt}
				{JSON.stringify(event.receipt)}
			{:else if event.transaction}
				{JSON.stringify(event.transaction)}
			{/if}
		</div>
	</div>
</div>

<style>
	.above .name,
	.above .time {
		display: inline-block;
		color: var(--gray-400);
	}

	.self .above {
		text-align: right;
	}

	.message {
		background: var(--gray-50);
		color: var(--gray-500);
		border-radius: 0.5rem;
		padding: 1rem;
		margin: 0.5rem 0 1rem 0;
		display: inline-block;
		max-width: min(calc(100% - 4rem), 40rem);
	}

	.self .message-wrapper {
		text-align: right;
	}

	.self .message {
		text-align: left;
	}
</style>
