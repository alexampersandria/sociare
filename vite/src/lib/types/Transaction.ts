export interface Transaction {
	id: string
	group_id: string
	from_id: string
	to_id: string
	amount: number
	method: string
	confirmed: boolean
	deleted: boolean
	created_at: number
}
