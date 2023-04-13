<script lang="ts">
	import { createForm } from 'felte'
	import { log_in, sign_up } from '../lib/auth'
	import { goto } from '@roxi/routify'

	const { form: log_in_form } = createForm({
		onSubmit: async (value) => {
			log_in(value.username, value.password).then(() => {
				$goto('/app')
			})
		},
	})

	const { form: sign_up_form } = createForm({
		onSubmit: async (value) => {
			sign_up(value.username, value.password, value.email, value.name)
		},
	})
</script>

<div class="login">
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
</div>
