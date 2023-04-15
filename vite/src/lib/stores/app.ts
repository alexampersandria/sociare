import { writable, type Writable } from 'svelte/store'
import type { GroupListing } from '../types/GroupListing'

export const open_group = writable(null)
export const open_group_id = writable(null)

export const groups: Writable<GroupListing[] | []> = writable([])
export const groups_fetch_completed = writable(false)

const limit = 1
export const get_groups = (session: string) => {
	groups_fetch_completed.set(false)
	fetch(`${import.meta.env.VITE_API_URL}/api/v1/groups?limit=${limit}`, {
		headers: {
			Authorization: `Bearer ${session}`,
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

export const is_desktop = writable(false)

export const langs = ['en', 'da']

// @ts-ignore
import moment from 'moment/min/moment-with-locales'

export const time_since = (
	date: number,
	locale: string = 'en',
	short: boolean = false
) => {
	moment.locale(locale)
	return moment(date).fromNow(short)
}
