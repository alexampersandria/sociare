import Cookies from 'js-cookie'

import { session } from './stores'

const log_out = () => {
	Cookies.remove('payve-session')
	session.set(null)
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
			}
		})
}

export { log_out, log_in }
