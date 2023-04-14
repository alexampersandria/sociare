export interface Event {
	id: string
	user_id: string
	group_id: string
	event: string
	message_id?: string
	receipt_id?: string
	transaction_id?: string
	created_at: number
}
