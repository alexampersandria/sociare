export interface UserGroup {
	id: string
	user_id: string
	group_id: string
	nickname?: string
	is_admin: boolean
	active: boolean
	created_at: number
}
