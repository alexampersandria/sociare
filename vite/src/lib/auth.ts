import Cookies from 'js-cookie'
import { session, user_object } from './stores/session'

const log_out = () => {
	return new Promise((resolve, reject) => {
		Cookies.remove('payve-session')
		session.set(null)
		user_object.set(undefined)
		resolve(null)
	})
}

const log_in = (username, password) => {
	return new Promise((resolve, reject) => {
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
					resolve(res.id)
				} else {
					Cookies.remove('payve-session')
					reject(res.error)
				}
			})
	})
}

export const sign_up = (username, password, email, name) => {
	return new Promise((resolve, reject) => {
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
					resolve(res)
				} else if (res.error) {
					reject(res.error)
				}
			})
	})
}

export { log_out, log_in }
