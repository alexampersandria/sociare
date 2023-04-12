import Cookies from 'js-cookie'

import { session, user_object } from './stores'

const log_out = () => {
	Cookies.remove('payve-session')
	session.set(null)
	user_object.set(undefined)
}

const log_in = (username, password) => {
	fetch(`${import.meta.env.VITE_API_URL}/api/v1/user/login`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
		},
		body: JSON.stringify({
			username,
			password,
		}),
	})
		.then((res) => res.json())
		.then((res) => {
			if (!res.error && res.id) {
				Cookies.set('payve-session', res.id)
				session.set(res.id)
			} else {
				alert(res.error)
			}
		})
}

export const sign_up = (username, password, email, name) => {
	fetch(`${import.meta.env.VITE_API_URL}/api/v1/user/create`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
		},
		body: JSON.stringify({
			username,
			password,
			email,
			name,
		}),
	})
		.then((res) => res.json())
		.then((res) => {
			if (!res.error) {
				alert('Account created!')
			} else if (res.error) {
				alert(res.error)
			}
		})
}

export { log_out, log_in }
