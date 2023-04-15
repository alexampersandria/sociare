<script lang="ts">
	import { _, locale } from 'svelte-i18n'

	import type { GroupEvent } from '../lib/types/GroupEvent'

	export let event: GroupEvent

	import { get_user_name_by_id } from '../lib/types/GroupListing'
	import { open_group, time_since } from '../lib/stores/app'
	import { user_object } from '../lib/stores/session'
</script>

<div class="event">
	<div class="time">
		{time_since(event.event.created_at, $locale)}
	</div>
	<div class="user">
		{get_user_name_by_id($open_group, event.event.user_id, $user_object.id) ===
		'user_name_self'
			? $_(
					get_user_name_by_id($open_group, event.event.user_id, $user_object.id)
			  )
			: get_user_name_by_id($open_group, event.event.user_id, $user_object.id)}
	</div>
	<div class="type">
		{$_(event.event.event)}
	</div>
</div>

<style>
	.event {
		color: var(--gray-400);
		text-align: center;
		margin-bottom: 1rem;
	}

	.event .user,
	.event .type {
		display: inline-block;
	}
</style>
