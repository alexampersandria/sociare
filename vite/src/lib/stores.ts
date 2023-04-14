import Cookies from 'js-cookie'
import { writable, type Writable } from 'svelte/store'
import type { User } from './types/User'

export const session = writable(Cookies.get('payve-session'))
export const session_is_valid = writable(false)
export const user_object: Writable<User | undefined> = writable(undefined)
export const session_fetch_completed = writable(false)
