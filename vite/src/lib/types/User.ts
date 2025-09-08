export interface User {
	id: string
	username: string
	name: string
	email: string
	mobilepay: string
	paypal_me: string
	nickname?: string
	is_admin?: boolean
	active?: boolean
	created_at: number
}
