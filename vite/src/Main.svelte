<script lang="ts">
	import Router from 'svelte-spa-router'

	import LandingPage from './routes/LandingPage.svelte'
	import App from './routes/App.svelte'
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

	const routes = {
		'/app': App,
		'/': LandingPage,
	}

	import Preloader from './components/Preloader.svelte'
</script>

<Router {routes} />

<Preloader show={!$session_fetch_completed} />
