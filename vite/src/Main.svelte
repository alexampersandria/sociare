<script lang="ts">
	const mounted = Date.now()
	const show_preloader = writable(true)

	import Router from 'svelte-spa-router'

	import LandingPage from './routes/LandingPage.svelte'
	import App from './routes/App.svelte'
	import NotFound from './routes/NotFound.svelte'

	import {
		session,
		session_fetch_completed,
		session_is_valid,
		user_object,
	} from './lib/stores'

	const validate_session = async (session) => {
		console.log('validating session', session)
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
			const min_wait_time = 200
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

	const routes = {
		'/app': App,
		'/': LandingPage,
		'*': NotFound,
	}

	import Preloader from './components/Preloader.svelte'
	import { writable } from 'svelte/store'
</script>

<Router {routes} />

<Preloader show={$show_preloader} />
