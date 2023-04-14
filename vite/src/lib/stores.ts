import Cookies from 'js-cookie'
import { writable, type Writable } from 'svelte/store'
import type { UserObject } from './types/UserObject'

export const session = writable(Cookies.get('payve-session'))
export const session_is_valid = writable(false)
export const user_object: Writable<UserObject | undefined> = writable(undefined)
export const session_fetch_completed = writable(false)
