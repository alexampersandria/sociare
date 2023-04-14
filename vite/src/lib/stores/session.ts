import Cookies from 'js-cookie'
import { writable } from 'svelte/store'

export const session = writable(Cookies.get('payve-session'))
export const session_is_valid = writable(false)
export const user_object = writable(undefined)
export const session_fetch_completed = writable(false)
