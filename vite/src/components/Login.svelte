<script lang="ts">
	// vvvv #TODO: i18n this vvvv

	import { createForm } from 'felte'
	import { log_in, sign_up } from '../lib/auth'
	import { goto } from '@roxi/routify'
	import { validate } from 'email-validator'

	const username_regex = new RegExp('^[a-zA-Z0-9]+$')

	const { form: log_in_form, errors: log_in_errors } = createForm({
		onSubmit: async (value) => {
			log_in(value.username, value.password).then(() => {
				$goto('/app')
			})
		},
		validate: (value) => {
			const errors = {
				username: '',
				password: '',
			}
			if (!username_regex.test(value.username)) {
				errors.username =
					'Username must only contain letters, numbers, and underscore (_)'
			}
			if (value.username.length < 3) {
				errors.username = 'Username must be at least 3 characters long'
			} else if (value.username.length > 24) {
				errors.username = 'Username must be at most 24 characters long'
			}
			if (value.password.length < 7) {
				errors.password = 'Password must be at least 7 characters long'
			} else if (value.password.length > 96) {
				errors.password = 'Password must be at most 96 characters long'
			}
			console.log(errors)
			return errors
		},
	})

	const { form: sign_up_form, errors: sign_up_errors } = createForm({
		onSubmit: async (value) => {
			sign_up(value.username, value.password, value.email, value.name)
		},
		validate: (value) => {
			const errors = {
				username: '',
				password: '',
				email: '',
				name: '',
			}
			if (!username_regex.test(value.username)) {
				errors.username =
					'Username must only contain letters, numbers, and underscore (_)'
			}
			if (value.username.length < 3) {
				errors.username = 'Username must be at least 3 characters long'
			} else if (value.username.length > 24) {
				errors.username = 'Username must be at most 24 characters long'
			}
			if (value.password.length < 7) {
				errors.password = 'Password must be at least 7 characters long'
			} else if (value.password.length > 96) {
				errors.password = 'Password must be at most 96 characters long'
			}
			if (!validate(value.email)) {
				errors.email = 'Must be a valid email address'
			}
			if (value.name.length < 1) {
				errors.name = 'Display Name is required'
			} else if (value.name.length > 96) {
				errors.name = 'Display Name must be at most 96 characters long'
			}
			return errors
		},
	})

	let action: 'login' | 'signup' = 'login'
</script>

<div class="login wide">
	{#if action === 'login'}
		<h3 class="align-center">Log in</h3>
		<form use:log_in_form>
			<label for="username">Username</label>
			<input type="text" name="username" />
			{#if $log_in_errors.username}
				<div class="error">{$log_in_errors.username}</div>
			{/if}
			<label for="password">Password</label>
			<input type="password" name="password" />
			{#if $log_in_errors.password}
				<div class="error">{$log_in_errors.password}</div>
			{/if}
			<input type="submit" value="Sign in" class="button pink" />
		</form>
		<p class="muted align-center">
			Don't have an account?{' '}
			<a href="#" on:click={() => (action = 'signup')}>Sign up</a>
		</p>
	{:else}
		<h3 class="align-center">Sign up</h3>
		<form use:sign_up_form>
			<label for="username">Username</label>
			<input type="text" name="username" />
			{#if $sign_up_errors.username}
				<div class="error">{$sign_up_errors.username}</div>
			{/if}
			<label for="password">Password</label>
			<input type="password" name="password" />
			{#if $sign_up_errors.password}
				<div class="error">{$sign_up_errors.password}</div>
			{/if}
			<label for="email">E-Mail</label>
			<input type="text" name="email" />
			{#if $sign_up_errors.email}
				<div class="error">{$sign_up_errors.email}</div>
			{/if}
			<label for="name">Display Name</label>
			<input type="text" name="name" />
			{#if $sign_up_errors.name}
				<div class="error">{$sign_up_errors.name}</div>
			{/if}
			<input type="submit" value="Sign Up" class="button pink" />
		</form>
		<p class="muted align-center">
			Already have an account?{' '}
			<a href="#" on:click={() => (action = 'login')}>Log in</a>
		</p>
	{/if}
</div>
