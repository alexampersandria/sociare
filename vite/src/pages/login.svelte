<script lang="ts">
	import Login from '../components/Login.svelte'
	import payve_logo from '../assets/payve_logo.svg'
	import { session_fetch_completed, session_is_valid } from '../lib/stores'
	import { goto } from '@roxi/routify'

	session_is_valid.subscribe((is_valid) => {
		if (is_valid && session_fetch_completed) {
			$goto('/app')
		}
	})
</script>

<div class="full-page-login">
	<div class="go-back">
		<a href="/">
			<img src={payve_logo} alt="Payve Logo" />
		</a>
	</div>
	<div class="login-wrapper">
		<Login />
	</div>
</div>

<style>
	.full-page-login {
		position: relative;
		height: 100vh;
	}

	.go-back {
		position: absolute;
		top: 2em;
		width: 100%;
		text-align: center;
	}

	.go-back img {
		width: 100px;
	}

	.login-wrapper {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		min-width: min(50vw, 24rem);
	}
</style>
