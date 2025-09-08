export interface UserSession {
	id: string
	user_id: string
	created_at: number
	accessed_at: number
	ip_address: string
	user_agent: string
}
