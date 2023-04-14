<script lang="ts">
	import { format_currency } from '../lib/econ'
	import {
		get_user_name_by_id,
		type GroupListing,
	} from '../lib/types/GroupListing'
	export let group: GroupListing
	import { _ } from 'svelte-i18n'
	import moment from 'moment'
</script>

<div class="group">
	<div class="head">
		<div class="name">{group.group.name}</div>
		<div class="total">
			{format_currency(group.group.total, group.group.currency)}
		</div>
	</div>
	<div class="events">
		{#each group.events as event}
			<div class="event">
				<div class="user">
					{get_user_name_by_id(group, event.event.user_id)}
				</div>
				<div class="type">{$_(event.event.event)}</div>
				<div class="time">
					{moment(event.event.created_at).fromNow()}
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
	}

	.group:not(:last-child) {
		margin-bottom: 1em;
	}

	.group .head .name,
	.group .head .total {
		display: inline-block;
		color: var(--gray-700);
	}

	.group .head .name {
		font-size: 1.5em;
	}

	.group .head .total {
		float: right;
	}

	.events .event .user,
	.events .event .type,
	.events .event .time {
		display: inline-block;
		color: var(--gray-400);
	}

	.events .event .time {
		float: right;
	}
</style>
