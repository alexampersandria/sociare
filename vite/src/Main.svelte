<script lang="ts">
	import Router from 'svelte-spa-router'

	import LandingPage from './routes/LandingPage.svelte'
	import App from './routes/App.svelte'

	const routes = {
		'/app': App,
		'/': LandingPage,
	}
</script>

<Router {routes} />
