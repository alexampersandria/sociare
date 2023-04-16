<script lang="ts">
	import { _, locale } from 'svelte-i18n'
	import type { GroupEvent } from '../lib/types/GroupEvent'

	export let event: GroupEvent

	import { get_user_name_by_id } from '../lib/types/GroupListing'
	import { open_group, time_since } from '../lib/stores/app'
	import { user_object } from '../lib/stores/session'
	import { string_contains_only_emojis } from '../lib/emoji'
	import { format_currency } from '../lib/econ'
</script>

{#if $open_group}
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
				&mdash; {time_since(event.event.created_at, $locale)}
			</div>
		</div>
		<div class="message-wrapper">
			{#if event.message}
				<div
					class="message"
					class:emoji-only={string_contains_only_emojis(event.message.content)}
				>
					{event.message.content}
				</div>
			{:else if event.receipt}
				<div class="receipt">
					<div class="amount">
						{format_currency(event.receipt.amount, $open_group.group.currency)}
					</div>
					{#if event.receipt.info}
						<div class="info">{event.receipt.info}</div>
					{/if}
				</div>
			{:else if event.transaction}
				<div class="transaction">
					{JSON.stringify(event.transaction)}
				</div>
			{/if}
		</div>
	</div>
{/if}

<style>
	.above .name,
	.above .time {
		display: inline-block;
		color: var(--gray-400);
	}

	.self .above {
		text-align: right;
	}

	.message,
	.receipt {
		background: var(--gray-50);
		color: var(--gray-500);
		border-radius: 0.5rem;
		padding: 1rem;
		margin: 0.5rem 0 1rem 0;
		display: inline-block;
		max-width: min(calc(100% - 4rem), 40rem);
	}

	.receipt {
		text-align: center;
	}

	.receipt .amount {
		font-size: 1.5rem;
		padding: 0 2rem;
		white-space: nowrap;
	}

	.receipt .info {
		color: var(--gray-400);
	}

	.message.emoji-only {
		font-size: 2rem;
		background: none;
		padding: 0;
	}

	.self .message-wrapper {
		text-align: right;
	}

	.self .message {
		text-align: left;
	}
</style>
