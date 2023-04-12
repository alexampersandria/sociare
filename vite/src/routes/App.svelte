<script>
	import Button from '../components/Button.svelte'
	import { user_object, session_is_valid, session } from '../lib/stores'
	import { createForm } from 'felte'

	import Cookies from 'js-cookie'

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

	const { form } = createForm({
		onSubmit: async (value) => {
			log_in(value.username, value.password)
		},
	})
</script>

<div class="container">
	<a href="/">⬅️ landing page</a>

	{#if $session_is_valid}
		<div class="user">
			<h1>Hello <i>{$user_object.name}</i>!</h1>
			<div class="username">
				<strong>username: </strong>
				{$user_object.username}
			</div>
			<div class="name">
				<strong>name: </strong>
				{$user_object.name}
			</div>
			<Button on:click={log_out}>Log Out</Button>
		</div>
	{:else}
		<h2>Not logged in!</h2>

		<form use:form>
			<label for="username">username</label>
			<input type="text" name="username" /><br />
			<label for="password">password</label>
			<input type="password" name="password" /><br />
			<input type="submit" value="Sign in" />
		</form>
	{/if}
</div>
