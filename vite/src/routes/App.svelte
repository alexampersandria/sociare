<script>
	import { link } from 'svelte-spa-router'

	import Button from '../components/Button.svelte'
	import { user_object, session_is_valid } from '../lib/stores'
	import { createForm } from 'felte'

	import { log_in, log_out } from '../lib/auth'

	const { form } = createForm({
		onSubmit: async (value) => {
			log_in(value.username, value.password)
		},
	})
</script>

<div class="container">
	<a href="/" use:link>⬅️ landing page</a>

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
