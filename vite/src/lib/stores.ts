import Cookies from 'js-cookie'
import { writable, type Writable } from 'svelte/store'
import type { UserObject } from './UserObject'

export const session = writable(Cookies.get('payve-session'))
export const session_is_valid = writable(false)
export const user_object: Writable<UserObject | undefined> = writable(undefined)

const validate_session = async (session) => {
	const res = await fetch(
		`${import.meta.env.VITE_API_URL}/api/v1/sessions/${session}`
	)
	let data = await res.json()
	if (!data.error && data.id) {
		session_is_valid.set(true)
		user_object.set(data)
	} else {
		session_is_valid.set(false)
	}
}

session.subscribe(async (session) => {
	validate_session(session)
})
