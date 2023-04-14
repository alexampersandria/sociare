import type { Event } from './Event'
import type { Message } from './Message'
import type { Receipt } from './Receipt'
import type { Transaction } from './Transaction'

export interface GroupEvent {
	event: Event
	message?: Message
	receipt?: Receipt
	transaction?: Transaction
}
