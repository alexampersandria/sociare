<script lang="ts">
	import { _, locale } from 'svelte-i18n'

	import type { GroupEvent } from '../lib/types/GroupEvent'

	export let event: GroupEvent
	export let group
	export let inline: boolean = false
	export let concise: boolean = false

	import { get_user_name_by_id } from '../lib/types/GroupListing'
	import { time_since } from '../lib/stores/app'
	import { user_object } from '../lib/stores/session'
	import MessageDown from './MessageDown.svelte'

	let f = ''
	$: {
		let sections = event.event.event.split(' ')
		let user = undefined
		let user_is_self = false
		let localized_sections = sections.map((section) => {
			if (!section.includes(':')) return section
			else if (section.startsWith('user_id:')) {
				let id = section.replace('user_id:', '')
				user = get_user_name_by_id(group, id, $user_object.id)
				if (id === event.event.user_id) {
					user_is_self = true
				}
			}
		})
		f = localized_sections
			.map((section) => {
				if (section) {
					if (user_is_self) {
						return $_(section + '_self')
					} else {
						return $_(section)
					}
				}
			})
			.join(' ')
			.replace('{user}', user)
	}
</script>

<div class="event" class:inline>
	<div class="time">
		{time_since(event.event.created_at, $locale, concise)}
	</div>
	<div class="action">
		{get_user_name_by_id(group, event.event.user_id, $user_object.id) ===
		'user_name_self'
			? $_(get_user_name_by_id(group, event.event.user_id, $user_object.id))
			: get_user_name_by_id(
					group,
					event.event.user_id,
					$user_object.id
			  )}{#if event.message}:
			{event.message.content}
		{:else}
			&nbsp;{f}
		{/if}
	</div>
</div>

<style>
	.event {
		color: var(--gray-400);
		text-align: center;
		white-space: nowrap;
	}

	.event:not(.inline) {
		margin-bottom: 1rem;
	}

	.event.inline .action {
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.event.inline {
		display: flex;
		justify-content: space-between;
		flex-direction: row-reverse;
	}
</style>
