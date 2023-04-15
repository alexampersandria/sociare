import { writable, type Writable } from 'svelte/store'
import type { GroupListing } from '../types/GroupListing'

export const open_group = writable(null)
export const open_group_id = writable(null)

export const groups: Writable<GroupListing[] | []> = writable([])
export const groups_fetch_completed = writable(false)

export const is_desktop = writable(false)
