<script>
	import Button from '../components/Button.svelte'
	import { user_object, session, session_is_valid } from '../lib/stores'
	import { createForm } from 'felte'

	import { log_in, log_out, sign_up } from '../lib/auth'

	const { form: log_in_form } = createForm({
		onSubmit: async (value) => {
			log_in(value.username, value.password)
		},
	})

	const { form: sign_up_form } = createForm({
		onSubmit: async (value) => {
			sign_up(value.username, value.password, value.email, value.name)
		},
	})
</script>

<div class="container">
	<a href="/">⬅️ landing page</a>

	{#if $session_is_valid && $user_object && $session}
		<div class="user">
			<h2>user</h2>
			<div class="id">
				<strong>id: </strong>
				{$user_object.id}
			</div>
			<div class="username">
				<strong>username: </strong>
				{$user_object.username}
			</div>
			<div class="name">
				<strong>name: </strong>
				{$user_object.name}
			</div>
		</div>
		<div class="session">
			<h2>Authorization</h2>
			<div class="id">
				<strong>Bearer </strong>
				{$session}
			</div>
		</div>
		<div class="actions" style="margin-top: 2rem;">
			<Button variant="yellow" on:click={log_out}>Log Out</Button>
		</div>
	{:else}
		<h2>Not logged in!</h2>

		<h3>Log in</h3>
		<form use:log_in_form>
			<label for="username">username</label>
			<input type="text" name="username" /><br />
			<label for="password">password</label>
			<input type="password" name="password" /><br />
			<input type="submit" value="Sign in" />
		</form>

		<h3>Sign up</h3>
		<form use:sign_up_form>
			<label for="username">username</label>
			<input type="text" name="username" /><br />
			<label for="password">password</label>
			<input type="password" name="password" /><br />
			<label for="email">email</label>
			<input type="text" name="email" /><br />
			<label for="name">name</label>
			<input type="text" name="name" /><br />
			<input type="submit" value="Sign Up" />
		</form>
	{/if}
</div>
