<script lang="ts">
	import '@/assets/app.scss'

	const mounted = Date.now()
	const show_preloader = writable(true)

	import { Router } from '@roxi/routify'
	import { routes } from '../.routify/routes'

	import {
		session,
		session_fetch_completed,
		session_is_valid,
		user_object,
	} from './lib/stores/session'

	const validate_session = async (session) => {
		session_fetch_completed.set(false)
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
		session_fetch_completed.set(true)
	}
	session.subscribe(async (session) => {
		validate_session(session)
	})

	session_fetch_completed.subscribe(async (session_fetch_completed) => {
		if (session_fetch_completed) {
			const min_wait_time = 300
			const time_diff = Date.now() - mounted
			if (time_diff < min_wait_time) {
				setTimeout(() => {
					show_preloader.set(false)
				}, min_wait_time - time_diff)
			} else {
				show_preloader.set(false)
			}
		}
	})

	import Preloader from './components/Preloader.svelte'
	import { writable } from 'svelte/store'

	import { register, init } from 'svelte-i18n'

	register('en', () => import('./lang/en.json'))

	init({
		fallbackLocale: 'en',
	})

	init({
		// fallback to en if current locale is not in the dictionary
		fallbackLocale: 'en',
		initialLocale: 'en',
	})

	import { page } from '@roxi/routify'
</script>

<Router {routes} />

<Preloader show={$show_preloader} />
