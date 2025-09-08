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
	if (group_listing) {
		return group_listing.users.find((user) => user.id === user_id)
	}
}

export const get_user_name_by_id = (
	group_listing: GroupListing,
	user_id: string,
	self_id: string
): string => {
	if (group_listing) {
		let user = get_user_by_id(group_listing, user_id)
		if (user.id === self_id) {
			return 'user_name_self'
		} else if (user.nickname) {
			return user.nickname
		} else {
			return user.name
		}
	}
}
