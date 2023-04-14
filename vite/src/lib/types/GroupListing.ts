import type { Debt } from './Debt'
import type { Group } from './Group'
import type { GroupEvent } from './GroupEvent'
import type { User } from './User'

export interface GroupListing {
	group: Group
	events: GroupEvent[]
	users: User[]
	debts: Debt[]
}

export const get_user_by_id = (
	group_listing: GroupListing,
	user_id: string
): User => {
	return group_listing.users.find((user) => user.id === user_id)
}

export const get_user_name_by_id = (
	group_listing: GroupListing,
	user_id: string
): string => {
	let user = get_user_by_id(group_listing, user_id)
	return user.nickname ? user.nickname : user.name
}
